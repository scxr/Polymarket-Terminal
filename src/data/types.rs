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

