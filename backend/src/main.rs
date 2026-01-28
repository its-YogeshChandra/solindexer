//create the web server in this only
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use std::io;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("web server in the rust ");
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    //have to put the meessage that server is running on port :
}
