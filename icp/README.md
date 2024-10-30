# ICP Research

# Components on ICP

## Canisters

Canisters are ICP's version of smart contracts. Canisters are implemented as WebAssembly modules. This allows for maximum interoperability, as developers can write canisters in a variety of languages that target WebAssembly.

## Chain Key Technology

### Chain Key Cryptography

Chain Key Cryptography is a Treshold Signature Scheme implementation that allows subnets with ICP to jointly hold cryptographic keys to prevents any single node from having full control over the private key, enhancing security. Even if some nodes are compromised, they cannot individually create a valid signature unless the threshold of honest nodes is reached.

### Chain Key Signatures

Chain Key Signatures allows for direct interoperability with other blockchains by allowing canisters to generate threshold ECDSA signatures which is the most commonly used signature scheme used on other blockchains (including Bitcoin and Ethereum). A rough overview of how this may work in a cross chain protocol is:

- **Assigning a subnet to our Canister**

When we deploy our canister it gets assigned to a subnet.

- **Generating the Key**

Use a distributed key generator to generate keys which would be managed by the subnet(a group of nodes) responsible for handling your canister’s requests. We can use [ICPs/ DFINITY's DKG](https://eprint.iacr.org/2021/339).

- **Signing Cross Chain Txs**

When your canister needs to interact with a smart contract on another blockchain (like Ethereum), it sends a request for signature to the nodes in the subnet which then uses threshold ECDSA to generate a valid signature, which can be used to sign a transaction or message intended for the other blockchain. This signature can be verified on the other chain just like any other standard cryptographic signature.

## Chain Fusion

Chain Fusion uses the chain key technology to allow ICP to connect with multiple blockchains without the use of bridges.

# Implementation of existing Canisters on ICP

## Modules

- Cover Canister
- Pool Canister
- Governance Canister
- Token Canister

## Architectural Diagram

![ICP Canisters Architectural Diagram](https://brown-high-badger-463.mypinata.cloud/ipfs/QmYB9nPsfP1PEtz71L117zXwpAughVSAi48BZcHWiKuVGW)

The BQ Insurance Protocol would leverage ICP as the main infrastructure for managing governance, token, and liquidity pool operations, while the protocol's cover contract is deployed across multiple Bitcoin Layer 2 (L2) networks to provide users with the ability to purchase insurance covers on different networks.

### ICP Canisters

- Governance Canister: Handles the administrative control and management of governance proposals within the protocol.
- Token Canister: Manages the protocol's tokens.
- Pool Canister: Responsible for liquidity management and insurance claim payouts.

These three canisters interact with each other within the ICP ecosystem and will leverage ICP’s ic_cdk (Canister Development Kit) for smooth inter-canister communication and state sharing.

### Cross Chain Communication

The Cover contract/ canister would be deployed across Bitcoin L2s to allow users purhase covers on different networks. The communcation between these contracts and the BQ protocol on ICP would be achieved using ICP's Chain Key Technology.
