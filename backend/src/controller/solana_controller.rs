use crate::MainDataStruct;
use actix_web::{HttpResponse, Responder, get, post, web};

//fetch data
#[post("/data")]
pub async fn fetch_data(data: web::Data<MainDataStruct>) -> impl Responder {
    let value = data.data.lock().unwrap();
    let main_value = value.to_string();
    println!("the body data is : {}", &main_value);
    HttpResponse::Ok().body(main_value)
}
