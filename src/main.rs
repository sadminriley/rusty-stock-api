extern crate reqwest;
extern crate tokio;
extern crate dotenv;
extern crate clap;

use reqwest::header::{HeaderMap, UPGRADE_INSECURE_REQUESTS};
use dotenv::dotenv;
use std::env;
use tokio::main;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// CLI args to pass to clap
    symbol: String,
}

async fn get_stock_quote(symbol: &str) -> Result<String, reqwest::Error> {
    dotenv().ok(); // Load environment variables from .env file

    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let mut headers = HeaderMap::new();
    headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());

    let url = format!(
        "https://financialmodelingprep.com/api/v3/quote-short/{}?apikey={}",
        symbol, api_key
    );

    let res = reqwest::Client::new()
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

#[main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Cli::parse();

    match get_stock_quote(&args.symbol).await {
        Ok(quote) => println!("Stock quote for {}: {}", args.symbol, quote),
        Err(e) => eprintln!("Error fetching stock quote: {}", e),
    }

    Ok(())
}
