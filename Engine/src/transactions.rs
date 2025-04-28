use alloy::{
    network::{EthereumWallet},
    json_abi::JsonAbi,
    primitives::{Address,U256,address},
    providers::{Provider,ProviderBuilder},
    signers::{local::PrivateKeySigner},
    transports::{http::Http,Transport},
    contract::{ContractInstance,Interface},
    dyn_abi::{DynSolValue,DynSolType},
};

use std::time::Duration;
use std::env;
use dotenv::dotenv;
use url::Url;
const ABI_JSON:&str  = r#"
[
    {
        "inputs": [
            { "internalType": "address[]", "name": "ad_vendors", "type": "address[]" },
            { "internalType": "uint256[]", "name": "credits", "type": "uint256[]" }
        ],
        "name": "update_credits",
        "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            { "internalType": "address[]", "name": "users", "type": "address[]" },
            { "internalType": "uint256[]", "name": "rewardPoints", "type": "uint256[]" }
        ],
        "name": "UpdateUserRewards",
        "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
        "stateMutability": "nonpayable",
        "type": "function"
    }
]
"#;
use super::*;

pub async fn update_transaction(data : Arc<Mutex<Appstate>>)  {

    loop {
        tokio::time::sleep(Duration::from_secs(600)).await;
    dotenv().ok();
    let provider_url = env::var("RPC_URL").expect("Missing INFURA_URL");
    let private_key = env::var("PRIVATE_KEY").expect("Missing PRIVATE_KEY");
    let contract_address: Address = env::var("CONTRACT_ADDRESS")
        .expect("Missing CONTRACT_ADDRESS")
        .parse()
        .expect("Invalid contract address");
    let url = Url::parse(&provider_url).expect("Invalid RPC URL");
    let signer:PrivateKeySigner =  private_key.parse().expect("Invalid private key");

    let wallet:EthereumWallet = EthereumWallet::from(signer);
    // Setup provider and wallet
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .on_http(url);
    let abi = serde_json::from_str(ABI_JSON).expect("Invalid ABI JSON");
    let interface = Interface::new(abi);

    // Create contract instance using ABI
    let contract = ContractInstance::new(contract_address, provider.clone(),interface);

    let mut _state = data.lock().await;
    let mut vendors: Vec<DynSolValue> = Vec::new();
    let mut credits: Vec<DynSolValue> = Vec::new();

    for vendor in _state.ADVendors.iter() {
        vendors.push(DynSolValue::Address(vendor.parse().expect("Invalid vendor address")));
        credits.push(DynSolValue::Uint(U256::from(*_state.ADVendors_Credit.get(vendor).unwrap_or(&0)),256));
    }
    // Build transaction to call update_credits
    let tx1 = contract.function("update_credits", &[DynSolValue::Array(vendors),DynSolValue::Array(credits)]).expect("errro while parsing args").send().await.expect("error while sending transaction")
    .watch().await.expect("error while watching transaction");


    let mut users: Vec<DynSolValue> = Vec::new();
    let mut reward_points: Vec<DynSolValue> = Vec::new();
    for user in _state.users.iter() {
        users.push(DynSolValue::Address(user.parse().expect("Invalid user address")));
        reward_points.push(DynSolValue::Uint(U256::from(*_state.RewardPoints.get(user).unwrap_or(&0)),256));
    }
    

    // Build transaction to call UpdateUserRewards
    let tx2 = contract.function("UpdateUserRewards", &[DynSolValue::Array(users),DynSolValue::Array(reward_points)]).expect("error while sending the tran").send()
    .await.expect("error while sending transaction")
    .watch().await.expect("error while watching transaction");

    // Check transaction results
     println!("Transaction 1: {:?}", tx1);
     println!("Transaction 2: {:?}", tx2);
    }
}
