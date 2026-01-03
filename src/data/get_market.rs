use thiserror::Error;
use crate::data::types::MarketSpecificDetails;

#[derive(Error, Debug)]
pub enum MarketError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("No market found for slug")]
    NotFound,
}

pub async fn get_market_from_slug(market_slug: &str) -> Result<MarketSpecificDetails, MarketError> {
    let url = format!("https://gamma-api.polymarket.com/markets/slug/{}", market_slug);

    let response = reqwest::get(&url).await?;
    let status = response.status();

        let body = response.text().await?;



    // Now try to parse
    let data: MarketSpecificDetails = serde_json::from_str(&body)
        .map_err(|e| {
            eprintln!("JSON parse error: {}", e);
            eprintln!("Raw response: {}", body);
            MarketError::Json(e)

        })?;

    Ok(data)
}