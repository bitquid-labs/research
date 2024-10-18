use async_trait::async_trait;
use frost_ristretto255::{self as frost, Identifier};
use std::collections::{BTreeMap, HashMap};
use bincode;
use frost::keys::dkg::round2;
use libp2p::SwarmBuilder;
use libp2p::{
    Multiaddr,
    swarm::{NetworkBehaviour, SwarmEvent, dial_opts::DialOpts},
    identity,
    PeerId,
    swarm::Swarm,
    mdns,
    noise,
    tcp,
    yamux,
    futures::StreamExt
};
use libp2p_request_response as request_response;
use request_response::ProtocolSupport;
use std::time::Duration;
use std::error::Error as StdError;
use serde::{Serialize, Deserialize};
use futures::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Round2Request {
    SendPackage(round2::Package),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Round2Response {
    Acknowledgment(String),
}

// Define the behaviour struct
#[derive(NetworkBehaviour)]
pub struct Round2Behaviour {
    pub requestresponse: request_response::Behaviour<Round2Codec>,
    pub mdns: mdns::tokio::Behaviour,
}

impl Round2Behaviour {
    fn new(key: identity::Keypair) -> Self {
        let _codec = Round2Codec;
        let protocols = vec![("round2-protocol".to_string(), ProtocolSupport::Full)];
        let config = request_response::Config::default();
        let requestresponse = request_response::Behaviour::new(protocols, config);
        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id()).expect("error from mdns");
        Round2Behaviour { requestresponse, mdns }
    }
}

#[derive(Clone, Default)]
pub struct Round2Codec;

#[async_trait]
impl request_response::Codec for Round2Codec {
    type Protocol = String;
    type Request = Round2Request;
    type Response = Round2Response;

    async fn read_request<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
    ) -> std::io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;
        bincode::deserialize(&buf).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    async fn read_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
    ) -> std::io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;
        bincode::deserialize(&buf).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    async fn write_request<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> std::io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let encoded = bincode::serialize(&req)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        io.write_all(&encoded).await
    }

    async fn write_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        res: Self::Response,
    ) -> std::io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let encoded = bincode::serialize(&res)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        io.write_all(&encoded).await
    }

}

pub async fn setup_r2(_local_peer_id: PeerId, local_key: identity::Keypair) -> Result<(Swarm<Round2Behaviour>, HashMap<PeerId, Multiaddr>), Box<dyn StdError>> {
    let mut peer_to_multiaddrs = HashMap::new();

    let mut swarm = SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(tcp::Config::new(), noise::Config::new, yamux::Config::default)?
        // .with_quic()
        .with_behaviour(|key| Round2Behaviour::new(key.clone()))?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    let listening_address: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse()?;
    swarm.listen_on(listening_address.clone())?;

    while peer_to_multiaddrs.is_empty() && swarm.connected_peers().count() < 1 {
        let event = swarm.select_next_some().await;
        for peer in swarm.connected_peers() {
            println!("Peers : {:?}", peer)
        }
        match event {
            // SwarmEvent::NewListenAddr { address, .. } => {
            //     println!("Listening on {:?}", address);
            //     if address.to_string().contains("/tcp/") {
            //         PeerId::
            //         swarm.listen_on(address).expect("Error adding listen address");
            //     }
            // }
            SwarmEvent::Behaviour(Round2BehaviourEvent::Mdns(mdns::Event::Discovered(peers))) => {
                    for (peer_id, multiaddrs) in peers {
                        println!("Discovered peer: {:?} with addr: {:?}", peer_id, multiaddrs);
                        swarm.add_peer_address(peer_id, multiaddrs.clone());
                        if multiaddrs.to_string().contains("/tcp/") {
                            peer_to_multiaddrs.insert(peer_id, multiaddrs.clone());
                        }
                        println!("Added new address for {:?}", peer_id);
                    }
            }
            SwarmEvent::ConnectionEstablished { peer_id, connection_id, endpoint, num_established, .. } => {
                println!("Connection established with {:?} with ID {:?} on {:?} with {:?} connections", peer_id, connection_id, endpoint, num_established )
            }
            _ => {}
        }
    }

    Ok((swarm, peer_to_multiaddrs))
}

