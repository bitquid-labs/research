# Cross Chain Communication for BQ Chain

- Overview and Problem Statement
- Architecture
  - Diagram of the current and proposed Architecture
- Constraints
- Possible Solutions
- Reference

## Overview

BQLabs is an insurance protocol for the bitcoin ecosystem. It would be powered by the BQ Chain for L2s within the bitcoin ecosystem to address possible risks that can occur within the bitcoin ecosystem. While there are more components with the protocol, currently there are 3 smart contracts/ components for our devnet.

- The Governance: For managing proposals and votes which are measured with `BQ` tokens.
- The Insurance Cover: For managing covers and paying yields for liquidity providers
- The Insurance Pool: For managing pools for liquidity providers and paying claims for approved proposals.

The goal of this research is to come up with a working mechanism or architecture for seamless interaction between contracts on the Bitcoin L2s and the BQ Chain.

## Architecture

- Current Architecture
  All contracts are located/ deployed on the target network(Bitcoin L2). While this approach is straight-forward, its less efiicient because we would have to deploy and manage multiple instances of the goverance contract and BQ tokens, which would be difficult to keep track of.

  ![Current Architecture](https://brown-high-badger-463.mypinata.cloud/ipfs/QmdX2HiPwkVRYAHK2Bj6t6c3UUW5CLmgUTScJWGshyHTLJ)

- Proposed approach
  The insurance pool and cover contract is located on the target network, while the governace and token contract is deployed on BQ Chain. This approach/ architecture would involve making cross contract calls across the target chain and the BQ Chain.
  ![Proposed Architecture](https://brown-high-badger-463.mypinata.cloud/ipfs/QmPcfWWvRcTeDGHLKxkFamb23P8fkkm2N9CqPVzwq8u5SP)

## Constraints

- Network Management:
  How would these contracts addresses on this several target networks be managed on the governance contract. Think about adding or making any changes without having to deploy a new contract
- Security Considerations
- Future improvement, Upgradability and Extensibility.

## Research Focus

This research aims to explore and evaluate different methods for implementing efficient and secure cross-chain communication between the BQ Chain and various Bitcoin L2 networks. We will investigate existing solutions, assess their compatibility with our specific use case, and determine the most suitable approach for our insurance protocol.

- XCMP

- Axelar
  Explore the possible use of Axelar's General Message Passing or EVM relayer.
- Layer Zero
