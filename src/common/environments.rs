#![allow(unused)]

use std::env;
use log::error;

pub fn base_url() -> String {
    let e = env::var("BASE_URL");
    if !e.is_ok() {
        error!("BASE_URL is empty");
    }
    return e.unwrap();
}

pub fn initial_balance() -> i32 {
    let e = env::var("INITIAL_BALANCE");
    if !e.is_ok() {
        error!("INITIAL_BALANCE is empty");
        return 10;
    }
    return e.unwrap().parse::<i32>().unwrap();
}

pub fn concurrent_position() -> i32 {
    let e = env::var("CONCURRENT_POSITION");
    if !e.is_ok() {
        error!("CONCURRENT_POSITION is empty");
        return 1;
    }
    return e.unwrap().parse::<i32>().unwrap();
}

/* Coinbase ENV */
pub fn coinbase_access_id() -> String {
    let e = env::var("COINBASE_ACCESS_ID");
    if !e.is_ok() {
        error!("COINBASE_ACCESS_ID is empty");
        return String::from("");
    }
    return e.unwrap();
}

pub fn coinbase_secret_key() -> String {
    let e = env::var("COINBASE_SECRET_KEY");
    if !e.is_ok() {
        error!("COINBASE_SECRET_KEY is empty");
        return String::from("");
    }
    return e.unwrap();
}

/* Kucoin ENV */
pub fn kucoin_api_key() -> String {
    let e = env::var("KUCOIN_API_KEY");
    if !e.is_ok() {
        error!("KUCOIN_API_KEY is empty");
        return String::from("");
    }
    return e.unwrap();
}

pub fn kucoin_api_secret() -> String {
    let e = env::var("KUCOIN_API_SECRET");
    if !e.is_ok() {
        error!("KUCOIN_API_SECRET is empty");
        return String::from("");
    }
    return e.unwrap();
}

pub fn kucoin_api_passphrase() -> String {
    let e = env::var("KUCOIN_API_PASSPHRASE");
    if !e.is_ok() {
        error!("KUCOIN_API_PASSPHRASE is empty");
        return String::from("");
    }
    return e.unwrap();
}

/* Bybit ENV */
pub fn bybit_api_key() -> String {
    let e = env::var("BYBIT_API_KEY");
    if !e.is_ok() {
        error!("BYBIT_API_KEY is empty");
        return String::from("");
    }
    return e.unwrap();
}

pub fn bybit_api_secret() -> String {
    let e = env::var("BYBIT_API_SECRET");
    if !e.is_ok() {
        error!("BYBIT_API_SECRET is empty");
        return String::from("");
    }
    return e.unwrap();
}
