# Nubit Research

# What is Nubit?

Nubit is a scalable, cost-efficient, data availability layer secured by Bitcoin for the Bitcoin community. Nubit enables the scaling of Bitcoin's data capacities, empowering applications like Ordinals, Layer 2s, and price oracles, thus broadening the scope and efficiency of the Bitcoin ecosystem.

## Data Availability Layer

In the context of blockchain, data availability refers to the assurance by network participants that the data uploaded by users is faithfully proposed and stored in certain nodes of the network, and that these nodes can prove this property to other participants. 

To achieve consensus on data availability, the most straightforward approach, such as methods in early blockchain systems like Bitcoin, requires network participants to download all block data and verify its validity, leading to significant scalability issues: if the blockchain stores large enough data, ordinary users may not be able to afford the cost of processing this data or may not wish to spend a lot of bandwidth on data they are not interested in. As a result, they may be unable or unwilling to participate in verification, leading to centralization issues in the network. 

To mitigate this challenge, LazyLedger proposed a new technique that uses random sampling to verify data availability. Technically, Data Availability Sampling (DAS) allows network participants to ensure the full availability of block data without requiring any participant to download it completely. This scheme allows potentially malicious block proposers to encode the content of the block into a commitment 𝜎 and a complete encoded block 𝜋. The commitment 𝜎 is added to the block header and it allows light nodes, which are run by ordinary network participants, to verify the availability of the complete 𝜋 by requesting a few positions in 𝜋 randomly. If a sufficient number of light nodes successfully probe 𝜋, DAS ensures that the data is fully available. 

## Introduction
### Consensus Layer backed by BTC-Native Staking

- Developed using the Cosmos SDK, Nubit DA will support BTC staking through solutions like Babylon. This allows for BTC staking on the Nubit DA, ensuring its economic security.

- Since Nubit DA is built on Cosmos SDK, for detailed RPC methods to interact with Nubit Validators, please visit the Validator APIs.


### DAS Mechanism with Full and Light Nodes
To participate in Nubit DA by running a data availability node, individuals can operate either a full node or a light node. Nubit utilizes a data availability mechanism that combines KZG commitments with two-dimensional Reed-Solomon erasure codes. This framework ensures the correctness of RS encoding through validity proofs based on KZG commitments, rather than using traditional fraud proofs. Consequently, light nodes can retrieve necessary data through KZG opening proofs instead of Merkle proofs, significantly enhancing retrieval speeds ---- by up to 10X in typical scenarios and up to 100X in more extreme situations.

## Data Availability Layer
This chapter will guide you through the prerequisites and how to run various components in Nubit DA. There are four types of components, each detailed below:

- **Validator:** Participates in consensus and produces blocks, typically operated by advanced users or node vendors.

- **Bridge Node:** Fetches blobs and block headers from the validator and broadcasts them to the entire node p2p network. It also functions as a full node within the network.

- **Full Node:** Broadcasts blobs and headers to the node p2p network and handles DA sampling requests from light nodes.

- **Light Node:** Easily run by regular users, the light node secures the node p2p network.

Besides:

All kinds of nodes could serve Nubit DA Node APIs and submit blobs to the consensus network.
For a more detailed understanding of those four components, please refer to the introduction.

## Interact with Nubit DA
When running a light or full node, you can interact with Nubit DA in various ways. 

Make sure to set up the environment in a new terminal first. Next, you can manage your keys and wallet. Please ensure you securely save your Nubit address, as it is crucial for running nodes on Nubit DA. You can also obtain NUB from our faucet and explore more node operations, including checking address balances, initiating transfers, submitting Blobs (supporting text and images), and querying transactions.

To better query transactions and network information, you can access our browser, where you can also find various types of information such as addresses, blocks, validators, and namespaces.

This setup enables you to effectively interact with the Nubit DA Alpha Testnet, execute transactions, and manage node operations efficiently. For more integrations, please refer to Nubit Node APIs. For any issues or further assistance, contact our technical support team.