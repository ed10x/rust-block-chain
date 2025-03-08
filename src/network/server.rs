use actix_web::{web, Responder};
use crate::core::chain::Blockchain;
use actix_web::{get, post, HttpResponse};
use std::sync::{Arc, Mutex};
use crate::core::block::Block;

#[get("/blocks/{index}")]
pub async fn get_block(chain: web::Data<Arc<Mutex<Blockchain>>>, index: web::Path<u64>) -> Box<dyn Responder<Body = actix_web::body::BoxBody>> {
    let chain = chain.lock().unwrap();
    match chain.chain.iter().find(|b| b.index == *index) {
        Some(block) => Box::new(web::Json(block.clone())),
        None => Box::new(HttpResponse::NotFound().finish())
    }
}

#[post("/sync")]
pub async fn sync_chain(
    chain: web::Data<Arc<Mutex<Blockchain>>>,
    new_chain: web::Json<Vec<Block>>
) -> impl Responder {
    let mut local_chain = chain.lock().unwrap();
    if new_chain.len() > local_chain.chain.len() && local_chain.is_chain_valid() {
        local_chain.chain = new_chain.into_inner();
        return HttpResponse::Ok().json(serde_json::json!({ "status": "sync successful" }));
    }
    HttpResponse::BadRequest().json(serde_json::json!({ "error": "invalid chain" }))
}

pub async fn get_blocks(chain: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    web::Json(chain.lock().unwrap().chain.clone())
}