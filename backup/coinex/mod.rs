extern crate reqwest;

use log::{debug, info, trace};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha256::digest;

use crate::env::{coinbase_access_id, initial_balance, leverage, position_type, coinbase_secret_key, tpp};
use crate::utils;
use crate::utils::{get_target_price, validate_url};

use self::reqwest::header::{CONTENT_TYPE, HeaderMap};

#[derive(Serialize, Deserialize, Debug)]
pub struct AdjustLeverage {
    #[serde(rename = "market")]
    pub _market: String,
    #[serde(rename = "leverage")]
    pub _leverage: i32,
    #[serde(rename = "position_type")]
    pub _position_type: i8,
    #[serde(rename = "timestamp")]
    pub _timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PutLimitRequest {
    #[serde(rename = "market")]
    pub _market: String,
    #[serde(rename = "side")]
    pub _side: i8,
    #[serde(rename = "price")]
    pub _price: f32,
    #[serde(rename = "amount")]
    pub _amount: f32,
    #[serde(rename = "timestamp")]
    pub _timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloseLimitRequest {
    #[serde(rename = "market")]
    pub _market: String,
    #[serde(rename = "position_id")]
    pub _position_id: f64,
    #[serde(rename = "price")]
    pub _price: f32,
    #[serde(rename = "amount")]
    pub _amount: f32,
    #[serde(rename = "timestamp")]
    pub _timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerRequest {
    #[serde(rename = "market")]
    pub _market: String,
    #[serde(rename = "timestamp")]
    pub _timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PendingPositionRequest {
    #[serde(rename = "market")]
    pub _market: Option<String>,
    #[serde(rename = "timestamp")]
    pub _timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderStatusRequest {
    #[serde(rename = "market")]
    pub _market: String,
    #[serde(rename = "order_id")]
    pub _order_id: f64,
    #[serde(rename = "timestamp")]
    pub _timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PendingOrderRequest {
    #[serde(rename = "market")]
    pub _market: String,
    #[serde(rename = "side")]
    pub _side: i8,
    #[serde(rename = "offset")]
    pub _offset: i8,
    #[serde(rename = "size")]
    pub _size: i8,
    #[serde(rename = "timestamp")]
    pub _timestamp: i64,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub code: i32,
    pub data: Value,
    pub message: String,
}

//------------- Change Leverage -------------
impl AdjustLeverage {
    #[allow(dead_code)]
    pub fn new(market: &str) -> Self {
        let position_type = position_type();
        return AdjustLeverage {
            _market: market.into(),
            _leverage: leverage(),
            _position_type: position_type,
            _timestamp: utils::get_current_timestamp(),
        };
    }

    #[allow(dead_code)]
    fn get_url_encoded(&self) -> String {
        return serde_urlencoded::to_string(&self).expect("can not serialize");
    }

    #[allow(dead_code)]
    pub fn send(&self) -> Result<ApiResponse, u16> {
        let target_path = String::from("market/adjust_leverage");
        let request_params = self.get_url_encoded();
        return call_api(request_params, target_path, "POST");
        // println!("{:?}", res.text());
    }
}


//------------- Put Limit Request -------------
impl PutLimitRequest {
    #[allow(dead_code)]
    pub fn new(market: &str, operation: &str, price: &f32) -> Self {
        let amount = initial_balance() as f32 * leverage() as f32 / price;
        let target_price = get_target_price(&operation, price, &tpp());
        info!("PutLimit -> market:{}  side:{}  price:{}  amount:{}", &market, &operation, &price, &amount);
        PutLimitRequest {
            _market: market.into(),
            _side: get_side(&operation),
            _price: target_price,
            _amount: amount,
            _timestamp: utils::get_current_timestamp(),
        }
    }

    #[allow(dead_code)]
    fn get_url_encoded(&self) -> String {
        return serde_urlencoded::to_string(&self).expect("can not serialize");
    }

    #[allow(dead_code)]
    pub fn send(&self) -> Result<ApiResponse, u16> {
        let target_path = String::from("order/put_limit");
        let request_params = self.get_url_encoded();
        return call_api(request_params, target_path, "POST");
    }
}

//------------- Close Limit Request -------------
impl CloseLimitRequest {
    #[allow(dead_code)]
    pub fn new(market: &str, side: &str, position_id: &f64, price: &f32, amount: &f32) -> Self {
        let target_price = get_target_price(side, price, &tpp());
        info!("CloseLimit -> market:{}  position_id:{}  price:{}  amount:{}", market, position_id, price, amount);
        CloseLimitRequest {
            _market: market.into(),
            _position_id: *position_id,
            _price: target_price,
            _amount: *amount,
            _timestamp: utils::get_current_timestamp(),
        }
    }

    #[allow(dead_code)]
    fn get_url_encoded(&self) -> String {
        return serde_urlencoded::to_string(&self).expect("can not serialize");
    }

    #[allow(dead_code)]
    pub fn send(&self) -> Result<ApiResponse, u16> {
        let target_path = String::from("order/close_limit");
        let request_params = self.get_url_encoded();
        return call_api(request_params, target_path, "POST");
    }
}

impl TickerRequest {
    #[allow(dead_code)]
    pub fn new(market: &str) -> Self {
        debug!("Ticker market: {} ", market);
        TickerRequest {
            _market: market.into(),
            _timestamp: utils::get_current_timestamp(),
        }
    }

    #[allow(dead_code)]
    fn get_url_encoded(&self) -> String {
        return serde_urlencoded::to_string(&self).expect("can not serialize");
    }

    #[allow(dead_code)]
    pub fn send(&self) -> Result<ApiResponse, u16> {
        let target_path = String::from("market/ticker");
        let request_params = self.get_url_encoded();
        return call_api(request_params, target_path, "GET");
    }
}

impl PendingPositionRequest {
    #[allow(dead_code)]
    pub fn new_with_market(market: Option<&str>) -> Self {
        debug!("Ticker market: {} ", market.unwrap());
        PendingPositionRequest {
            _market: Some(market.unwrap().into()),
            _timestamp: utils::get_current_timestamp(),
        }
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        PendingPositionRequest {
            _market: Default::default(),
            _timestamp: utils::get_current_timestamp(),
        }
    }

    #[allow(dead_code)]
    fn get_url_encoded(&self) -> String {
        return serde_urlencoded::to_string(&self).expect("can not serialize");
    }

    #[allow(dead_code)]
    pub fn send(&self) -> Result<ApiResponse, u16> {
        let target_path = String::from("order/pending");
        let request_params = self.get_url_encoded();
        return call_api(request_params, target_path, "GET");
    }
}


impl PendingOrderRequest {
    #[allow(dead_code)]
    pub fn new() -> Self {
        // debug!("Ticker market: {} ", market.unwrap());
        PendingOrderRequest {
            _market: "BTCUSDT".to_string(),
            _side: 0,
            _offset: 0,
            _size: 10,
            _timestamp: utils::get_current_timestamp(),
        }
    }

    #[allow(dead_code)]
    fn get_url_encoded(&self) -> String {
        return serde_urlencoded::to_string(&self).expect("can not serialize");
    }

    #[allow(dead_code)]
    pub fn send(&self) -> Result<ApiResponse, u16> {
        let target_path = String::from("order/pending");
        let request_params = self.get_url_encoded();
        return call_api(request_params, target_path, "GET");
    }
}

impl OrderStatusRequest {
    #[allow(dead_code)]
    pub fn new(market: &str, order_id: &f64) -> Self {
        debug!("Ticker market: {} ", market);
        OrderStatusRequest {
            _market: market.into(),
            _order_id: *order_id,
            _timestamp: utils::get_current_timestamp(),
        }
    }

    #[allow(dead_code)]
    fn get_url_encoded(&self) -> String {
        return serde_urlencoded::to_string(&self).expect("can not serialize");
    }

    #[allow(dead_code)]
    pub fn send(&self) -> Result<ApiResponse, u16> {
        let target_path = String::from("order/status");
        let request_params = self.get_url_encoded();
        return call_api(request_params, target_path, "GET");
    }
}


fn call_api(params_url_encoded: String, target_path: String, method: &str) -> Result<ApiResponse, u16> {
    let full_params = format!("{}&secret_key={}", params_url_encoded, coinbase_secret_key());

    trace!("Clear: {}", &full_params);
    let sign = digest(&full_params).to_lowercase();
    trace!("Sign: {}", &sign);

    let url = validate_url(&target_path);

    trace!("URL: {}", url);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
    headers.insert("AccessId", coinbase_access_id().parse().unwrap());
    headers.insert("Authorization", sign.parse().unwrap());

    let http_client = reqwest::blocking::Client::builder().use_rustls_tls().build().unwrap();

    return if method == "POST" {
        let resp = http_client.post(&url)
            .headers(headers)
            .body(params_url_encoded)
            .send();
        if resp.is_ok() {
            Ok(resp.unwrap().json().unwrap())
        } else {
            Err(resp.unwrap().status().as_u16())
        }
    } else {
        let url = format!("{}?{}", url, params_url_encoded);
        let resp = http_client.get(url)
            .headers(headers)
            .send();
        if resp.is_ok() {
            Ok(resp.unwrap().json().unwrap())
        } else {
            Err(resp.unwrap().status().as_u16())
        }
    };
}

fn get_side(operation: &str) -> i8 {
    let mut side: i8 = 1;
    if operation.eq("LONG") {
        side = 2;
    }
    return side;
}