use actix_web::{Responder, HttpResponse, get, post, put, delete, web};
use serde_json::json;
use uuid::Uuid;
use crate::order::order_entity::CreateOrderEntity;
use crate::order::order_services::OrderServices;


#[get("/")]
pub async fn get_all_orders() -> impl Responder {
    let orders = OrderServices::new().await.get_all_orders().await;

    match orders {
        Ok(list_of_orders) => {
            HttpResponse::Ok().json(json!({
                "status": "success",
                "data": list_of_orders
            }))
        }
        Err(error) => {
            println!("{:?}", error);
            HttpResponse::InternalServerError().json(json!({
                "status": "fail",
                "message": "something went wrong when getting the orders!"
            }))
        }
    }
}

#[get("/id/{id}")]
pub async fn get_order_by_id(path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();
    let order = OrderServices::new().await.get_order_by_id(id).await;

    match order {
        Ok(order_data) => {
            HttpResponse::Ok().json(json!({
                "status": "success",
                "data": order_data
            }))
        }
        Err(error) => {
            println!("{:?}", error);
            HttpResponse::InternalServerError().json(json!({
                "status": "fail",
                "message": "something went wrong when getting order data!"
            }))
        }
    }
}

#[get("/user_id/{user_id}")]
pub async fn get_order_by_user_id(path: web::Path<Uuid>) -> impl Responder {
    let user_id = path.into_inner();
    let orders = OrderServices::new().await.get_order_by_user_id(user_id).await;

    match orders {
        Ok(list_of_orders) => {
            HttpResponse::Ok().json(json!({
                "status": "success",
                "data": list_of_orders
            }))
        }
        Err(error) => {
            println!("{:?}", error);
            HttpResponse::InternalServerError().json(json!({
                "status": "fail",
                "message": "something went wrong when getting user orders!"
            }))
        }
    }
}

#[post("/")]
pub async fn create_new_order(body: web::Json<CreateOrderEntity>) -> impl Responder {
    let new_order_data = body.into_inner();
    let order = OrderServices::new().await.create_order(new_order_data).await;

    match order {
        Ok(order_data) => {
            HttpResponse::Ok().json(json!({
                "status": "success",
                "data": order_data
            }))
        }
        Err(error) => {
            println!("{:?}", error);
            HttpResponse::BadRequest().json(json!({
                "status": "fail",
                "message": "something went wrong when creating the order!"
            }))
        }
    }  
}

#[put("/id/{id}")]
pub async fn modify_order(path: web::Path<Uuid>, body: web::Json<CreateOrderEntity>) -> impl Responder {
    let id = path.into_inner();
    let order_data = body.into_inner();

    let modified_order = OrderServices::new().await.modify_order(id, order_data).await;
    
    match modified_order {
        Ok(order_data) => {
            HttpResponse::Ok().json(json!({
                "status": "success",
                "data": order_data
            }))
        }
        Err(error) => {
            println!("{:?}", error);
            HttpResponse::InternalServerError().json(json!({
                "status": "fail",
                "message": "Something went wrong when modifying the order!"
            }))
        }
    }
}

#[delete("/id/{id}")]
pub async fn delete_order(path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();
    let deleted_order_id = OrderServices::new().await.delete_order(id).await;

    match deleted_order_id {
        Ok(order_id) => {
            HttpResponse::Ok().json(json!({
                "status": "success",
                "data": order_id
            }))
        }
        Err(error) => {
            println!("{:?}", error);
            HttpResponse::InternalServerError().json(json!({
                "status": "fail",
                "message": "something went wrong when deleting the order!"
            }))
        }
    }

}