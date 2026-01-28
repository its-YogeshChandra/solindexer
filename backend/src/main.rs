//create the web server in this only
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use std::{io, sync::Mutex};
mod controller;
mod routes;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

//main data struct
struct MainDataStruct {
    data: Mutex<String>,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("web server in the rust ");

    //create instance of the main data struct
    let main_data = web::Data::new(MainDataStruct {
        data: Mutex::new("data".to_string()),
    });

    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

    //have to put the meessage that server is running on port
}
