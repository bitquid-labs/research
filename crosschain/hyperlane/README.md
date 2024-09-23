# Hyperlane Implementation

Anyone can permissionlessly deploy hyperlane to any blockchain environment, whether it is a layer 1, rollup, or app-chain, allowing that chain to communicate seamlessly with any other chain on which Hyperlane has been deployed, with customizable security model.
[Supported Chains](https://github.com/hyperlane-xyz/hyperlane-registry/tree/main/chains)
[Read about Warp Routes for Token Transfers](https://docs.hyperlane.xyz/docs/protocol/warp-routes/warp-routes-overview)

## Components

- Mailbox
  The mailbox are smart contracts provided by hyperlane which facilitates the sending and recieving of arbitrary messages. There is a Mailbox contract deployed on every chain Hyperlane supports. The `IMailbox` interface exposes two state-mutating functions; `dispatch()` and `process()`, which are used to send and receive messages, respectively.
- Interchain Security Module (ISM)
  Hyperlane is secured by ISMs, which are SCs that are responsible that messages sent from the origin chain are delivered to the destination chain. Hyperlane provides a pre-built ISMs which we can deploy or override if we want to customize our own ISM. Some of them are:
  - Multisig ISM: verify that m of n Validators have attested to the validity of a particular interchain message.
  - Routing ISM: allows devs to change ISMs based on the context.
  - Aggregation ISM: combine security from multiple ISMs.
  - CCIP-Read ISM
- Relayer
  Relayers run the transport layer by delivering messages from origin chains to destination chains.Before attempting to deliver a message the Relayer gathers metadata relating to the ISM used to prove the security of the message.
- Sender
- Recipient
