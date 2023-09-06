use uuid::Uuid;
use crate::Product::ProductEntity::ProductEntity;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct OrderEntity {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub list_of_products: Vec<ProductEntity>,
    pub createdAt: Option<DateTime<Utc>>,
    pub updatedAt: Option<DateTime<Utc>>,
}