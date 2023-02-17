use ethers::providers::{Provider, Http};
use ethers::{utils, prelude::*};
use std::{path::Path, sync::Arc};
use serde::{Deserialize, Serialize};
use serde_json;
// use yew::prelude::*;

abigen!(Cryptic, "./cryptic_ABI.json", event_derives(serde::Deserialize, serde::Serialize));

type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

#[tokio::main]
async fn main() ->Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(
        "https://goerli.infura.io/v3/16087e2d4b5247d589382c9038b12f12"
        )?;
    let wallet: LocalWallet = "ac5bb51f6a3012f69e637f82fd2c24524149231162a31450d18bd375becfc7f8".parse::<LocalWallet>()?.with_chain_id(5u64);
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let address = "0x0F271b49fA57b769c5d5E8329c85Dfa0d0C284D9".parse::<Address>()?;
    let contract = Cryptic::new(address.clone(), Arc::new(client.clone()));
    let tx = contract.encrypt_password(String::from("highly_paid")).send().await?.await?;
    let encrypted = contract.get_encrypted().call().await?;
    println!("The encrypted password is {}", encrypted);
    println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);
    Ok(())
}
