use actix_web::web;
use actix_web::{web::{
    Data,
    Json,
}, post, get, HttpResponse, HttpRequest};
use crate::{models::user::User, repository::database::Database};
use common::models::credentials::Credentials;

#[post("/register")]
pub async fn register_user(db: Data<Database>, new_user: Json<User>) -> HttpResponse {
    let user = db.create_user(new_user.into_inner());
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users")]
pub async fn get_users(db: web::Data<Database>, request: HttpRequest) -> HttpResponse {
    match crate::utils::secure::jwt_from_header(request) {
        Ok(_) => {
            let users = db.get_users();
            match users {
                Ok(users) => HttpResponse::Ok().json(users),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        },
        Err(_err) => HttpResponse::InternalServerError().body(_err.to_string()),
    }
}

#[post("/login")]
pub async fn login(db: web::Data<Database>, credentials: Json<Credentials>) -> HttpResponse {
    let user = db.login_by_password(credentials.into_inner());
    
    let token = match user {
        Ok(user) => {
            crate::utils::secure::get_jwt_for_user(user)
        },
        Err(_) => {
            return HttpResponse::InternalServerError().body("Unauthorized".to_string());
        }
    };
    HttpResponse::Ok().json(token)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(register_user)
            .service(get_users)
            .service(login)
    );
}
