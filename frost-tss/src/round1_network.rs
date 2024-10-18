use crate::key_generation;

use frost_ristretto255::{self as frost, Identifier};
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use bincode;
use frost::keys::dkg::round1;
use libp2p::SwarmBuilder;
use libp2p::{
    mdns,
    swarm::{NetworkBehaviour, SwarmEvent},
    identity,
    PeerId,
    gossipsub,
    gossipsub::{IdentTopic, MessageAuthenticity},
    swarm::Swarm,
    noise,
    tcp,
    yamux,
    futures::StreamExt,
};
use std::collections::hash_map::DefaultHasher;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::error::Error as StdError;
use tokio::io;
use key_generation::State;

#[derive(NetworkBehaviour)]
pub struct MyBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

pub async  fn init(state: Arc<Mutex<State>>) -> Result<(PeerId, identity::Keypair, Identifier), Box<dyn StdError>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);
    let peer_id_bytes = local_peer_id.to_bytes();
    let identifier = frost::Identifier::derive(&peer_id_bytes)?;

    {
        let state_guard = state.lock().unwrap(); 
        let mut map = state_guard.identifier_to_peer_id.lock().unwrap();
        map.insert(identifier, local_peer_id.clone());
    }
    
    Ok((local_peer_id, local_key, identifier))
}

pub async fn setup_r1(_local_peer_id: PeerId, local_key: identity::Keypair) -> Result<(Swarm<MyBehaviour>, IdentTopic), Box<dyn StdError>> {
    println!("Building Swarm...");
    let mut swarm: Swarm<MyBehaviour> = SwarmBuilder::with_existing_identity(local_key)
    .with_tokio()
    .with_tcp(
        tcp::Config::default(),
        noise::Config::new,
        yamux::Config::default,

    )?
    .with_quic()
    .with_behaviour(|key| {
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };

            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10))
                .validation_mode(gossipsub::ValidationMode::Strict)
                .message_id_fn(message_id_fn)
                .build()
                .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?;

            let gossipsub = gossipsub::Behaviour::new(
                MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )?;

            let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
            Ok(MyBehaviour { gossipsub, mdns })
    })?
    .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
    .build();

    // Listen on all interfaces and whatever port the OS assigns
    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
   
    println!("Swarm built, getting topic....");
    let topic = gossipsub::IdentTopic::new("frost-dkg");
    tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;

     Ok((swarm, topic))
}

pub async fn broadcast_r1(swarm: &mut Swarm<MyBehaviour>, topic: &IdentTopic, package: round1::Package) -> Result<(), Box<dyn StdError>> {
    
    let message = bincode::serialize(&package)?;
    // let topic_hash = topic.hash();
    let _ = tokio::time::sleep(tokio::time::Duration::from_secs(10));
    // let peer_count = swarm.behaviour_mut().gossipsub.mesh_peers(&topic_hash).count();
    // println!("Publishing message to topic: {:?} with {:?} peers", topic, peer_count);
    match swarm.behaviour_mut().gossipsub.publish(topic.clone(), message) {
        Ok(_) => println!("Broadcast done...."),
        Err(e) => println!("Error during broadcasting: {:?}", e),
    }
    Ok(())
}

pub async fn receive_r1(_state: Arc<Mutex<State>>, nodes: usize, swarm: &mut Swarm<MyBehaviour>, local_peer_id: PeerId) -> Result<(BTreeMap<Identifier, round1::Package>, HashMap<Identifier, PeerId>), Box<dyn StdError>> {
    let mut received_r1_packages = BTreeMap::new();
    let mut idetifier_to_peers = HashMap::new();

    while received_r1_packages.len() <  nodes {
        let event = swarm.select_next_some().await;
        println!("Waiting to receive..");
        match event {
                SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source,
                    message_id: _,
                    message,
                })) => {
                    if propagation_source != local_peer_id {
                        let package: round1::Package = bincode::deserialize(&message.data)?;
                        let peer_id_bytes = propagation_source.to_bytes();
                        let identifier = frost::Identifier::derive(&peer_id_bytes)?;
                        println!("Recieved package from {:?}", identifier);
                        received_r1_packages.insert(identifier, package);

                        // let state_guard = state.lock().unwrap();
                        // let mut map = state_guard.identifier_to_peer_id.lock().unwrap().clone();
                        if !idetifier_to_peers.contains_key(&identifier) {
                            idetifier_to_peers.insert(identifier, propagation_source);
                            println!("Added identifier {:?} with peer {:?}", identifier, propagation_source);
                        } else {
                            println!("Identifier {:?} already exists in the map", identifier);
                        }
                    }
                }
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(peers))) => {
                    for (peer_id, _) in peers {
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                }
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(peers))) => {
                    for (peer_id, _) in peers {
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                }
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Local node is listening on: {}", address);
                }
                _ => {}
            }
        }

        Ok((received_r1_packages, idetifier_to_peers))
}