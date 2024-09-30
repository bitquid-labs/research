use alloy::contract::{ContractInstance, Interface};
use alloy::primitives::{Address, U256};
use alloy::{network::EthereumWallet, providers::ProviderBuilder, signers::local::PrivateKeySigner};
use std::str::FromStr;
use serde_json::Value;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let gov = "0x328d42318A8f15B0fb05437d52B6200A0255257D";
    let rpc = "https://rpc-amoy.polygon.technology/";
    let owner = "0xDA01D79Ca36b493C7906F3C032D2365Fb3470aEC";

    let _owner = Address::from_str(owner).expect("Error from owner address");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in .env file");
    let signer = PrivateKeySigner::from_str(&private_key).expect("Error instantiating private key");
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(rpc.parse().expect("Error parson url"));
    let gov_contract_address = Address::from_str(gov).expect("Error from cover address");
    let path = std::env::current_dir().expect("msg").join("artifacts/Gov.json");
    let artifacts = std::fs::read(path).expect("Failed to read artifact");
    let json: Value = serde_json::from_slice(&artifacts).expect("");
    let abi_value = json.get("abi").expect("Failed to get ABI from artifact");
    let abi = serde_json::from_str(&abi_value.to_string()).expect("msg");
    let interface = Interface::new(abi);

    let poolid = U256::from(1); 

    let gov_contract = ContractInstance::new(gov_contract_address, provider.clone(), interface);
    let result = gov_contract.function("poolActive", &[poolid.into()]).expect("msg").call().await.expect("msg");

    println!("Result: {:?}", result);
    let response = result[0].as_tuple().unwrap();
    println!("Tuple length: {}", response.len());

    let status = response[0].as_bool().unwrap();
    println!("Pool {:?} status: {:?}", poolid, status);
}
