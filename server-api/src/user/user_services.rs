use std::env;
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::user::user_entity::{UserEntity, CreateUserEntity, UserRole};

pub struct UserServices {
    db: Pool<Postgres>
}
impl UserServices {
    pub async fn new() -> UserServices {
        let db_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        UserServices {
            db: Pool::<Postgres>::connect(db_url).await.expect("can't connect db")
        }
    }

    pub async fn get_all_users_service(&self) -> Vec<UserEntity> {
        sqlx::query_as!(UserEntity, r#"SELECT id, name, email, createdat, updatedat, list_of_orders, user_role AS "user_role!: UserRole" FROM users"#)
        .fetch_all(&self.db)
        .await
        .expect("something went wrong when getting data from db.")
    }
    
    pub async fn get_employees_service(&self) -> Vec<UserEntity> {
        sqlx::query_as!(UserEntity, r#"
        SELECT id, name, email, createdat, updatedat, list_of_orders, user_role AS "user_role!: UserRole"
        FROM users
        WHERE user_role=$1"#,
        UserRole::Employee as UserRole)
        .fetch_all(&self.db)
        .await
        .expect("something went wrong when getting data from db.")
    }
    
    pub async fn get_customers_service(&self) -> Vec<UserEntity> {
        sqlx::query_as!(UserEntity, r#"
        SELECT id, name, email, createdat, updatedat, list_of_orders, user_role AS "user_role!: UserRole"
        FROM users
        WHERE user_role=$1"#,
        UserRole::Customer as UserRole)
        .fetch_all(&self.db)
        .await
        .expect("something went wrong when getting data from db.")
    }
    
    pub async fn get_user_by_id_service(&self, id: Uuid) -> Result<UserEntity, sqlx::Error> {
        sqlx::query_as!(UserEntity, r#"
        SELECT id, name, email, createdat, updatedat, list_of_orders, user_role AS "user_role!: UserRole"
        FROM users
        WHERE id=$1"#,
        id)
        .fetch_one(&self.db)
        .await
    }
    
    pub async fn get_user_by_email_service(&self, email: String) -> Result<UserEntity, sqlx::Error> {
        sqlx::query_as!(UserEntity, r#"
        SELECT id, name, email, createdat, updatedat, list_of_orders, user_role AS "user_role!: UserRole"
        FROM users
        WHERE email=$1"#,
        email)
        .fetch_one(&self.db)
        .await
    }
    
    pub async fn create_user_service(&self, user_data: CreateUserEntity) -> UserEntity {
        let CreateUserEntity { name, email, list_of_orders, user_role} = user_data;
        let now = Utc::now();
        
        sqlx::query_as!(UserEntity, r#"
        INSERT INTO users
        (name, email, list_of_orders, createdat, updatedat, user_role) 
        VALUES 
        ($1, $2, $3, $4, $5, $6)
        RETURNING id, name, email, list_of_orders, createdat, updatedat, user_role AS "user_role!: UserRole"
        "#,
        name, email, &list_of_orders, now, now, user_role as UserRole
        )
        .fetch_one(&self.db)
        .await
        .expect("Something went wrong when inserting data")
    }
    
    pub async fn modify_user_service(&self, id: Uuid, user_data: CreateUserEntity) -> UserEntity {
        let CreateUserEntity { name, email, list_of_orders, user_role} = user_data;
        let now = Utc::now();
        
        sqlx::query_as!(UserEntity, r#"
        UPDATE users
        SET 
        name = $1,
        email = $2, 
        list_of_orders = $3,
        updatedat = $4,
        user_role = $5
        WHERE id = $6
        RETURNING id, name, email, list_of_orders, createdat, updatedat, user_role AS "user_role!: UserRole"
        "#,
        name, email, &list_of_orders, now, user_role as UserRole, id
        )
        .fetch_one(&self.db)
        .await
        .expect("Something went wrong when inserting data")
    }
    
    pub async fn delete_user_service(&self, id: Uuid) -> Uuid {
        sqlx::query!("
        DELETE FROM users
        WHERE id=$1
        RETURNING id
        ",
        id
        )
        .fetch_one(&self.db)
        .await
        .expect("Something went wrong when inserting data")
        .id
    }
}