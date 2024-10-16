mod round1_network;
mod round2_network;
mod key_generation;

use key_generation::{State, perform_round_one, perform_round_two, finalize_key_generation};
use round1_network::{init, setup_r1, MyBehaviour, MyBehaviourEvent};
use round2_network::setup_r2;
use std::sync::{Arc, Mutex};
use tokio;
use libp2p::{Swarm, swarm::SwarmEvent, mdns, gossipsub::IdentTopic};
use futures::StreamExt;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let state = Arc::new(Mutex::new(State {
        nodes: vec![String::from("node1"), String::from("node2"), String::from("node3")],
        max_signers: 2,
        min_signers: 2,
        identifier_to_peer_id: Mutex::new(HashMap::new()),
    }));
    let (peer_id, local_key, _identifier) = init(state.clone()).await?;

    println!("Setting up environment.......");
    let (mut swarm1, topic) = setup_r1(peer_id, local_key.clone()).await?;
    let nodess: usize = 1;

    println!("Swarm and Topics gotten, waiting for peers....");

    wait_for_peers(&topic, &mut swarm1, 1).await?;

    let (sp1, _round1_package, received_round1_packages, identifier_to_peers) = perform_round_one(nodess, local_key.clone(), &topic, swarm1, peer_id, state.clone()).await?;
    println!("Completed Round 1");

    wait_for_round_completion(1, received_round1_packages.len(), nodess).await?;

    let (swarm2, peer_to_multiaddrs) = setup_r2( peer_id, local_key.clone()).await?;
    let (sp2, round2_package, received_round2_packages) = perform_round_two(peer_to_multiaddrs, state.clone(), nodess, swarm2, sp1, &received_round1_packages, identifier_to_peers).await?;
    println!("Completed Round 2");
    println!("Sent round 2 package: {:?}", round2_package);
    println!("Received {} round 2 packages", received_round2_packages.len());

    wait_for_round_completion(2, received_round2_packages.len(), nodess).await?;

    let (key_package, pubkey_package) = finalize_key_generation(sp2, &received_round1_packages, &received_round2_packages).await?;
    
    println!("Node Key Package: {:?}", key_package);
    println!("Node Public Key Package: {:?}", pubkey_package);

    Ok(())
}

pub async fn wait_for_peers(topic: &IdentTopic,swarm: &mut Swarm<MyBehaviour>, min_peers: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("Waiting to discover peers....");
    let mut discovered_peers = 0;
    let mut subscribed_peers: usize = 0;
    let topic_hash = topic.hash();
    let timeout_duration = tokio::time::Duration::from_secs(30); // Set a timeout duration
    let start_time = tokio::time::Instant::now();

    let status = swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
    println!("Subscription done? {:?}", status);
    
    while subscribed_peers < min_peers {
        if start_time.elapsed() >= timeout_duration {
            println!("Timeout reached while waiting for peers.");
            break;
        }

        match swarm.select_next_some().await {
            SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(peers))) => {
                for (peer_id, _) in &peers {
                    println!("Discovered peer: {:?}", peer_id);
                    let topic_hash = topic.hash();
                    swarm.behaviour_mut().gossipsub.add_explicit_peer(peer_id);
                    let peer_count = swarm.behaviour_mut().gossipsub.mesh_peers(&topic_hash).count();
                    println!("Publishing message to topic: {:?} with {:?} peers", topic, peer_count);
                }
                discovered_peers += peers.len();
                println!("Discovered {} peers, total: {}", peers.len(), discovered_peers);
            }
            SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(libp2p::gossipsub::Event::Subscribed { peer_id, topic })) => {
                if topic == topic_hash {
                    println!("Peer {:?} has subscribed to topic: {:?}", peer_id, topic);
                    subscribed_peers += 1;
                    println!("New peer subscribed. {:?} peers exists", subscribed_peers)
                }
            }
            _ => {}
        }
    }
    Ok(())
}

async fn wait_for_round_completion(round: u8, received_packages: usize, total_nodes: usize) -> Result<(), Box<dyn std::error::Error>> {
    if received_packages < total_nodes - 1 {
        println!("Warning: Round {} completed with fewer packages than expected", round);
        println!("Received {} packages, expected {}", received_packages, total_nodes - 1);
    }
    Ok(())
}