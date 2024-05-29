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
    /// The v3 API endpoint to use (quote-short,profile,etc. Found here https://site.financialmodelingprep.com/developer/docs
    v3endpoint: String,
    /// Stock ticker symbol
    symbol: String,
}

async fn get_stock_quote(v3endpoint: &str, symbol: &str) -> Result<String, reqwest::Error> {
    dotenv().ok(); // Load environment variables from .env file

    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let mut headers = HeaderMap::new();
    headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());

    let url = format!(
        "https://financialmodelingprep.com/api/v3/{}/{}?apikey={}",
        v3endpoint, symbol, api_key
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

    match get_stock_quote(&args.v3endpoint, &args.symbol).await {
        Ok(quote) => println!("Stock quote for {} at {}: {}", args.symbol, args.v3endpoint, quote),
        Err(e) => eprintln!("Error fetching stock quote: {}", e),
    }

    Ok(())
}

