use crate::state::AppState;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use futures_util::{StreamExt, SinkExt};
use std::sync::Arc;
use tracing::info;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let tx = state.tx.clone();
    let mut rx = tx.subscribe();

    let (mut ws_write, mut ws_read) = socket.split();

    let tx_clone = tx.clone();
    let mut send_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = ws_read.next().await {
            info!("Received: {}", text);
            let _ = tx_clone.send(text.to_string());
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if ws_write.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    info!("Client disconnected");
}