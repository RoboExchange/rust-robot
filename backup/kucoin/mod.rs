extern crate data_encoding;
extern crate hex;
extern crate reqwest;
extern crate ring;
extern crate rustc_serialize as serialize;

use data_encoding::BASE64;
use log::info;
use ring::hmac;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::env::{initial_balance, leverage, tpp, kucoin_api_key, kucoin_api_passphrase, kucoin_api_secret};
use crate::utils::{get_current_timestamp, get_target_price, validate_url};

use self::reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderRequest {
    #[serde(rename = "symbol")]
    pub _symbol: String,
    #[serde(rename = "side")]
    pub _side: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "leverage")]
    pub _leverage: i32,
    #[serde(rename = "price")]
    pub _price: f32,
    #[serde(rename = "size")]
    pub _size: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionRequest {
    #[serde(rename = "symbol")]
    pub _symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractRequest {
    #[serde(rename = "symbol")]
    pub _symbol: String,
}




#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub code: String,
    pub data: Value,
}

//------------- Put Limit Request -------------
impl OrderRequest {
    #[allow(dead_code)]
    pub fn new(symbol: &str, operation: &str, price: &f32) -> Self {
        let size = initial_balance() as f32 * leverage() as f32 / price;
        let target_price = get_target_price(&operation, price, &tpp());
        info!("PutLimit -> market:{}  side:{}  price:{}  amount:{}", &symbol, &operation, &price, &size);
        OrderRequest {
            _symbol: symbol.into(),
            _side: get_side(&operation),
            _type: String::from("limit"),
            _leverage: leverage(),
            _price: target_price,
            _size: size,
        }
    }

    #[allow(dead_code)]
    fn to_json(&self) -> String {
        return serde_json::to_string(&self).expect("can not serialize");
    }

    #[allow(dead_code)]
    pub fn send(&self) {
        let target_path = String::from("api/v1/orders");
        let json = self.to_json();
        call_api(json, target_path, "POST");
    }
}

impl PositionRequest {
    #[allow(dead_code)]
    pub fn new(symbol: &str) -> Self {
        info!("Position symbol:{}", &symbol);
        PositionRequest {
            _symbol: symbol.into(),
        }
    }

    #[allow(dead_code)]
    fn get_url_encoded(&self) -> String {
        return serde_urlencoded::to_string(&self).expect("can not serialize");
    }

    #[allow(dead_code)]
    pub fn send(&self) {
        let target_path = String::from("api/v1/position");
        let query = self.get_url_encoded();
        call_api(query, target_path, "GET");
    }
}

pub fn call_api(query: String, target_path: String, method: &str) {

    //Initial data
    let kc_api_passphrase = kucoin_api_passphrase();
    let timestamp = get_current_timestamp();
    let kc_api_key = kucoin_api_key();
    let kc_api_secret = kucoin_api_secret();

    // Create sign key
    let key = hmac::Key::new(hmac::HMAC_SHA256, kc_api_secret.as_bytes());

    // Sign query
    let string_to_sign = if method == "GET" {
        format!("{}{}/{}?{}", &timestamp, &method, &target_path, &query)
    } else {
        format!("{}{}/{}{}", &timestamp, &method, &target_path, &query)
    };
    info!("Clear: {}", &string_to_sign);
    let query_sign = BASE64.encode(hmac::sign(&key, &string_to_sign.as_ref()).as_ref());
    info!("data sign: {}", query_sign);


    let url = validate_url(&target_path);
    info!("URL: {}", url);

    // Sign passphrase
    let passphrase_sign = BASE64.encode(hmac::sign(&key, &kc_api_passphrase.as_ref()).as_ref());
    info!("passphrase sign: {}", passphrase_sign);

    // Init HeaderValue
    let hv_timestamp: HeaderValue = HeaderValue::from(timestamp);
    let hv_version: HeaderValue = HeaderValue::from(2);
    let hv_sign: HeaderValue = HeaderValue::from_str(query_sign.as_ref()).unwrap();
    let hv_passphrase: HeaderValue = HeaderValue::from_str(passphrase_sign.as_str()).unwrap();
    let hv_api_key: HeaderValue = HeaderValue::from_str(kc_api_key.as_str()).unwrap();

    // Set Headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "Content-Type:application/json".parse().unwrap());
    headers.insert("KC-API-KEY", hv_api_key);
    headers.insert("KC-API-PASSPHRASE", hv_passphrase);
    headers.insert("KC-API-SIGN", hv_sign);
    headers.insert("KC-API-TIMESTAMP", hv_timestamp);
    headers.insert("KC-API-KEY-VERSION", hv_version);

    // Send request
    let http_client = reqwest::blocking::Client::builder().use_rustls_tls().build().unwrap();

    let resp = http_client.post(&url)
        .headers(headers)
        .body(query)
        .send()
        .unwrap();

    println!("{}", resp.text().unwrap());


    // return if method == "POST" {
    //     let resp = http_client.post(&url)
    //         .headers(headers)
    //         .body(query)
    //         .send();
    //     if resp.is_ok() {
    //         Ok(resp.unwrap().json().unwrap())
    //     } else {
    //         Err(resp.unwrap().status().as_u16())
    //     }
    // } else {
    //     let url = format!("{}?{}", url, query);
    //     let resp = http_client.get(url)
    //         .headers(headers)
    //         .send();
    //
    //     if resp.is_ok() {
    //         Ok(resp.unwrap().json().unwrap())
    //     } else {
    //         Err(resp.unwrap().status().as_u16())
    //     }
    // };
}

fn get_side(operation: &str) -> String {
    let mut side = String::from("sell");
    if operation.eq("LONG") {
        side = String::from("buy");
    }
    return side;
}