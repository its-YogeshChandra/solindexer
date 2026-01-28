use crate::MainDataStruct;
use actix_web::{HttpResponse, Responder, get, post, web};

//fetch data
#[post("/data")]
pub async fn fetch_data(data: web::Json<MainDataStruct>) -> impl Responder {
    let value = data.data.to_string();
    println!("the body data is : {}", &value);
    HttpResponse::Ok().body(value)
}
