use std::env;
use super::ProductEntity::{CreateProductEntity, ProductEntity};
use uuid::Uuid;
// for database
use sqlx::Pool;
use sqlx::postgres::Postgres;

// --------------------------------------------------
// INSERT PRODUCTS INTO DATABASE
// --------------------------------------------------
pub async fn create_product_service (product_data: CreateProductEntity) -> Uuid {
    // Connection to database.
    let db_url = &env::var("DATABASE_URL").unwrap();
    let pool = Pool::<Postgres>::connect(db_url).await;

    let CreateProductEntity { name, price, quantity, category } = product_data;

    // Query to db.
    let query_result = sqlx::query!("
        INSERT INTO products 
        (name, category, price, quantity)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        ",
        name, category, price, quantity
    )
    .fetch_one(&pool.unwrap())
    .await;

    // Returning data.
    query_result.unwrap().id
}

// --------------------------------------------------
// GET PRODUCTS FROM DATABASE
// --------------------------------------------------


pub async fn get_products_service () -> Vec<ProductEntity> {
    // Connection to database.
    let db_url = &env::var("DATABASE_URL").unwrap();
    let pool = Pool::<Postgres>::connect(db_url).await;

    // Query to db.
    let list_of_products: Vec<ProductEntity> = sqlx::query!(
        "SELECT id, name, price, quantity, category FROM products"
    )
    .fetch_all(&pool.unwrap())
    .await
    .unwrap()
    .iter()
    .map(|product| ProductEntity {
        id: product.id,
        name: product.name.to_string(),
        category: product.category.to_string(),
        price: product.price,
        quantity: product.quantity
    })
    .collect();

    // returning data.
    list_of_products
}

pub async fn get_product_by_id_service (user_id: Uuid) -> ProductEntity {
    // Connection to database.
    let db_url = &env::var("DATABASE_URL").unwrap();
    let pool = Pool::<Postgres>::connect(db_url).await;

    // Query to db.
    let query_result = sqlx::query!(
        "SELECT id, name, price, quantity, category FROM products WHERE id=$1",
        user_id
    )
    .fetch_one(&pool.unwrap())
    .await
    .unwrap();

    // returning data.
    ProductEntity {
        id: query_result.id,
        name: query_result.name.to_string(),
        category: query_result.category.to_string(),
        price: query_result.price,
        quantity: query_result.quantity
    }
}

pub async fn get_product_by_name_service (user_name: String) -> Vec<ProductEntity> {
    let pool = Pool::<Postgres>::connect(&env::var("DATABASE_URL").unwrap()).await;


    let list_of_products = sqlx::query!(
        "SELECT id, name, price, quantity, category FROM products WHERE name=$1"
        , user_name
    )
    .fetch_all(&pool.unwrap())
    .await
    .unwrap()
    .iter()
    .map(|product| ProductEntity {
        id: product.id,
        name: product.name.to_string(),
        category: product.category.to_string(),
        price: product.price,
        quantity: product.quantity
    })
    .collect();

    list_of_products
}

pub async fn get_products_by_category_service (category_name: &str) -> Vec<ProductEntity> {
    let pool = Pool::<Postgres>::connect(&env::var("DATABASE_URL").unwrap()).await;


    let list_of_products = sqlx::query!(
        "SELECT id, name, price, quantity, category FROM products WHERE name=$1"
        , category_name
    )
    .fetch_all(&pool.unwrap())
    .await
    .unwrap()
    .iter()
    .map(|product| ProductEntity {
        id: product.id,
        name: product.name.to_string(),
        category: product.category.to_string(),
        price: product.price,
        quantity: product.quantity
    })
    .collect();

    list_of_products
}

// --------------------------------------------------
// MODIFY PRODUCTS IN DATABASE
// --------------------------------------------------


pub async fn modify_product_service (product: ProductEntity) -> Uuid {
    let pool = Pool::<Postgres>::connect(&env::var("DATABASE_URL").unwrap()).await;

    let ProductEntity { id, name, category, price, quantity } = product;

    let query_result = sqlx::query!(
        "UPDATE products 
        SET name = $1,
        price = $2, 
        quantity = $3, 
        category = $4
        WHERE id=$5
        RETURNING id",
        name, price, quantity, category, id
    )
    .fetch_one(&pool.unwrap())
    .await
    .unwrap();

    query_result.id
}

// --------------------------------------------------
// DELETE PRODUCTS FROM DATABASE
// --------------------------------------------------

pub async fn delete_product_service (id: Uuid) -> Uuid {
    let pool = Pool::<Postgres>::connect(&env::var("DATABASE_URL").unwrap()).await;

    sqlx::query!(
        "DELETE FROM products WHERE id = $1",
        id
    )
    .fetch_one(&pool.unwrap())
    .await
    .unwrap();

    id
}