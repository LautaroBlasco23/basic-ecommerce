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
use Product::ProductControllers::{get_products_controller, create_product_controller, get_product_by_name_controller, get_product_by_id_controller /*, modify_product, delete_product */};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // getting env variables and a println if everything's ok. 
    dotenv().ok();
    let PORT = env::var("PORT").unwrap();
    let URL = env::var("SERVER_URL").unwrap();
    println!("server currently running on url: {} port: {}", &URL, &PORT);
    
    HttpServer::new(|| {
        // Cors is permissive because this is a portfolio project.
        let cors = Cors::permissive();

        App::new()
        .wrap(cors)
        .service ( // Routing the controllers.
            web::scope("/products")
                .service(create_product_controller)
                .service(get_products_controller)
                .service(get_product_by_id_controller)
                .service(get_product_by_name_controller)
                /*
                .service(modify_product)
                .service(delete_product)
                */
        )
        .service(
            web::scope("/users")
        )
        .service(
            web::scope("/order")
        )
    })
    .bind((URL, PORT.parse().unwrap()))?
    .run()
    .await
}