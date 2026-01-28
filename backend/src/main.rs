//create the web server in this only
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use std::{io, sync::Mutex};
mod controller;
mod routes;
use controller::fetch_data;

#[post("/")]
fetch_data


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

//main data struct
pub struct MainDataStruct {
    data: Mutex<String>,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("web server in the rust ");

    //create instance of the main data struct
    let main_data = web::Data::new(MainDataStruct {
        data: Mutex::new("data".to_string()),
    });

    //the main server instance
    HttpServer::new(move || {
        //moving main_data into the closure
        App::new()
            .app_data(main_data.clone())
            .service(hello)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    //have to put the meessage that server is running on port
}
