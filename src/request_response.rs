use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    // TODO: determine what we need to a newly created user
    // TODO: determine if db is needed - it probably is
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    pub user_id: String,
}

#[derive(Serialize, Debug)]
pub struct LoginRequest {
    pub user_id: String,
    pub passphrase: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub user_id: String,
    pub device_id: String,
}