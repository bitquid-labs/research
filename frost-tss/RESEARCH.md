# TSS, Treshold Signature Scheme

Allows participants to collectively generate signatures without anyone having full control of the private key.

In our case, each participant is a node.

Frost is a TSS.

### Key Generation
When a key is generated, each participants will get:

* a secret share,

* a verifying share, which is used by others to verify the signature share produced,

* a grouping verifying key,  a public key matching the private key that was split into shares. It is used to verify the final signature generated with frost.

### Method of Key Generation
Opting for the use of Distributed Key Generation rather than the Trusted Dealer Generation because trusted dealer requires the entire key to be in memory at some point which could be a security flaw.

**Cordinator** (which can be one of the shareholders) selects the message to be signed and the participants that will generate the signature. Each node sends fresh nonce commitments to the Coordinator, which then consolidates them and sends them to each participant. Each one will then produce a signature share, which is sent to the Coordinator who finally aggregates them and produces the final signature.

## Steps for Distributed Key Generation
1. Each node performs the `dkg::part1` function passing in their identifiers. The function returns a `round1` secret package and a `round1` package.
2. The secret package is kept in memory while the package is sent to other nodes.
3. Each node performs the `dkg::part2` function which takes the node's `round1` secret package and the `round1` packages received from the other participants.
4. Similarly to the `round1`, the secret is kept in memory and the `round2` package is sent to other nodes.
5. Each node performs the `dkg::part3` function which is the final part using the node's `round2` secret key, the `round1` packages recieved and the `round2` packages recieved from other participants.
6. It returns the KeyPackage that has the long-lived key share for the participant, and the PublicKeyPackages that has public information about all participants; both of which are required to compute FROST signatures.

### Implementation of Broadcast and Peer to Peer communication channels
> **_OBSERVATION:_**  The communication pattern between the nodes is not directly P2P, its more of a one-to-many communication among nodes. With this we will be be implementing 

In a P2P network, this broadcast might be implemented using techniques like gossip protocols or flooding.

### Possible improvements and reviews
* Use of numbers as node identifiers
* Use of a single BTree map for mapping Identifiers to `round1` and updating it to `round2` pacakges when needed. As opposed to creating 2 BTree maps and storing both within the Nodes struct.

### Doubts/ Problems
* Who determines the max and minimum number of signers?
* We implement for number of nodes, and the key is shared among all nodes, in a case where a new node is added to the network would the key be reshared where each node have a new share or how would this work? Possible resharing or refreshing.
* Implementation of peer to peer channels for communication between nodes ie an authenticated and confidential communication channel. Possible libp2p

## TODOs
* Implementation of secure communication between nodes
    - Possible solution: 
        * Use `libp2p` with the `GossipSub` protocol to handle broadcasting, and use `Noise` to ensure that the communication is secure. 
        * I have to configure `GossipSub` to avoid self-messages.
* Some sort of consensus mechanism to agree on the final public key or to detect malicious behavior.
* Fix the mutable `State` within the functions.
* Improved error handling and Concurrency.