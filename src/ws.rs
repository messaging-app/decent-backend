use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

use crate::{Client, Clients};

pub async fn client_connection(ws: WebSocket, id: String, clients: Clients, mut client: Client) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    client.sender = Some(client_sender);
    clients.write().await.insert(id.clone(), client);

    println!("{} connected", id);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving ws message for id: {}): {}", id.clone(), e);
                break;
            }
        };
        client_msg(&id, msg, &clients).await;
    }

    clients.write().await.remove(&id);
    println!("{} disconnected", id);
}

async fn client_msg(id: &str, msg: Message, clients: &Clients) {
    if !msg.is_text() {
        eprintln!("Message not text");
        return;
    }

    println!("received message from {}", id);
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    clients
        .read()
        .await
        .iter()
        .for_each(|(uuid, client)| forward_msg(message, uuid, id, client.clone()));
}

fn forward_msg(msg: &str, uuid: &str, sender_id: &str, client: Client) {
    if !msg.contains(uuid) {
        println!("Cannot forward message");
        return;
    }
    println!("Forward message to: {}", uuid);
    if let Some(s) = client.sender {
        let _ = s.send(Ok(Message::text(msg.replace(uuid, sender_id))));
    } else {
        eprintln!("Receiver offline");
    }
}
