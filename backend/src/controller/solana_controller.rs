use crate::MainDataStruct;
use actix_web::{HttpResponse, Responder, get, post, web};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{account_info, pubkey::Pubkey};
use std::str::FromStr;

//fetch data
#[post("/data")]
pub async fn fetch_data(data: web::Json<MainDataStruct>) -> impl Responder {
    let address_array = data.data.clone();

    //create the client
    let client = RpcClient::new_with_commitment(
        String::from("https://api.mainnet.solana.com"),
        CommitmentConfig::confirmed(),
    );

    //for loop
    for addr in address_array {
        //read the acount data from these addresses
        println!("the address is : {}", &addr);
        let pubkey = Pubkey::from_str(&addr).unwrap();
        let account_info = client.get_account(&pubkey).await.unwrap();
        println!("addr is {:?}", account_info);
    }

    HttpResponse::Ok().body("value is acheived")
}
