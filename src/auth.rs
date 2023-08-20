use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

pub const AUTH_TOKEN: &str = "auth-token";

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Option<String>,
    pub email: String,
    pub password: String,
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
