use crate::data::{state::SharedState, types::MarketData};

pub async fn get_new_markets() -> Vec<(String, String)> {
    let mut output = Vec::new();
    let url = "https://gamma-api.polymarket.com/markets?limit=1000&closed=false&order=createdAt&ascending=false";
    
    // Do all the async work BEFORE locking
    if let Ok(resp) = reqwest::get(url).await {
        if let Ok(text) = resp.text().await {
            if let Ok(markets) = serde_json::from_str::<Vec<MarketData>>(&text) {
                for market in markets {
                    output.push((market.question, market.volume));
                }
            }
        }
    }
    
    output
}