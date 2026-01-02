use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub condition_id: String,
    pub title: String,
    pub price: f64,
    pub size: f64,
    pub side: String,
    pub outcome: String,
    pub proxy_wallet: String,

    
}

#[derive(Debug, Deserialize)]
pub struct FullPayload {
    pub payload: Payload,
}

#[derive(Debug, Deserialize)]
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
    
    pub liquidity: Option<String>,  // It's a string in the API, not f64
    #[serde(default)]
    pub volume: String,  // Might not be present or might be a string
}

#[derive(Debug, Deserialize)]
pub struct MarketsResponse {
    pub markets: Vec<MarketData>,
}