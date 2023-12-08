use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use warp::ws::Message;
use warp::{Filter, Rejection};

mod handler;
mod ws;

type Result<T> = std::result::Result<T, Rejection>;

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: Option<usize>,
    pub user_name: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

type Clients = Arc<RwLock<HashMap<String, Client>>>;

#[tokio::main]
async fn main() {
    let clients = generate_clients();

    let health_route = warp::path!("health").and_then(handler::health_handler);
    let user_route = warp::path!("users")
        .and(with_clients(clients.clone()))
        .and_then(handler::users_handler);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(handler::ws_handler);

    let routes = health_route
        .or(user_route)
        .or(ws_route)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn generate_clients() -> Clients {
    let names = vec!["Adrian", "Alice", "Ben", "Bob", "Kevin"];
    let mut users: HashMap<String, Client> = HashMap::new();
    for (i, name) in names.iter().enumerate() {
        users.insert(
            Uuid::new_v4().as_simple().to_string(),
            Client {
                user_id: Some(i),
                sender: None,
                user_name: name.to_string(),
            },
        );
    }
    Arc::new(RwLock::new(users))
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
