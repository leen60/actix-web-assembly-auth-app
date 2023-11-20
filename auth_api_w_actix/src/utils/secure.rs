use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Serialize,Deserialize};
use chrono::{Duration, Utc};
use crate::{models::user::User};
use actix_web::HttpRequest;

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

fn get_secret() -> Vec<u8> {
    "JWT_SECRET".to_string().into_bytes()
}

pub fn get_jwt_for_user(user: User) -> String {
    let expiration_time = Utc::now().checked_add_signed(Duration::seconds(60)).expect("invalid timestamp").timestamp();
    let user_claims = Claims {
        sub: user.name.clone(),
        role: "Admin".to_string(),
        exp: expiration_time as usize,
    };

    match encode(
         &Header::default(),
         &user_claims,
         &EncodingKey::from_secret(&get_secret()),
     ) {
         Ok(t) => t,
         Err(_) => panic!(),
    }
}


pub fn jwt_from_header(request: HttpRequest) -> std::result::Result<String, crate::utils::errors::ErrorResponse> {
    if get_authorization(&request).is_some() {
        let req_headers = request.headers();
        let basic_auth_header = req_headers.get("Authorization");
        let basic_auth: &str = basic_auth_header.unwrap().to_str().unwrap();

        decode::<Claims>(basic_auth, &DecodingKey::from_secret(&get_secret()), &Validation::default())
            .map_err(|_| crate::utils::errors::ErrorResponse::new("Invalid JWT token"))?;

        Ok(basic_auth.to_string())
    } else {
        Err(crate::utils::errors::ErrorResponse::new("Invalid Header"))
    }
}

fn get_authorization(req: &HttpRequest) -> Option<&str> {
    req.headers().get("Authorization")?.to_str().ok()
}