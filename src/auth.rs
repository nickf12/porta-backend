use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub const AUTH_TOKEN: &str = "auth-token";

#[allow(non_snake_case)]
#[derive(Validate, Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Option<Uuid>,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub address: String,
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub address: String,
    pub exp: u64,
}

pub struct _Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl _Keys {
    pub fn _new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
