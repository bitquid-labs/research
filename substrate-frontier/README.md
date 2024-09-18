# Substrate and Frontier Research

## Table of Contents

- Introduction
- Substrate
- Frontier
- Relationship
- Resources
- Glossary
- Questions to revisit

## Introduction

Understanding how to implement precompiled contracts using Frontier on a substrate chain led to making a more in depth study to Substrate, frontier and the components that exists within them.

## Substrate

Substrate is an SDK for building applicatiom-specific bloockchains from modular and extensible components.These app-specific chains can function alone or in parallel with other chains to take advantage of the `shared security provided by the Polkadot ecosystem`.

By using Substrate and FRAME, you can build `proof-of-concept` application-specific blockchains without the complexity of building a blockchain from scratch or the limitations of building on a general-purpose blockchain. With this one can focus on the application logic which most times takes the form of plug-in modules `pallets`.

## Components within Substrate

### FRAME (Framework for Runtime Aggregation of Modularized Entities)

`FRAME` provides the modular and extensible components called `pallets` for developing application specific logic to configure or suit the requirements of app specific chains.

Pallets are predefined modules that provide functionalities to implement a specific feature for app chains. e.g `pallet_balances` for managing account assets and transfers between accounts.

FRAME is an environment that enables you to select and configure pallets that you want to include in your runtime. In addition to these pallets, FRAME relies on the following required modules to construct and enable the client outer node services to interact with the runtime:

- FRAME system crate frame_system provides low-level types, storage, and functions for the runtime.
- FRAME support crate frame_support is a collection of Rust macros, types, traits, and modules that simplify the development of Substrate pallets.
- FRAME executive pallet frame_executive orchestrates the execution of incoming function calls to the respective pallets in the runtime.

Creating your own pallet requires designing the app logic, storage requirements, error handling and so on.

#### <h3><u>Steps in adding a pallet:</u></h3>

- Open the `runtime/Cargo.toml`

- Locate the `[dependencies]` section and add the pallet as a dependency:

```bash
pallet-utility = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", branch = "polkadot-vX.Y.Z" }
```

- Locate the `[features]` section and add the pallet to the list for the standard binary:

```bash
[features]
default = ["std"]
std = [
  ...
  "pallet-utility/std",
  ...
]
```

- Save your changes on your `Cargo.toml` and close it. Open the `runtime/src/lib.rs` file in your code editor.

- Add the implementation for the `Config` for that specific pallet:

```rust
impl pallet_utility::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type RuntimeCall = RuntimeCall;
  type PalletsOrigin = OriginCaller;
  type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
}
```

Every pallet has its own `Config` trait for the specific parameters and type it requires. Always check the [docs](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/index.html) for the a pallet implementation to check out its configuration requirements.

- Add the pallet inside the `construct_runtime!` macro

```rust
construct_runtime!(
 pub struct Runtime
 where
    Block = Block,
    NodeBlock = opaque::Block,
    UncheckedExtrinsic = UncheckedExtrinsic
 {
        System: frame_system,
        RandomnessCollectiveFlip: pallet_randomness_collective_flip,
        Timestamp: pallet_timestamp,
        Aura: pallet_aura,
        ...
        Utility: pallet_utility, // Add this line
        ...
 }
 )
```

### Substrate Node

Nodes just like in every decentralized network communicate with each other about txs and blocks. A substrate node consists of 2 main parts with separate responsilbilities:

<h3><u>A Core Client</u></h3>

Which contains `outer node services` that handles network activities such as peer discovery, managing tx requests, reaching consensus with peers and responding to RPC calls. These outer node services are responsible for activities that takes placeoutside of the runtime:

- Storage: The outer node persists the evolving state of the blockchain using key-value storage layer.
- Peer-to-peer networking: For communication between other nodes.
- Consensus: Communicates with other nodes to agree on the state of the blockchain.
- RPC API: Accepts requests to allow users interact with the blockchain
- Telementary: Collects and provides accessto node metrics.
- Execution environment

<h3><u> A Runtime </u></h3>
That contains all of the business logic for executing the state transition function of the blockchain. The runtime determies whether txs are valid or invalid and is responsible for handling changes to the blockchain state. Results coming in comes through the client into the runtime and the runtime is responsible for  the state transition  functions and storing the resulting state. Because the runtime executes the functions it receieves, it controls how txs are included in blocks and how the blocks are returned to the outer node for gossiping.

The custom runtime (State transition logic) is a self-contained WASM object that is stored as a part of the chain state, you can easily iterate on the aplication design and evolve your application.

The runtime design enables:

- Support for forkless upgrades.
- Multi-platform compatibility.
- Runtime validity checking.
- Validation proofs for relay chain consensus mechanisms.

Similar to how the outer node has a way to provide information to the runtime, the runtime uses specialized `host functions` to communicate with the outer node or the outside world.

This separation of responsibility/ activity between the client and runtime makes the Substrate node upgradeable. The application logic(which is what makes your chain unique) is stored onchain in WASM binary. To make changes to your logic you can compile new WASM binary and submit a tx to update the WASM binary with the new one.

