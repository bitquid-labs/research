use crate::round2_network;
use crate::round1_network;

use libp2p::Multiaddr;
use round1_network::{broadcast_r1, receive_r1, MyBehaviour};
use round2_network::{send_r2, receive_r2, Round2Behaviour};
use rand::rngs::OsRng;
use frost_ristretto255 as frost;
use frost::keys::{KeyPackage, PublicKeyPackage};
use frost::keys::dkg::{round1, round2};
use frost::{Identifier, Error};
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use libp2p::{
    gossipsub::IdentTopic,
    swarm::Swarm,
    PeerId,
    identity,
};

// A struct for the values that would be determined by the protocol and not from the nodes or code.
#[warn(dead_code)]
pub struct State {
    pub nodes: Vec<String>,
    pub max_signers: u16,
    pub min_signers: u16,
    pub identifier_to_peer_id: Mutex<HashMap<Identifier, PeerId>>,
}

pub async fn perform_round_one(nodes: usize, _local_key: identity::Keypair, topic: &IdentTopic, mut swarm: Swarm<MyBehaviour>, peer_id: PeerId, state: Arc<Mutex<State>>) -> Result<(round1::SecretPackage, round1::Package, BTreeMap<Identifier, round1::Package>, HashMap<Identifier, PeerId>), Box<dyn std::error::Error>> {
    let rng = OsRng;
    let state_guard = state.lock().unwrap();
    let max_signers = state_guard.max_signers;
    let min_signers = state_guard.min_signers;
    drop(state_guard); 


    let peer_id_bytes = peer_id.to_bytes();
    let identifier = frost::Identifier::derive(&peer_id_bytes)?;
    let (first_secret_package, round1_package) = frost::keys::dkg::part1(identifier, max_signers, min_signers, rng)?;
    
    broadcast_r1(&mut swarm, &topic, round1_package.clone()).await?;

    let (receive_round_one_packages, identifier_to_peers) = receive_r1(state, nodes, &mut swarm, peer_id).await?;

    let mut round1_packages = BTreeMap::new();
    for (key, value) in receive_round_one_packages.clone() {
       let round1_pkg = value;
        round1_packages.insert(key, round1_pkg);
    }

    Ok((first_secret_package, round1_package, receive_round_one_packages, identifier_to_peers))
}

pub async fn perform_round_two(
    peer_to_multiaddrs: HashMap<PeerId, Multiaddr>,
    _state: Arc<Mutex<State>>,
    nodes: usize,
    mut swarm: Swarm<Round2Behaviour>,
    sp1: round1::SecretPackage,
    received_round1_packages: &BTreeMap<Identifier, round1::Package>,
    identifier_to_peers: HashMap<Identifier, PeerId>,
) -> Result<(round2::SecretPackage, BTreeMap<Identifier, round2::Package>, BTreeMap<Identifier, round2::Package>), Box<dyn std::error::Error>> {
    println!("Performing part 2....");
    let (round2_secret_package, round2_package) = frost::keys::dkg::part2(sp1, &received_round1_packages)?;

    println!("Part 2 DKG Done...");

    let arc_round2_package = Arc::new(round2_package.clone());

    // Spawn the send_r2 function
    // let send_task = {
    //     let arc_swarm = Arc::clone(&arc_swarm);
    //     let arc_round2_package = Arc::clone(&arc_round2_package);
    //     tokio::spawn(async move {
    //         send_r2(peer_to_multiaddrs.clone(), identifier_to_peers.clone(), arc_swarm, (*arc_round2_package).clone()).await
    //     })
    // };
    send_r2(peer_to_multiaddrs.clone(), identifier_to_peers.clone(), &mut swarm, (*arc_round2_package).clone()).await.expect("msg");

    // Spawn the receive_r2 function
    // let receive_task = {
    //     let arc_swarm = Arc::clone(&arc_swarm);
    //     tokio::spawn(async move {
    //         receive_r2(nodes, arc_swarm).await
    //     })
    // };

    let result = receive_r2(nodes, &mut swarm, (*arc_round2_package).clone()).await.expect("Error receiving results");

    // Wait for both tasks to complete
    //  let (send_result, received_packages) = tokio::join!(send_task, receive_task);
    // send_result?.expect("Error sending result");
    // let received_packages = result.expect("Error receiving packages");

    // let mut round2_packages = BTreeMap::new();
    // for (key, value) in result {
    //     round2_packages.insert(key, value);
    // }

    Ok((round2_secret_package, round2_package, result))
}

// Parameter values gotten from perform_round_two function
pub async fn finalize_key_generation(sp2: round2::SecretPackage, received_round1_packages: &BTreeMap<Identifier, round1::Package>, received_round2_packages: &BTreeMap<Identifier, round2::Package>) -> Result<(KeyPackage, PublicKeyPackage), Error> {

    let (key_package, pubkey_package) = frost::keys::dkg::part3(
        &sp2,
        &received_round1_packages,
        &received_round2_packages,
    )?;

    Ok((key_package, pubkey_package))
}

// With its own key package and the pubkey package, each participant can now proceed to sign with FROST.