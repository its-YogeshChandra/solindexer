//create the web server in this only
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;
use std::io;
mod controller;
mod routes;
use controller::fetch_data;

// pub struct AppDataStruct {
//     data: Mutex<String>,
// }

//main data struct
#[derive(Debug, Deserialize)]
pub struct MainDataStruct {
    data: Vec<String>,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    //the main server instance
    println!("server is working");
    HttpServer::new(move || {
        //moving main_data into the closure
        App::new()
            .wrap(actix_cors::Cors::permissive())
            .service(fetch_data)
    })
    .bind(("0.0.0.0", 9000))?
    .run()
    .await
}
