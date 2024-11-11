

# Fractical Bitcoin Network

## Chain Information


* **chainId:** 0x2024
* **Message Magic Number:** d99e94b9

### Mempool Browser
* **Mainnet:** (https://mempool.fractalbitcoin.io)
* **Testnet:** https://mempool-testnet.fractalbitcoin.io

### Explorer
* **Mainnet:**
    https://explorer.unisat.io/fractal-mainnet
    https://www.okx.com/web3/explorer/fractal-bitcoin
* **Testnet:** 
    https://explorer.unisat.io/fractal-testnet
    


### API Support
* ***Open API:*** https://open-api-fractal.unisat.io/swagger.html
* ***API-KEY application:*** https://developer.unisat.io/


### Wallet Support
* ***Chrome Web Store:*** https://chromewebstore.google.com/detail/unisat-wallet/ppbibelpcjmhbdihakflkdcoccbgbkpo?pli=1


## brc-20 on Fractal
To avoid conflicts with existing assets on Bitcoin mainnet, the rules for brc-20 on Fractal have been specifically tailored.
1. Ticker names on Fractal Mainnet will be limited to 6 - 12 bytes. Tickers with 4 or 5 characters will not be permitted, as they are already in use on the Bitcoin mainnet.
2. For brc-20 on Fractal, ticker names can include letters (both uppercase and lowercase: a-z/A-Z), numbers (0-9), and underscores (_). In total, you have 63 different characters to work with.
Ticker names are not case-sensitive. For example, "Aaaaaaaaaaaaaa" is treated the same as "aaaaaaaaaaaaaa."
3. Both fair-launch and self-issuance are supported for all brc-20 assets on Fractal.
"self-issuance" means these assets can only be minted by the holder of deploy inscription. Since deployment inscription can be transferred, whoever holds the deployment inscription has the right to mint the ticker.
In addition to the three rules mentioned above, brc-20 rules on Fractal are consistent with the brc-20 rules on Bitcoin mainnet.
https://layer1.gitbook.io/layer1-foundation/protocols/brc-20/indexing 



## Runes on Fractal
Runes on Fractal follows the same set of rules as the Bitcoin mainnet. If you're familiar with the runes protocol on Bitcoin, you'll find that the structure and functionalities on Fractal are fully aligned with it. Here's what you need to know:
1. Same Rules as Bitcoin Mainnet
The Runes protocol on Fractal follows the same fundamental rules as on Bitcoin, with all phases completed within one halving cycle (it takes 4 years on Bitcoin, and takes 2 years on Fractal)
There are 12 phases on both networks. Therefore, any single phase takes 120 days on Bitcoin, and 60 days on Fractal.
2. Ticker Names on Fractal
On the Fractal mainnet, all Runes tickers are displayed in lowercase letters. 
For more detailed information on the Runes protocol, you can refer to the official documentation here: Runes Protocol Documentation.