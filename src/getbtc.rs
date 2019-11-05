
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
extern crate chrono;

use std::time::{UNIX_EPOCH, Duration};
use reqwest::Error;
use serde::{Deserialize};
use chrono::prelude::*;


#[derive(Deserialize, Debug)]
struct Ticker {
    success: String,
    values: Value,
}

#[derive(Deserialize, Debug)]
struct Value {
    price: f64,
    priceCny: f64,
    priceUSD: f64,
    resistence: f64,
    resistenceCny: f64,
    resistenceUSD: f64,
    status: u32,
    support: f64,
    supportCny: f64,
    supportUSD: f64,
    timestamp: u32,
    vol24h: f64,
}

    pub fn getbtc_message() -> String {
    let request_url = format!("https://tokenview.com/v9api/getResistenceAndSupportValue?coin={coin}&exchange=huobipro&stgy=001",
                              coin = "btc");

    let mut response = reqwest::get(&request_url).unwrap();

    let ticker: Ticker = response.json().unwrap();

    let d = UNIX_EPOCH + Duration::from_secs(ticker.values.timestamp.into());
    let datetime = DateTime::<Utc>::from(d);
    let fixed_dt = datetime.with_timezone(&FixedOffset::east(8*3600));

    let timestamp_str = fixed_dt.format("%Y-%m-%d %H:%M:%S").to_string();

    let eth_status_str: &str = match ticker.values.status {
        0 => "Sell BTC",
        1 => "Buy BTC",
        _ => "Unknown",
    };

    let eth_message_str: String = format!("BTC Tracking: suggest datetime: {}, {}, suggest price: {}",
                                timestamp_str, 
                                eth_status_str, 
                                ticker.values.priceUSD);
    return eth_message_str;
    }

    pub fn getbtc_ts() -> u32 {
        let request_url = format!("https://tokenview.com/v9api/getResistenceAndSupportValue?coin={coin}&exchange=huobipro&stgy=001",
                              coin = "btc");
        let mut response = reqwest::get(&request_url).unwrap();
        let ticker: Ticker = response.json().unwrap();
        let d = ticker.values.timestamp;
        return d;
    }