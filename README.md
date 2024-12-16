# Research

This repository contains notes, research findings, and proof-of-concept implementations related to protocols and tasks for BQLabs.

## [Anduro](https://github.com/bitquid-labs/research/tree/main/anduro)

High Level overview/ research of integrating or current smart contracts on Anduro and the Alys network. Majorly invloved running the network locally to deploy existing contracts on it using hardhat.

## [Crosschain Implementation](https://github.com/bitquid-labs/research/tree/main/crosschain)

Exploring the use of technologies like Hyperlane and Layerzero for crosschain communication across our base chain and EVM chains where our contracts would be deployed.

### Proposed Architecture:

- **Insurance Pool and Cover Contracts**: Deployed on the target EVM network.

- **Governance and Token Contracts**: Hosted on BQ Chain.

This approach/ architecture would involve making cross contract calls across the target chain and the BQ Chain.

## [Frost Treshold Signature Scheme](https://github.com/bitquid-labs/research/tree/main/frost-tss)

**Overview**

This section focuses on the FROST (Flexible Round-Optimized Schnorr Threshold) Signature Scheme, a modern and efficient protocol for Distributed Key Generation (DKG) and signing. The implementation uses libp2p for peer-to-peer (P2P) communication between participating nodes.

Leveraged libp2p's request-response and gossipsub protocols to manage both targeted and broadcasted messages during key exchange.

## [ICP](https://github.com/bitquid-labs/research/tree/main/icp)

High level overview of the use of ICP as base chain over the use of custom substrate app chain. Explored the impact/ use of ICP technologies like Chain Key Technology.

Current implementation [here](https://github.com/bitquid-labs/icp)

## [Substrate Frontier](https://github.com/bitquid-labs/research/tree/main/substrate-frontier)

An in-depth study of Substrate and Frontier to understand how precompiled contracts can be implemented on a Substrate-based chain. This includes research into Frontier's components and their integration within Substrate.
