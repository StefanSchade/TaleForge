use std::sync::Arc;
use actix_web::{HttpResponse, Responder, web};
use crate::web::option_01_actixweb::app_state::AppState;
use crate::web::option_01_actixweb::server::SimpleState;

pub async fn index(data: web::Data<Arc<SimpleState>>) -> impl Responder {
    log::info!("Accessing SimpleState...");
    HttpResponse::Ok().body("SimpleState is accessible!")
}
