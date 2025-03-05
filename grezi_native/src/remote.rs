use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::{any, Router},
};
use serde::Deserialize;
use std::{ops::Deref, sync::Arc};

use crate::AppHandle;

#[derive(Clone)]
pub struct Remote {
    pub app_handle: AppHandle,
    pub cached_slideshow_file: Arc<std::path::Path>,
}

#[derive(Deserialize, Debug)]
enum Message {
    Index { index: usize, reset_time: bool },
    Get,
}

impl Remote {
    #[tokio::main(flavor = "current_thread")]
    pub async fn run(self) {
        let app = Router::new()
            .route("/subscribe", any(subscribe))
            .with_state(self);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}

async fn subscribe(ws: WebSocketUpgrade, State(remote): State<Remote>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, remote))
}

async fn handle_socket(mut socket: WebSocket, remote: Remote) {
    while let Some(Ok(message)) = socket.recv().await {
        let message = message.into_data();
        let Ok(message) = postcard::from_bytes::<Message>(&message) else {
            continue;
        };

        match message {
            Message::Index { index, reset_time } => {
                remote
                    .app_handle
                    .root_owner_sender
                    .send(crate::FileOwnerMessage::Index { index, reset_time })
                    .unwrap();
            }
            Message::Get => {
                let file = std::fs::read(remote.cached_slideshow_file.deref()).unwrap();

                socket
                    .send(axum::extract::ws::Message::Binary(file.into()))
                    .await
                    .unwrap();
            }
        }
    }
}
