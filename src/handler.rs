use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reply::json, Reply};

use crate::{ws, Clients, Result};

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

pub async fn ws_handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply> {
    let client = clients.read().await.get(&id).cloned();
    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, id, clients, c))),
        None => Err(warp::reject::not_found()),
    }
}
