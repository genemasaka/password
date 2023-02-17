// import necessary modules
use ethers::providers::{Provider, Http};
use ethers::{utils, prelude::*};
use std::{path::Path, sync::Arc};
use serde::{Deserialize, Serialize};
use serde_json;

// generate Rust bindings for the smart contract using the ABI file
abigen!(Cryptic, "./cryptic_ABI.json", event_derives(serde::Deserialize, serde::Serialize));

// define the client type using SignerMiddleware with Http provider and k256 ECDSA signing key
type Client = SignerMiddleware<Provider<Http>, Walletk256::ecdsa::SigningKey>;

// main function
#[tokio::main]
async fn main() ->Result<(), Box<dyn std::error::Error>> {
// define the HTTP provider with the Goerli testnet endpoint
let provider = Provider::<Http>::try_from(
"https://goerli.infura.io/v3/16087e2d4b5247d589382c9038b12f12"
)?;
// create a local wallet from a private key and set the chain ID to Goerli (5)
let wallet: LocalWallet = "ac5bb51f6a3012f69e637f82fd2c24524149231162a31450d18bd375becfc7f8".parse::<LocalWallet>()?.with_chain_id(5u64);
// create the client with the HTTP provider and local wallet
let client = SignerMiddleware::new(provider.clone(), wallet.clone());
// define the contract address and create the Cryptic contract instance
let address = "0x0F271b49fA57b769c5d5E8329c85Dfa0d0C284D9".parse::<Address>()?;
let contract = Cryptic::new(address.clone(), Arc::new(client.clone()));
// call the encrypt_password function of the contract to encrypt a password and wait for transaction confirmation
let tx = contract.encrypt_password(String::from("highly_paid")).send().await?.await?;
// call the get_encrypted function of the contract to retrieve the encrypted password
let encrypted = contract.get_encrypted().call().await?;
// print the encrypted password and transaction receipt as JSON string
println!("The encrypted password is {}", encrypted);
println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);
// return Ok() if the program completes without error
Ok(())
}