Tasks/ activities performed by the outer node frequently require queries to and fro the runtime. The `runtime API` facilitates this communications between the outer node and the runtime. The `sp_api` crate provides the interface to implement the runtime API. `Every runtime must implement Core and Metadata`.

In working with substrate nodes, the `chain_spec.rs` file describes the configuration of the default Development and Local Testnet chains, including information about default pre-funded development accounts and the nodes that are preconfigured with the authority to produce blocks. If you create a custom chain, you use this file to identify the network that a node connects to and the other nodes that the local node communicates with.

### Substrate Libraries

The Substrate libraries are divided into three main areas of responsibility:

- Core client libraries for outer node services: enables the node to handle its network responsiblities, including consensus and block execution are Rust crates that use the `sc_` prefix.
- FRAME libraries for the runtime: The library that enable you build runtime logic. Use the `frame_` prefix. In addition to the `frame*_` libraries, the runtime can contain one or more `pallet_` libraries which represents a single frame module.
- Primitive libraries for underlying functions and interfaces for communication between the libraries (outer nodes and the runtime): They give you control to the underlying operations and enable communication between the core client services and the runtime. They are rust crate that use the `sp_` prefix.

The Polkadot relay chain doesn't support smart contracts. However, the parachains that connect to Polkadot can support arbitrary state transitions, so any parachain can be a potential platform for smart contract deployment. Substrate provides tools to support two types of smart contracts:

- The `contracts` pallet in the FRAME library enables a Substrate-based chain to execute smart contracts compiled to WASM regardless of the language used to write the smart contract.
- The `evm` pallet in the Frontier project enables a Substrate chain to run EVM contracts written in Solidity.

Core Primitives are data types that must be defined and must fulfill a particular interface to work within the substrate framework.

## Transactions and Blocks

Tx provides a mechanism for making changes to a state that can be included in a block. There are 3 tx types in Substrate:

- Signed Tx
  This tx must include the signature of an account making the request to execute some runtime call. Usually signed with a private key for the account submitting the request and in most cases also pays a tx fee.
- Unsigned Tx
  This type of tx dont require a signatre and doesnt include any information about who submitted the tx.
- Inherent Tx
  Sometimes referred to as inherents are a special type of unsigned tx that allow block authoring nodes to add information directly to a block. Inherents can only be inserted into a block by the block authoring node that calls them.

All 3 tx types are more braodly referred to as extrinsics. `Extrinsics` is a term used to refer to information that originates from outside the runtime.

In Substrate, a **`Block`** consists of a `header` and an array of tx. The header contains the block height, parent hash, tx root, state root and the digest.

### Transaction lifecycle.

Using rules defined in the runtime the tx pool checks the validity of each tx. The checks ensure that only valid transactions that meet specific conditions are queued to be included in a block. After the initial validity check, the transaction pool periodically checks whether existing transactions in the pool are still valid. If a transaction is found to be invalid or has expired, it is dropped from the pool. There are two queues for valid txs:

- The ready queue contains txs that can be included in the new pending block.
- The future ready contains txs that might becomde valid in future.

#### <u>Transaction ordered by priority</u>

When a node authors a block, the tx are ordered based on its priority calculated in the runtime using its weight and fees. This tx priority is provided to the outer node as tags and applies to all tx types excpet from inherents which are always placed as first priority using the `EnsureInherentsAreFirst` trait.

#### <u>Executing tx and producing blocks</u>

After valid transactions are placed in the transaction queue, a separate executive module orchestrates how transactions are executed to produce a block. The executive module calls functions in the runtime modules and executes those functions in specific order.

- Initializing a block
  To initialize a block the executive module first call `on_initialize`(which defines the logic that should be executed before tx are executed) in the system pallet and then in all other runtime pallets. The function must first be called on the system pallet, the other pallets are called in the order they are defined in the `constrct_runtime!` macro.

  After all of the `on_initialize` functions have been executed, the executive module checks the parent hash in the block header and the trie root to verify the info is correct.

- Executing Tx
  After block initialization, each valid tx is executed in order of tx priority. State changes and events are written directly to storage during execution, so any chage made even to a failed tx is not reverted, that is why the runtime logic should make sure and extrinsic will succeed before committing any changes to storage or emitting an event.

- Finalizing a block
  After execution, the executive module calls `on_idle` and `on_finalize` on each pallet to perform any logic that should take place at the end of the block, based on the order in the `construct_runtime!` macro but the system pallet is executed last. The executive module then checks that the digest and storage root in the block header match what was calculated when the block was initialized.

## Glossary

- Consensus model or algorithm is the method that a blockchain uses to batch txs into a block and to selct which node can submit a block to the chain

## Questions to revisit

- What are the different consensus that can be used on app chains?
- How is interaction with other chains or interoperable possible or handles? Read about Cross-Consensus messaging (XCM)
- How are chains upgraded and what are forkeless upgrades?
- Read more about state transition and storage
