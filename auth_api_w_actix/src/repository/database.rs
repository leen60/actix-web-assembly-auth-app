use chrono::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use core::fmt::Error;
use crate::{models::user::User};
use common::models::credentials::Credentials;
use crate::repository::schema::usr::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct Database {
    pub pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn create_user(&self, user: User) -> Result<User, Error> {
        let hashed_password = crate::utils::password::hash(user.password.as_bytes());
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            name: user.name,
            password: hashed_password,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..user
        };
        diesel::insert_into(usr).values(&user).execute(&mut self.pool.get().unwrap()).expect("Error creating new user");
        Ok(user)
    }

    pub fn get_users(&self) -> Result<Vec<User>, Error> {
        Ok(usr.load::<User>(&mut self.pool.get().unwrap()).expect("Error loading all users"))
    }

    pub fn login_by_password(&self, login_credentials: Credentials) -> Result<User, crate::utils::errors::ErrorResponse>  {
        let all_users = self.get_users().unwrap();
        for i in all_users.iter() {
            if i.name == login_credentials.username {
                let r = crate::utils::password::verify_password(i.password.as_ref(), login_credentials.password.as_bytes());
                match r {
                    Ok(()) => {
                        return Ok(i.clone());
                    },
                    Err(_) => {
                        return Err(crate::utils::errors::ErrorResponse::new("Unauthorized"));
                    }
                }
            }
        }
        Err(crate::utils::errors::ErrorResponse::new("Unauthorized"))
    }
}
