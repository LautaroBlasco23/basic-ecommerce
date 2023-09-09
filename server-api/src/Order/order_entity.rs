use uuid::Uuid;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct OrderEntity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub list_of_products: Vec<Uuid>,
    #[serde(rename = "createdAt")]
    pub createdat: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updatedat: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct CreateOrderEntity {
    pub user_id: Uuid,
    pub list_of_products: Vec<Uuid>
}