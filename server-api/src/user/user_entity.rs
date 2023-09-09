use uuid::Uuid;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone)]
#[sqlx(rename_all = "snake_case", type_name = "user_role")]
pub enum UserRole {
    Customer,
    Employee
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct UserEntity {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub list_of_orders: Vec<Uuid>,
    pub user_role: UserRole,
    pub createdat: Option<DateTime<Utc>>,
    pub updatedat: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserEntity {
    pub name: String,
    pub email: String,
    pub list_of_orders: Vec<Uuid>,
    pub user_role: UserRole,
}