use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct ProductEntity {
    pub id: uuid::Uuid,
    pub name: String,
    pub category: String,
    pub price: i32,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CreateProductEntity {
    pub name: String,
    pub category: String,
    pub price: i32,
    pub quantity: i32,
}