use alloy::contract::{ContractInstance, Interface};
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::{network::EthereumWallet, providers::ProviderBuilder, signers::local::PrivateKeySigner};
use dotenv::dotenv;
use tokio::time::sleep;
use std::time::Duration;
use std::env;
use std::str::FromStr;
use serde_json::Value;
use hex;

use crate::types::{Covers, Pool};

pub const OWNER: &str = "0xDA01D79Ca36b493C7906F3C032D2365Fb3470aEC";
pub const COVER: &str = "0x9054b93ca6c02B4Fc6EF19D7cf0F45c002BcAE3e";
pub const GOVERNANCE: &str = "0x6a927024d37F950e1B15787Fc8da4Be1FA854346";
pub const POOL: &str = "0xC3614e27e1e30182CE0DC15F7E6E21562CF5F212";
pub const ALFAJORES: &str = "https://alfajores-forno.celo-testnet.org";
pub const _PWR_URL: &str = "https://bitcoinplus.pwrlabs.io/";
pub const _BEVM: &str = "https://testnet.bevm.io/";
pub const _BITLAYER: &str = "https://testnet-rpc.bitlayer.org/";
pub const _MERLIN: &str = "https://testnet-rpc.merlinchain.io";
pub const _BOB: &str = "https://bob-sepolia.rpc.gobob.xyz/";
pub const _ROOTSTOCK: &str = "https://public-node.testnet.rsk.co/";

#[warn(dead_code)]
pub async fn create_cover(cover: Covers) {
    sleep(Duration::from_secs(2)).await;
    dotenv().ok();
    
    let _owner = Address::from_str(OWNER).expect("Error from owner address");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in .env file");
    let signer = PrivateKeySigner::from_str(&private_key).expect("Error instantiating private key");
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(ALFAJORES.parse().expect("Error parson url"));
    let cover_contract_address = Address::from_str(COVER).expect("Error from cover address");
    let path = std::env::current_dir().expect("msg").join("artifacts/InsuranceCover.json");
    let artifacts = std::fs::read(path).expect("Failed to read artifact");
    let json: Value = serde_json::from_slice(&artifacts).expect("");
    let abi_value = json.get("abi").expect("Failed to get ABI from artifact");
    let abi = serde_json::from_str(&abi_value.to_string()).expect("msg");
    let interface = Interface::new(abi);

    let cover_contract = ContractInstance::new(cover_contract_address, provider.clone(), interface);
    let cid = cover.cid.into();
    let risk = cover.risk.into();
    let name = cover.name.into();
    let chains = cover.chains.into();
    let capacity = cover.capacity.into();
    let cost = cover.cost.into();
    let pool_id = cover.pool_id.into();
    let owner = Address::from_str(OWNER).expect("Error getting address");

    let _nonce = provider.get_transaction_count(owner).await.expect("Error getting nonce");
    let gas_price = provider.get_gas_price().await.expect("Error getting gas price");

    let function_call = cover_contract
        .function(
            "createCover", 
            &[cid, risk, name, chains, capacity, cost, pool_id]
        )
        .expect("Error creating cover function")
        .gas_price(gas_price);

    let result = function_call
        .send()
        .await
        .expect("Error sending cover creation function")
        .with_required_confirmations(3); 
    let hash = result.tx_hash();

    let _receipt = provider.get_transaction_receipt(*hash).await.expect("Error getting tx receipt");
    
    let hash_string = format!("0x{}", hex::encode(hash));
    println!("Tx Hash: {:?}", hash_string)

}

pub async fn create_pool(pool: Pool) {
    sleep(Duration::from_secs(2)).await;
    dotenv().ok();

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in .env file");
    let signer = PrivateKeySigner::from_str(&private_key).expect("Error instantiating private key");
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(ALFAJORES.parse().expect("Error parson url"));
    
    let pool_contract_address = Address::from_str(POOL).expect("Error from pool address");
    let path = std::env::current_dir().expect("msg").join("artifacts/InsurancePool.json");
    let artifacts = std::fs::read(path).expect("Failed to read artifact");
    let json: Value = serde_json::from_slice(&artifacts).expect("");
    let abi_value = json.get("abi").expect("Failed to get ABI from artifact");
    let abi = serde_json::from_str(&abi_value.to_string()).expect("msg");
    let interface = Interface::new(abi);

    println!("Pool Address: {:?}", pool_contract_address);

    let pool_contract = ContractInstance::new(pool_contract_address, provider.clone(), interface);
    let risk = pool.risk.into();
    let name = pool.name.into();
    let apy = pool.apy.into();
    let min_period = pool.min_period.into();
    let owner = Address::from_str(OWNER).expect("Error getting address");

    let _nonce = provider.get_transaction_count(owner).await.expect("Error getting nonce");
    let gas_price = provider.get_gas_price().await.expect("Error getting gas price");

    let function_call = pool_contract
        .function(
            "createPool",
            &[risk, name, apy, min_period]
        )
        .expect("Error creating pool function")
        .gas_price(gas_price);

    let result = function_call
        .send()
        .await
        .expect("Error sending pool creation function"); 
    let hash = result.tx_hash(); 

    let _receipt = provider.get_transaction_receipt(*hash).await.expect("Error getting tx receipt");

    let hash_string = format!("0x{}", hex::encode(hash));
    println!("Tx Hash: {:?}", hash_string)
}

