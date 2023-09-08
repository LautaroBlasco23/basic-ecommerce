// modules
#[allow(non_snake_case)]
mod Product;
mod user;
#[allow(non_snake_case)]
mod Order;

use std::env;
use dotenv::dotenv;
use actix_cors::Cors;
use actix_web::{HttpServer, App, web};

// Product Controllers
use Product::ProductControllers::{get_products_controller, create_product_controller, get_product_by_name_controller, 
    get_product_by_id_controller, modify_product_controller, delete_product_controller};
// User Controllers
use user::user_controllers::{create_user, get_all_users, get_all_customers, get_all_employees, modify_user, 
    delete_user, get_user_by_email, get_user_by_id};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // getting env variables and a println if everything's ok. 
    dotenv().ok();
    let port = env::var("PORT").unwrap();
    let url = env::var("SERVER_URL").unwrap();
    println!("server currently running on url: {} port: {}", &url, &port);
    
    HttpServer::new(|| {
        // Cors is permissive because this is a portfolio project.
        let cors = Cors::permissive();

        App::new()
        .wrap(cors)
        .service ( // Routing product controllers
            web::scope("/products")
                .service(create_product_controller)
                .service(get_products_controller)
                .service(get_product_by_id_controller)
                .service(get_product_by_name_controller)
                .service(modify_product_controller)
                .service(delete_product_controller)
        )
        .service ( // Routing user controllers
            web::scope("/users")
                .service(create_user)
                .service(get_all_users)
                .service(get_all_customers)
                .service(get_all_employees)
                .service(get_user_by_email)
                .service(get_user_by_id)
                .service(modify_user)
                .service(delete_user)
        )
        .service(
            web::scope("/order")
        )
    })
    .bind((url, port.parse().unwrap()))?
    .run()
    .await
}