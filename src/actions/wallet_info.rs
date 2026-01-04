use alloy::{primitives::address, providers::ProviderBuilder, sol};
use std::error::Error;
use std::str::FromStr;
use alloy::network::EthereumWallet;
use alloy::primitives::{Address, U256};
use alloy::signers::local::LocalSigner;
use alloy::signers::Signer as _;
use crate::actions::approvals::{is_fully_approved, ApprovalStatus};
use alloy::providers::{Provider, RootProvider};
use alloy::transports::http::Http;
use reqwest::Client;

const USDCE: Address = address!("0x2791bca1f2de4661ed88a30c99a7a9449aa84174");

sol! {
    #[sol(rpc)]
    contract ERC20 {
        function balanceOf(address owner) public view returns (uint256);
    }
}
pub async fn get_wallet_full(private_key: &str) -> Result<(Address, f64, f64, bool), Box<dyn Error>> {
    let signer = LocalSigner::from_str(private_key)?.with_chain_id(Some(137));
    let wallet = EthereumWallet::from(signer.clone());
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect_http("https://polygon-rpc.com".parse()?);
    let user_address = signer.address();

    let is_approved = is_fully_approved(&provider, user_address).await?;

    let pol_balance = provider.get_balance(user_address).await?;
    let erc20 = ERC20::new(USDCE, provider.clone());
    let balance = erc20.balanceOf(user_address).call().await?;
    let usdce_balance = balance.to::<u128>() as f64 / 1e6;
    let balance_f64 = pol_balance.to::<u128>() as f64 / 1e18;
    Ok((user_address, usdce_balance, balance_f64, is_approved))
}