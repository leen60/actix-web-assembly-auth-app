use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}