pub async fn send_r2(peer_to_multiaddrs: HashMap<PeerId, Multiaddr>, identifier_to_peers: HashMap<Identifier, PeerId>,  swarm: &mut Swarm<Round2Behaviour>, packages: BTreeMap<Identifier, round2::Package>) -> Result<(), Box<dyn StdError + Send + Sync>> {
    println!("{:?}", identifier_to_peers);
    tokio::time::sleep(Duration::from_secs(5)).await;

    for (identifier, package) in packages.iter() {
        if let Some(peer_id) = identifier_to_peers.get(identifier) {
            println!("Attempting to send package to peer: {:?}", peer_id);
            if let Some(multiaddrs) = peer_to_multiaddrs.get(peer_id) {
                println!("Multi address: {:?}", multiaddrs);
                
                if !swarm.is_connected(peer_id) {
                    let dial_opts = DialOpts::peer_id(peer_id.clone())
                        .addresses(vec![multiaddrs.clone()])
                        .build();
                    if let Err(e) = swarm.dial(dial_opts) {
                        eprintln!("Failed to dial peer {}: {:?}", peer_id, e);
                        continue;
                    }
                }

                let r2_package = Round2Request::SendPackage(package.clone());
                let req_id = swarm.behaviour_mut().requestresponse.send_request(peer_id, r2_package.clone());
                println!("Message sent {:?} to {:?}, request ID: {:?}", r2_package, peer_id, req_id);
            
            } else {
                println!("No peer ID found for identifier: {:?}", identifier);
            }
        } else {
            println!("No peer ID found for identifier: {:?}", identifier);
        }
    }

    Ok(()) 
}

pub async fn receive_r2(nodes: usize, 
    swarm: &mut Swarm<Round2Behaviour>,
    packages: BTreeMap<Identifier, round2::Package>
) -> Result<BTreeMap<Identifier, round2::Package>, Box<dyn StdError + Send + Sync>> {
    println!("Waiting to recieve round 2 packages...");
    let mut received_r2_packages = BTreeMap::new();
    println!("Current length {:?}", received_r2_packages.len());
    while received_r2_packages.len() < nodes {
        let event = {
            swarm.select_next_some().await
        };
        println!("Waiting to receive..");
        match event {
                SwarmEvent::Behaviour(Round2BehaviourEvent::Requestresponse(request_response::Event::Message { peer, message })) => {
                   println!("Does it ever enter here?");
                    match message {
                        request_response::Message::Request { request, channel, .. } => {
                            let Round2Request::SendPackage(package) = request;
                            println!("Received package from {}: {:?}", peer, package);
                            let peer_id_bytes = peer.to_bytes();
                            let identifier = frost::Identifier::derive(&peer_id_bytes)?;
                            received_r2_packages.insert(identifier, package);
                                
                            let response = Round2Response::Acknowledgment("Package received".to_string());
                            swarm.behaviour_mut().requestresponse.send_response(channel, response).unwrap();
                            }

                        request_response::Message::Response { response, .. } => {
                            let Round2Response::Acknowledgment(msg) = response;
                            println!("Received acknowledgment from {}: {}", peer, msg);
                        }
                    }
                }
                SwarmEvent::Behaviour(Round2BehaviourEvent::Mdns(mdns::Event::Discovered(peers))) => {
                    for (peer_id, multiaddrs) in peers {
                        println!("Discovered peer: {:?}", peer_id);
                        let dial = DialOpts::peer_id(peer_id).addresses(vec![multiaddrs]).build();
                        let peer_id_bytes = peer_id.to_bytes();
                        if !swarm.is_connected(&peer_id) {
                            if let Err(err) = swarm.dial(dial) {
                                eprintln!("Error dialing peer {} with count {:?}: {:?}", peer_id, swarm.connected_peers().count(), err);
                            } else {
                                println!("Dialed peer: {:?}", peer_id);
                            }
                        } else {
                            println!("Peer {:?} is already connected.", peer_id);
                        }
                        let identifier = frost::Identifier::derive(&peer_id_bytes)?;
                        if let Some(package) = packages.get(&identifier) {
                            let r2_package = Round2Request::SendPackage(package.clone());
                            let req_id = swarm.behaviour_mut().requestresponse.send_request(&peer_id, r2_package.clone());
                            println!("Message sent {:?} to {:?}, request ID: {:?}", r2_package, peer_id, req_id);
                        }
                    }
                }
                SwarmEvent::ConnectionEstablished { peer_id, connection_id, endpoint, num_established, .. } => {
                    println!("Connection established with {:?} with ID {:?} on {:?} with {:?} connections", peer_id, connection_id, endpoint, num_established );
                    if !swarm.is_connected(&peer_id) {
                        println!("Not a dialed peer: {:?}", peer_id);
                    } else {
                        println!("Peer {:?} is already connected.", peer_id);
                    }
                    let peer_id_bytes = peer_id.to_bytes();
                    let identifier = frost::Identifier::derive(&peer_id_bytes)?;
                    if let Some(package) = packages.get(&identifier) {
                        let r2_package = Round2Request::SendPackage(package.clone());
                        let req_id = swarm.behaviour_mut().requestresponse.send_request(&peer_id, r2_package.clone());
                        println!("Message sent {:?} to {:?}, request ID: {:?}", r2_package, peer_id, req_id);
                    }
                }
                SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                    println!("Connection closed with peer: {:?}, cause: {:?}", peer_id, cause);
                }
                _ => {}
        }

    }

    Ok(received_r2_packages)
}