pub async fn pool_active() {
    sleep(Duration::from_secs(2)).await;
    dotenv().ok();

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in .env file");
    let signer = PrivateKeySigner::from_str(&private_key).expect("Error instantiating private key");
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(ALFAJORES.parse().expect("Error parson url"));
    
    let pool_contract_address = Address::from_str(POOL).expect("Error from pool address");
    let path = std::env::current_dir().expect("msg").join("artifacts/InsurancePool.json");
    let artifacts = std::fs::read(path).expect("Failed to read artifact");
    let json: Value = serde_json::from_slice(&artifacts).expect("");
    let abi_value = json.get("abi").expect("Failed to get ABI from artifact");
    let abi = serde_json::from_str(&abi_value.to_string()).expect("msg");
    let interface = Interface::new(abi);

    println!("Pool Address: {:?}", pool_contract_address);

    let pool_contract = ContractInstance::new(pool_contract_address, provider.clone(), interface);
    let pool_id = U256::from(1);

    let result = pool_contract.function("poolActive", &[pool_id.into()]).expect("error").call().await.expect("error2");

    let response = result[0].as_bool().unwrap();
    println!("Pool Active: {:?}", response)
}

// pub async fn get_address() {
//     sleep(Duration::from_secs(2)).await;
//     dotenv().ok();

//     let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in .env file");
//     let signer = PrivateKeySigner::from_str(&private_key).expect("Error instantiating private key");
//     let wallet = EthereumWallet::from(signer);

//     let provider = ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(ALFAJORES.parse().expect("Error parson url"));
    
//     let pool_contract_address = Address::from_str(POOL).expect("Error from pool address");
//     let path = std::env::current_dir().expect("msg").join("artifacts/InsurancePool.json");
//     let artifacts = std::fs::read(path).expect("Failed to read artifact");
//     let json: Value = serde_json::from_slice(&artifacts).expect("");
//     let abi_value = json.get("abi").expect("Failed to get ABI from artifact");
//     let abi = serde_json::from_str(&abi_value.to_string()).expect("msg");
//     let interface = Interface::new(abi);

//     println!("Pool Address: {:?}", pool_contract_address);

//     let pool_contract = ContractInstance::new(pool_contract_address, provider.clone(), interface);
//     let gov = Address::from_str(GOVERNANCE).expect("error from gov address");
// }

pub async fn pool_set() {
    sleep(Duration::from_secs(2)).await;
    dotenv().ok();

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in .env file");
    let signer = PrivateKeySigner::from_str(&private_key).expect("Error instantiating private key");
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(ALFAJORES.parse().expect("Error parson url"));
    
    let pool_contract_address = Address::from_str(POOL).expect("Error from pool address");
    let path = std::env::current_dir().expect("msg").join("artifacts/InsurancePool.json");
    let artifacts = std::fs::read(path).expect("Failed to read artifact");
    let json: Value = serde_json::from_slice(&artifacts).expect("");
    let abi_value = json.get("abi").expect("Failed to get ABI from artifact");
    let abi = serde_json::from_str(&abi_value.to_string()).expect("msg");
    let interface = Interface::new(abi);

    let governance = Address::from_str(GOVERNANCE).expect("Error from gov address");
    let cover = Address::from_str(COVER).expect("Error from cover address");
    let pool_contract = ContractInstance::new(pool_contract_address, provider.clone(), interface);
    let function_call = pool_contract.function("setGovernance", &[governance.into()]).expect("Error");
    let gov_result = function_call.send().await.expect("Error from setting governance").with_required_confirmations(3);
    let hash = gov_result.tx_hash(); 

    let _receipt = provider.get_transaction_receipt(*hash).await.expect("Error getting tx receipt");

    let hash_string = format!("0x{}", hex::encode(hash));
    println!("Pool set gov Tx Hash: {:?}", hash_string);

    let f_call = pool_contract.function("setCover", &[cover.into()]).expect("Error");
    let cover_result = f_call.send().await.expect("Error from setting cover").with_required_confirmations(3);
    let hash = cover_result.tx_hash(); 

    let _receipt = provider.get_transaction_receipt(*hash).await.expect("Error getting tx receipt");

    let hash_string = format!("0x{}", hex::encode(hash));
    println!("Pool set cover Tx Hash: {:?}", hash_string)
}

pub async fn cover_set() {
    dotenv().ok();
    
    let _owner = Address::from_str(OWNER).expect("Error from owner address");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in .env file");
    let signer = PrivateKeySigner::from_str(&private_key).expect("Error instantiating private key");
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(ALFAJORES.parse().expect("Error parson url"));
    let cover_contract_address = Address::from_str(COVER).expect("Error from cover address");
    let path = std::env::current_dir().expect("msg").join("artifacts/InsuranceCover.json");
    let artifacts = std::fs::read(path).expect("Failed to read artifact");
    let json: Value = serde_json::from_slice(&artifacts).expect("");
    let abi_value = json.get("abi").expect("Failed to get ABI from artifact");
    let abi = serde_json::from_str(&abi_value.to_string()).expect("msg");
    let interface = Interface::new(abi);

    let governance = Address::from_str(GOVERNANCE).expect("Error from gov address");
    let cover_contract = ContractInstance::new(cover_contract_address, provider.clone(), interface);
    let function_call = cover_contract.function("setGovernance", &[governance.into()]).expect("error from setting governance");
    let result = function_call.send().await.expect("Error setting governance").with_required_confirmations(3);
    let hash = result.tx_hash(); 

    let _receipt = provider.get_transaction_receipt(*hash).await.expect("Error getting tx receipt");

    let hash_string = format!("0x{}", hex::encode(hash));
    println!("Cover set Tx Hash: {:?}", hash_string)
}