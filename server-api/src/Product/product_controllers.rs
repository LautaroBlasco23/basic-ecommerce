use actix_web::{get, post, Responder, HttpResponse, delete, put, web};
use uuid::Uuid;
use super::product_services::{get_products_service, create_product_service, get_product_by_id_service,
                            get_product_by_name_service, modify_product_service, delete_product_service,
                            get_products_by_category_service};
use super::product_entity::{ProductEntity, CreateProductEntity};

// --------------------------------------------------
// POST REQUESTS
// --------------------------------------------------

#[post("/create/")]
pub async fn create_product_controller(entry_data: web::Json<CreateProductEntity>) -> impl Responder {

    let product_data = CreateProductEntity {
        name: entry_data.name.clone(),
        category: entry_data.category.clone(),
        price: entry_data.price,
        quantity: entry_data.quantity
    };

    let product_id = create_product_service(product_data).await;

    HttpResponse::Created().json(serde_json::json!({
        "status": "success",
        "new_product_id": product_id,
    }))
}

// --------------------------------------------------
// GET REQUESTS
// --------------------------------------------------

// Get all products.
#[get("/")]
pub async fn get_products_controller() -> impl Responder {
    let list_of_products = get_products_service().await;

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "list_of_products": list_of_products,
    }))
}

// Get product by id.
#[get("/id/{id}")]
pub async fn get_product_by_id_controller(path: web::Path<Uuid>) -> impl Responder {
    // Getting query param.
    let user_id = path.into_inner();

    let list_of_products = get_product_by_id_service(user_id).await;

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "list_of_products": list_of_products,
    }))
}


// Get product by name.
#[get("/name/{name}")]
pub async fn get_product_by_name_controller(path: web::Path<String>) -> impl Responder {
    // Getting query param.
    let user_name = path.into_inner();

    let list_of_products = get_product_by_name_service(user_name).await;

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "list_of_products": list_of_products,
    }))
}

// Get product by name.
#[get("/category/{category}")]
pub async fn get_product_by_category_controller(path: web::Path<String>) -> impl Responder {
    // Getting query param.
    let category = path.into_inner();

    let list_of_products = get_products_by_category_service(&category).await;

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "list_of_products": list_of_products,
    }))
}

// --------------------------------------------------
// PUT REQUESTS
// --------------------------------------------------

#[put("/update/id/{id}")]
pub async fn modify_product_controller(path: web::Path<Uuid>, info: web::Json<CreateProductEntity>) -> impl Responder {
    let product_data = ProductEntity {
        id: path.into_inner(),
        name: info.name.clone(),
        category: info.category.clone(),
        price: info.price,
        quantity: info.quantity
    };

    let product_id = modify_product_service(product_data).await;

    HttpResponse::Ok().json(serde_json::json!({
        "status": "successs",
        "modified_product_id": product_id,
    }))
}

// --------------------------------------------------
// DELETE REQUESTS
// --------------------------------------------------

#[delete("/delete/id/{id}")]
pub async fn delete_product_controller(path: web::Path<Uuid>) -> impl Responder {
    let id: Uuid = path.into_inner();

    let product_id = delete_product_service(id).await;

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "deleted_product_id": product_id,
    }))
}