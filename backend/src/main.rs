//create the web server in this only
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::Deserialize;
use std::io;
mod controller;
mod routes;
use controller::fetch_data;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// pub struct AppDataStruct {
//     data: Mutex<String>,
// }

//main data struct
#[derive(Debug, Deserialize)]
pub struct MainDataStruct {
    data: String,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    //create instance of the main data struct
    // let main_data = web::Data::new(AppDataStruct {
    //     data: Mutex::new("data".to_string()),
    // });

    //the main server instance
    HttpServer::new(move || {
        //moving main_data into the closure
        App::new().service(fetch_data)
    })
    .bind(("0.0.0.0", 9000))?
    .run()
    .await
}
