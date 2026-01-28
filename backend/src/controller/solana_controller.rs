use crate::MainDataStruct;
use actix_web::{HttpResponse, Responder, web};

//fetch data
pub async fn fetch_data(data: web::Data<MainDataStruct>) -> impl Responder {
    let value = data.data.lock().unwrap();
    let main_value = value.to_string();
    HttpResponse::Ok().body(main_value)
}
