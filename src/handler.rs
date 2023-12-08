use crate::{Clients, Result};
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reply::json, Reply};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub user_id: String,
}

#[derive(Serialize)]
pub struct UsersResponse {
    pub user_id: usize,
    pub user_name: String,
    pub user_uuid: String,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    user_id: Option<String>,
    message: String,
}

pub async fn users_handler(clients: Clients) -> Result<impl Reply> {
    let users = clients
        .read()
        .await
        .iter()
        .map(|(uuid, client)| {
            let c = client.clone();
            UsersResponse {
                user_name: c.user_name,
                user_id: c.user_id.unwrap(),
                user_uuid: uuid.clone(),
            }
        })
        .collect::<Vec<UsersResponse>>();
    Ok(json(&users))
}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}
