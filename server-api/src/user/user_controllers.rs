use crate::user::user_services::UserServices;
use crate::user::user_entity::CreateUserEntity;

use actix_web::{get, post, put, delete, Responder, HttpResponse, web};
use serde_json::json;
use uuid::Uuid;

#[get("/")]
pub async fn get_all_users() -> impl Responder {
    let users = UserServices::new().await
    .get_all_users_service().await;

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": users,
    }))
}

#[get("/employees")]
pub async fn get_all_employees() -> impl Responder {
    let employees = UserServices::new().await
    .get_employees_service().await;
    
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": employees,
    }))
}

#[get("/customers")]
pub async fn get_all_customers() -> impl Responder {
    let customers = UserServices::new().await
    .get_customers_service().await;

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": customers,
    }))
}

#[get("/id/{id}")]
pub async fn get_user_by_id(path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();

    let user = UserServices::new().await
    .get_user_by_id_service(id).await;

    if user.is_err() {
        HttpResponse::NotFound().json(json!({
            "status": "fail",
            "error": "user not found",
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "status": "success",
            "userData": user.unwrap(),
        }))
    }
}

#[get("/email/{email}")]
pub async fn get_user_by_email(path: web::Path<String>) -> impl Responder {
    let email = path.into_inner();

    let user = UserServices::new().await
    .get_user_by_email_service(email).await;

    if user.is_err() {
        HttpResponse::NotFound().json(json!({
            "status": "fail",
            "error": "user not found",
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "status": "success",
            "userData": user.unwrap(),
        }))
    }
}


#[post("/")]
pub async fn create_user(body: web::Json<CreateUserEntity>) -> impl Responder {
    let user_to_create = body.into_inner();

    let new_user = UserServices::new().await
    .create_user_service(user_to_create).await;

    HttpResponse::Created().json(json!({
        "status": "success",
        "message": new_user
    }))
}

#[put("/modify/{id}")]
pub async fn modify_user(path: web::Path<Uuid>, body: web::Json<CreateUserEntity>) -> impl Responder {
    let id = path.into_inner();
    let user_data = body.into_inner();

    let modified_user = UserServices::new().await
    .modify_user_service(id, user_data).await;

    HttpResponse::Ok().json(json!({
        "status": "success",
        "userData": modified_user
    }))
}

#[delete("/delete/{id}")]
pub async fn delete_user(path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();

    let deleted_user_id = UserServices::new().await
    .delete_user_service(id).await;

    HttpResponse::Ok().json(json!({
        "status": "success",
        "deletedUserId": deleted_user_id
    }))
}