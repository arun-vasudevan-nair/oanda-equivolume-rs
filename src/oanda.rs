use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum Environment {
    Practice,
    Live,
}

impl Environment {
    pub fn base_url(&self) -> &str {
        match self {
            Environment::Practice => "https://api-fxpractice.oanda.com",
            Environment::Live => "https://api-fxtrade.oanda.com",
        }
    }
}

#[derive(Debug, Clone)]
pub struct OandaClient {
    client: Client,
    base_url: String,
    #[allow(dead_code)]
    account_id: String,
}

impl OandaClient {
    pub fn new(api_key: &str, account_id: &str, env: Environment) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        let auth_value = format!("Bearer {}", api_key);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&auth_value)?,
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            base_url: env.base_url().to_string(),
            account_id: account_id.to_string(),
        })
    }

    pub async fn get_candles(
        &self,
        instrument: &str,
        granularity: &str,
        count: u32,
    ) -> Result<Vec<Candle>> {
        let url = format!(
            "{}/v3/instruments/{}/candles",
            self.base_url, instrument
        );

        let count_str = count.to_string();
        let query_params = [
            ("granularity", granularity),
            ("count", count_str.as_str()),
            ("price", "M"),
        ];

        let response = self.client
            .get(&url)
            .query(&query_params)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!("API Error {}: {}", status, text);
        }

        let wrapper: CandleResponse = response.json().await?;
        Ok(wrapper.candles)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CandleResponse {
    pub instrument: String,
    pub granularity: String,
    pub candles: Vec<Candle>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Candle {
    pub time: String,
    pub volume: i64,
    pub complete: bool,
    #[serde(rename = "mid")]
    pub mid: Option<Ohlc>, // Use Option as it depends on "price" param (M, B, A)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ohlc {
    pub o: String,
    pub h: String,
    pub l: String,
    pub c: String,
}
