use crate::MainDataStruct;
use actix_web::{HttpResponse, Responder, get, post, web};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{account_info, pubkey::Pubkey};
use std::str::FromStr;
use serde::Serialize;

#[derive(Serialize)]
pub struct AccountResponse {
    pub address: String,
    pub lamports: u64,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
}

//fetch data
#[post("/data")]
pub async fn fetch_data(data: web::Json<MainDataStruct>) -> impl Responder {
    let address_array = data.data.clone();

    //create the client
    let client = RpcClient::new_with_commitment(
        String::from("https://api.mainnet.solana.com"),
        CommitmentConfig::confirmed(),
    );

    let mut results = Vec::new();

    //for loop
    for addr in address_array {
        //read the acount data from these addresses
        if let Ok(pubkey) = Pubkey::from_str(&addr) {
             if let Ok(account_info) = client.get_account(&pubkey).await {
                results.push(AccountResponse {
                    address: addr.clone(),
                    lamports: account_info.lamports,
                    owner: account_info.owner.to_string(),
                    executable: account_info.executable,
                    rent_epoch: account_info.rent_epoch,
                });
             }
        }
    }

    HttpResponse::Ok().json(results)
}
