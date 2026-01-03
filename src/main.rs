#[allow(unused)]
mod data;
mod ui;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::data::state::{AppState, SharedState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state: SharedState = Arc::new(Mutex::new(AppState::new()));

    let ws_state = state.clone();
    tokio::spawn(async move {
        if let Err(e) = data::ws::run(ws_state).await {
            eprintln!("WebSocket error: {:?}", e);
        }
    });

    let ui_state = state.clone();
    tokio::task::spawn_blocking(move || ui::run(ui_state)).await??;

    Ok(())
}