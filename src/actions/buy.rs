use alloy::network::EthereumWallet;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::LocalSigner;
use alloy::signers::Signer as _;
use std::str::FromStr;
use eyre::Result;
use polymarket_client_sdk::clob::{Client, Config};
use polymarket_client_sdk::{
    clob::types::{Amount, Side},
};
use polymarket_client_sdk::clob::types::response::PostOrderResponse;
use rust_decimal::Decimal;

const CLOB_URL: &str = "https://clob.polymarket.com";

fn parse_string_list(s: &str) -> Vec<String> {
    s.trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .map(|item| item.trim().trim_matches('"').to_string())
        .collect()
}

pub async fn buy_yes(private_key: &str, clob_ids: String, option: &str) -> Result<PostOrderResponse> {
    let clob_ids_parsed = parse_string_list(&clob_ids);


    let opt: String;
    if option == "Yes" {
        opt = clob_ids_parsed.get(0).unwrap().to_string();
    } else {
        opt = clob_ids_parsed.get(1).unwrap().to_string();
    }

    let signer = LocalSigner::from_str(private_key)?.with_chain_id(Some(137));
    let wallet = EthereumWallet::from(signer.clone());

    let _provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect_http("https://polygon-rpc.com".parse()?);
    let _user_address = signer.address();

    let client = Client::new(CLOB_URL, Config::default())?
        .authentication_builder(&signer)
        .authenticate()
        .await?;

    let amount_dec = Decimal::from_str("1.0")?;

    let market_order = client
        .market_order()
        .token_id(opt)
        .amount(Amount::usdc(amount_dec)?)
        .side(Side::Buy)
        .build()
        .await?;

    let signed_order = client.sign(&signer, market_order).await?;
    let _posted_order = client.post_order(signed_order).await?;

    Ok(_posted_order)
}