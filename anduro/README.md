# Understanding Anduro and Alys Network Deployment

Anduro is multi-sidechain network built on Bitcoin with two permissionless chains Cordinate and Alys. Cordinate is a UTXO chain while Alys (which I will be focusing on) is an EVM compatible chain geared toward issuing and trading tokenized real-world-assets. Anduro has 3 major components:

- The Collective
  A diverese consortium of Bitcoin entities that govern the network.

- The sidechains

- The sidechain native asset

## The Alys Network

Alys is an Ethereum compatible sidechain that can execute smart contracts built for managing RWAs. Use POW as consensus mechanism which is executed by Bitcoin miners and BTC as native currency as well. The Anduro collective also participate in alys consensus by signing periodic blocks to promote MEV-resistance and fast finality but the real engine behind Anduro sidechain block production is Bitcoin mining

Alys Network uses Geth which would make implementation with EVM tools easy.

## Running the Alys Sidechain and Deploying Contracts to the Network

I had hardhat set up and had added the Alys sidechain as a network in the `hardhat.config.js`:

```javascript
alys: {
  url: "localhost:8545",
  accounts: [PRIVATE_KEY],
},
```

In other to run Alys network locally I installed the following tools (I had rust installed already):

- Geth: I ran using the command `geth`
- Bitcoin Core which came with `bitcoin` and `bitcoin-cli`
- Foundry

After cloning the Alys repo and navogated to the root of the folder, I ran the following commands from different terminals:

```bash
geth

bitcoind -regtest -rpcuser=rpcuser -rpcpassword=rpcpassword -fallbackfee=0.002

cargo run --bin app -- --dev
```

I am able to successfully spin up and run the commands for geth and bitcoind, however, when I run: `cargo run --bin app -- --dev` I get this error:

```rust
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.30s
     Running `target/debug/app --dev`
2024-09-17T08:49:10.556999Z  INFO app::store: Using db path .alys/chain_db
2024-09-17T08:49:10.577027Z  INFO app::app: Head: Ok(None)
2024-09-17T08:49:10.577209Z  INFO app::app: Finalized: Ok(None)
2024-09-17T08:49:10.623483Z  INFO app::app: Using bitcoin deposit address bcrt1p3fz0jpyeqvp9tlf2xmh9ppya3halueq0n388h9n9zwf6t2d8ew3sdlkhpr
thread 'main' panicked at app/src/engine.rs:240:73:
called `Result::unwrap()` on an `Err` value: Auth(InvalidToken)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Aborted
```

Not sure what the issue is.

Resources

- [Alys Github](https://github.com/AnduroProject/alys)
- [Alys Litepaper](https://cdn.prod.website-files.com/65d7ad8d6664c459f717e27d/65fc7d9bbb856302865ea302_ANDURO-Litepaper.pdf)
