use crate::user::user_services::UserServices;
use crate::user::user_entity::{CreateUserEntity, UserRole};

use actix_web::{get, post, Responder, HttpResponse};
use serde_json::json;

#[get("/")]
pub async fn get_all_users() -> impl Responder {
    let users = UserServices::new().await
    .get_all_users_service().await
    .expect("Controller error getting data from service.");

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": users,
    }))
}

#[post("/")]
pub async fn create_user() -> impl Responder {
    let user_to_create = CreateUserEntity {
        name: "Lautaro".to_string(),
        email: "lauti_blasco@hotmail.es".to_string(),
        list_of_orders: vec![],
        user_role: UserRole::Customer
    };

    let new_user = UserServices::new().await
    .create_user_service(user_to_create).await;

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": new_user
    }))
}