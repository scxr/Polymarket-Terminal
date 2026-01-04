use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub condition_id: String,
    pub title: String,
    pub price: f64,
    pub size: f64,
    pub side: String,
    pub outcome: String,
    pub proxy_wallet: String,
    pub slug: String,

    
}

#[derive(Deserialize)]
pub struct FullPayload {
    pub payload: Payload,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketData {
    pub id: String,
    pub question: String,
    pub condition_id: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub slug: String,
    pub image: Option<String>,
    pub icon: Option<String>,
    
    pub liquidity: Option<String>,
    #[serde(default)]
    pub volume: String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketSpecificDetails {
    pub id: String,
    pub resolution_source: Option<String>,
    pub end_date: String,
    pub liquidity: String,
    pub start_date: String,
    pub description: String,
    pub outcomes: String,
    pub volume: String,
    pub active: bool,
    pub closed: bool,
    pub volume24hr: f64,
    pub volume1wk: f64,
    pub volume1mo: f64,
    pub volume1yr: f64,
    pub clob_token_ids: String,
    pub spread: f32,
    pub best_bid: f64,
    pub best_ask: f64,

}
