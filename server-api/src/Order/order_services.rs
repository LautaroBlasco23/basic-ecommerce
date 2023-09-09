use std::env;

use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::order_entity::{OrderEntity, CreateOrderEntity};




pub struct OrderRepository {
    db: Pool<Postgres>
}

impl OrderRepository {
    pub async fn new() -> OrderRepository {
        let db_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        OrderRepository {
            db: Pool::<Postgres>::connect(db_url).await.expect("can't connect db")
        }
    }

    pub async fn get_all_orders(&self) -> Result<Vec<OrderEntity>, sqlx::Error> {
        sqlx::query_as!(OrderEntity, "SELECT * FROM orders")
        .fetch_all(&self.db)
        .await
    }

    pub async fn get_order_by_id(&self, id: Uuid) -> Result<OrderEntity, sqlx::Error> {
        sqlx::query_as!(OrderEntity, "
        SELECT *
        FROM orders
        WHERE id=$1",
        id)
        .fetch_one(&self.db)
        .await
    }

    pub async fn get_order_by_user_id(&self, user_id: Uuid) -> Result<OrderEntity, sqlx::Error> {
        sqlx::query_as!(OrderEntity, 
        "SELECT *
        FROM orders
        WHERE user_id =$1
        ", user_id
        )
        .fetch_one(&self.db)
        .await
    }

    pub async fn create_order(&self, order_data: CreateOrderEntity) -> Result<OrderEntity, sqlx::Error> {
        let CreateOrderEntity { user_id, list_of_products } = order_data;
        let now = Utc::now();

        sqlx::query_as!(OrderEntity, "
        INSERT INTO orders
        (user_id, list_of_products, createdat, updatedat)
        VALUES
        ($1, $2, $3, $3)
        RETURNING *
        ", user_id, &list_of_products, now)
        .fetch_one(&self.db)
        .await
    }

    pub async fn modify_order(&self, id: Uuid, order_data: CreateOrderEntity) -> Result<OrderEntity, sqlx::Error> {
        let CreateOrderEntity { user_id, list_of_products } = order_data;
        let now = Utc::now();

        sqlx::query_as!(OrderEntity, "
        UPDATE orders
        SET
        user_id = $1,
        list_of_products = $2,
        updatedat = $3
        WHERE id = $4
        RETURNING *
        ", user_id, &list_of_products, now, id)
        .fetch_one(&self.db)
        .await
    }

    pub async fn delete_order(&self, id: Uuid) -> Result<Uuid, sqlx::Error> {
        let query_result = sqlx::query!("
        DELETE FROM orders
        WHERE id = $1
        ", id)
        .fetch_one(&self.db)
        .await;

        // We use this if statement, because the query returns a Record, not an Uuid, and we want to only return the id.
        match query_result {
            Ok(_) => Ok(id),
            Err(error) => Err(error),
        }
    }
}