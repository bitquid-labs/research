

# Citrea Network

Citrea is the first rollup that enhances the capabilities of Bitcoin blockspace with zero-knowledge technology, making it possible to build everything on Bitcoin.

## Chain Information

* **Chain ID:** 5115
* **Endpoint:** https://rpc.testnet.citrea.xyz
* **Explorer:** https://explorer.testnet.citrea.xyz
* **Chain Name:** Citrea Testnet
* **Currency Symbol:** cBTC

## Faucet

To get started, you can request testnet funds from our faucet: https://citrea.xyz/faucet

## Deployment Guide

Citrea is fully EVM-compatible, making it easy to deploy your EVM smart contracts and applications. Simply update your RPC endpoint to get started.

### Configuring Hardhat

To configure Hardhat for Citrea, update your `hardhat.config.js` file with the following configuration:
```javascript
module.exports = {
  // ... REST OF YOUR CONFIG GOES HERE ...
  networks: {
    citrea: {
      url: "https://rpc.testnet.citrea.xyz",
      chainId: 5115,
      accounts: ["YOUR_PRIVATE_KEY"],
    },
  },
  // ... REST OF YOUR CONFIG GOES HERE ...
};
```
Replace `YOUR_PRIVATE_KEY` with your actual private key.
