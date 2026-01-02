use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde_json::json;
use futures_util::{SinkExt, StreamExt};

use crate::data::new_markets::get_new_markets;
use crate::data::types::FullPayload;

use crate::data::state::SharedState;

const WS_URL: &str = "wss://ws-live-data.polymarket.com";

pub async fn run(state: SharedState) -> anyhow::Result<()> {
    let (ws_stream, _) = connect_async(WS_URL).await?;
    let (mut write, mut read) = ws_stream.split();

    let sub_req = json!({
        "action": "subscribe",
        "subscriptions": [{
            "topic": "activity",
            "type": "trades"
        }]
    });
    write.send(Message::Text(sub_req.to_string().into())).await?;
    let mut tick = 0;
    while let Some(msg) = read.next().await {
        if let Ok(msg) = msg {
            if let Ok(text) = msg.into_text() {
                process_message(&state, &text);
            }
        }
        if tick % 200 == 0 {
            let new_markets = get_new_markets().await;
            let mut app_state = state.lock().unwrap();
            app_state.set_new_markets(new_markets);
        }
        tick += 1;

    }

    Ok(())
}


fn process_message(state: &SharedState, msg: &str) {
    let mut state = state.lock().unwrap();
    if let Ok(full_payload) = serde_json::from_str::<FullPayload>(msg) {

        let payload = full_payload.payload;
        state.add_trade(payload.title, payload.size, payload.condition_id);
    }

}