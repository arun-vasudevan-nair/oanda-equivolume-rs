# Equivolume Technical Indicator (Rust)

This CLI tool fetches forex candle data from Oanda API and calculates the Equivolume technical indicator.

## Prerequisites

- Rust (latest stable)
- Oanda API Key (Live or Practice)
- Oanda Account ID (optional, default provided)

## Setup

1.  Clone the repository.
2.  Create a `.env` file in the root directory (or set environment variables):

    ```env
    OANDA_API_KEY=your_api_key_here
    OANDA_ACCOUNT_ID=your_account_id_here
    OANDA_ENV=practice  # or live
    ```

## Running

```bash
cargo run
```

This will fetch EUR_USD daily candles and print the Equivolume data (Time, Volume, Open, High, Low, Close).

## Running Tests

```bash
cargo test
```
