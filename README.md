# rusty-stock-cli
A stock cli tool written in Rust using the [Financial Modeling Prep API.](https://site.financialmodelingprep.com/developer/docs "Financial Modeling Prep API.")

[![CircleCI](https://dl.circleci.com/status-badge/img/gh/sadminriley/rusty-stock-cli/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/sadminriley/rusty-stock-cli/tree/main)



This is a Rust ported, updated version of my Python repo with similar functionality located here [FlaskStockAPI](https://github.com/sadminriley/FlaskStockAPI)



Example using clap for v3 endpoint requests -

```
$ cargo run -- --endpoint quote-order --query TSLA

     Running `target/debug/financial-modeling-prep-api quote-order TSLA`
Stock quote for TSLA at quote-order: [
  {
    "symbol": "TSLA",
    "name": "Tesla, Inc.",
    "price": 176.75,
    "changesPercentage": -1.3892,
    "change": -2.49,
    "dayLow": 173.16,
    "dayHigh": 178.2499,
    "yearHigh": 299.29,
    "yearLow": 138.8,
    "marketCap": 563691100000,
    "priceAvg50": 171.448,
    "priceAvg200": 213.3502,
    "exchange": "NASDAQ",
    "volume": 59394590,
    "avgVolume": 96260890,
    "open": 176.4,
    "previousClose": 179.24,
    "eps": 3.91,
    "pe": 45.2,
    "earningsAnnouncement": "2024-07-17T10:59:00.000+0000",
    "sharesOutstanding": 3189200000,
    "timestamp": 1716926400
  }
]
```


### Improved endpoints for v3
```
$ cargo run -- --endpoint quote-short --query AAPL
     Running `target/debug/financial-modeling-prep-api --endpoint quote-short --query AAPL`
Response: [
  {
    "symbol": "AAPL",
    "price": 189.99,
    "volume": 52071035
  }
]
```


#### And, if we had a paid version of the v4 key this would also work, in theory

```
$ cargo run -- --v4endpoint commitment_of_traders_reports --query AAPL
     Running `target/debug/financial-modeling-prep-api --v4endpoint commitment_of_traders_reports --query AAPL`
Response: {
  "Error Message": "Special Endpoint : This endpoint is not available under your current subscription please visit our subscription page to upgrade your plan at https://site.financialmodelingprep.com/developer/docs/pricing"
}

```

## Running with Rust binary

```




$ cargo run --bin rusty-stock-cli -- --endpoint quote-order --query TSLA
     Running `target/debug/rusty-stock-cli --endpoint quote-order --query TSLA`
Response: [
  {
    "symbol": "TSLA",
    "name": "Tesla, Inc.",
    "price": 176.75,
    "changesPercentage": -1.3892,
    "change": -2.49,
    "dayLow": 173.16,
    "dayHigh": 178.2499,
    "yearHigh": 299.29,
    "yearLow": 138.8,
    "marketCap": 563691100000,
    "priceAvg50": 171.448,
    "priceAvg200": 213.3502,
    "exchange": "NASDAQ",
    "volume": 59394590,
    "avgVolume": 96260890,
    "open": 176.4,
    "previousClose": 179.24,
    "eps": 3.91,
    "pe": 45.2,
    "earningsAnnouncement": "2024-07-17T10:59:00.000+0000",
    "sharesOutstanding": 3189200000,
    "timestamp": 1716926400
  }
]
```

# TODO

[*] Make a function that works with v4 api requests
[ ]  Implement support for query endpoints that do not take two arguments


