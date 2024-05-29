# Rust Stock API
A stock api written in rust using Financial Modeling Prep



Example using clap for v3 endpoint requests-

```
$ cargo run -- quote-order TSLA

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

#TODO

make a function that works with v4 api requests
