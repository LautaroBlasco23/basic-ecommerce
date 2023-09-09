use actix_web::{Responder, HttpResponse, get, post, put, delete, web};
use serde_json::json;
use uuid::Uuid;
use crate::order::order_entity::CreateOrderEntity;


#[get("/")]
pub async fn get_all_orders() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "success",
        "data": "Hola"
    }))
}

#[post("/")]
pub async fn create_new_order() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "success",
        "data": "Hola"
    }))
}

#[put("/id/{id}")]
pub async fn modify_order(path: web::Path<Uuid>, body: web::Json<CreateOrderEntity>) -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "success",
        "data": "Hola"
    }))
}

#[delete("/id/{id}")]
pub async fn delete_order(path: web::Path<Uuid>, body: web::Json<CreateOrderEntity>) -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "success",
        "data": "Hola"
    }))
}