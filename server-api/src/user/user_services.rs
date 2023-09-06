use std::env;
use chrono::Utc;
use sqlx::{Pool, Postgres};

use crate::user::user_entity::{UserEntity, CreateUserEntity, UserRole};

struct UserServices {
    db: Pool<Postgres>
}
impl UserServices {
    pub async fn new() -> UserServices {
        let db_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        UserServices {
            db: Pool::<Postgres>::connect(db_url).await.expect("can't connect db")
        }
    }

    pub async fn get_all_users_service(&self) -> Result<Vec<UserEntity>, sqlx::Error> {

        sqlx::query_as!(UserRole, "SELECT user_role FROM users")
        .fetch_all(&self.db)
        .await;

        vec![UserEntity{}]
    }
    
    pub fn get_employees_service() -> Vec<UserEntity> {}
    
    pub fn get_customers_service() -> Vec<UserEntity> {}
    
    pub fn get_user_by_id_service() -> Vec<UserEntity> {}
    
    pub fn get_user_by_email_service() -> Vec<UserEntity> {}
    
    pub fn create_user_service(user_data: CreateUserEntity) -> Vec<UserEntity> {
        let CreateUserEntity { name, email, list_of_orders, type_of_user} = user_data;
        let now = Utc::now();

        sqlx::query_as!(UserEntity, "
        INSERT INTO users 
        (name, email, list_of_orders, createdat, updatedat, user_role)
        VALUES
        ($1, $2, $3, $4, $5, $6)
        ",
        name, email, list_of_orders, now, now, user_role as user_roole
        )
        .fetch_all(&self.db)
        .await
    }
    
    pub fn modify_user_service() -> Vec<UserEntity> {}
    
    pub fn delete_user_service() -> Vec<UserEntity> {}
}