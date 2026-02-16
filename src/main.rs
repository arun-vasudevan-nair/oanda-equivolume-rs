mod oanda;
mod equivolume;

use std::env;
use dotenv::dotenv;
use oanda::{OandaClient, Environment};
use equivolume::calculate;
use anyhow::{Result, Context};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api_key = env::var("OANDA_API_KEY")
        .context("OANDA_API_KEY must be set in .env or environment")?;
    
    // Default to practice environment unless specified
    let env_str = env::var("OANDA_ENV").unwrap_or_else(|_| "practice".to_string());
    let environment = match env_str.to_lowercase().as_str() {
        "live" => Environment::Live,
        _ => Environment::Practice,
    };

    let account_id = env::var("OANDA_ACCOUNT_ID").unwrap_or_else(|_| "101-000-0000000-001".to_string()); // Placeholder if not provided

    let client = OandaClient::new(&api_key, &account_id, environment)?;

    let instrument = "EUR_USD";
    let granularity = "D"; // Daily candles
    let count = 10;

    println!("Fetching {} {} candles for {}...", count, granularity, instrument);

    let candles = client.get_candles(instrument, granularity, count).await?;
    
    println!("Fetched {} candles. Calculating Equivolume...", candles.len());

    let boxes = calculate(&candles)?;

    println!("\nEquivolume Data:");
    println!("{:<25} | {:<10} | {:<10} | {:<10} | {:<10} | {:<10}", "Time", "Volume", "Open", "High", "Low", "Close");
    println!("{}", "-".repeat(85));

    for b in boxes {
        println!("{:<25} | {:<10} | {:<10.5} | {:<10.5} | {:<10.5} | {:<10.5}", 
            b.time, b.volume, b.open, b.high, b.low, b.close);
    }

    Ok(())
}