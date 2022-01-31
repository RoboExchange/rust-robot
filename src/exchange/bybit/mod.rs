use std::collections::HashMap;
use std::string::String;

use reqwest::header::{CONTENT_TYPE, HeaderMap};
use ring::hmac;
use rustc_serialize::hex::ToHex;
use serde_json::Value;

use crate::common::environments::bybit_api_secret;
use crate::common::utils::validate_url;
use crate::exchange::bybit::market_structs::{ApiResponse, LeverageRequest, OrderRequest, PositionRequest, SwitchIsolatedRequest, TradingStop, WalletInformation};
use crate::exchange::general::MarketApi;
use crate::exchange::structs::*;
use crate::exchange::structs::HttpMethod::POST;

mod market_structs;

pub struct Market;

const ORDER_PATH: &str = "private/linear/order/create";
const POSITION_LIST_PATH: &str = "private/linear/position/list";
const WALLET_BALANCE_PATH: &str = "v2/private/wallet/balance";
const SWITCH_ISOLATED_PATH: &str = "private/linear/position/switch-isolated";
const LEVERAGE_PATH: &str = "private/linear/position/set-leverage";
const TRADING_STOP_PATH: &str = "private/linear/position/trading-stop";

impl MarketApi for Market {
    fn order(_order: Order) -> bool {
        let order_request = OrderRequest::new(_order);
        let query_params = order_request.get_query_map();
        let response = call_api(query_params, ORDER_PATH, HttpMethod::POST);
        if response.ret_code != 0 {
            println!("Error: {}:{}", response.ret_code, response.ret_msg);
        }

        response.ret_code == 0
    }

    fn take_profit(symbol: &String, qty: Option<f64>, _side: &OrderSide, take_profit: Option<f64>, _stop_loss: Option<f64>) -> bool {
        let take_profit_limit = Order {
            symbol: symbol.to_string(),
            time_in_force: TimeInForce::PostOnly,
            price: take_profit.unwrap(),
            qty: qty.unwrap(),
            reduce_only: Some(true),
            close_on_trigger: Some(true),
            order_type: OrderType::Limit,
            leverage: Option::None,
            side: OrderSide::Short,
            take_profit: None,
            stop_loss: None,
        };
        Market::order(take_profit_limit)
    }


    fn stop_loss(symbol: &String, _qty: Option<f64>, side: &OrderSide, take_profit: Option<f64>, stop_loss: Option<f64>) -> bool {
        let trading_stop = TradingStop::new(symbol, side, take_profit, stop_loss);
        let query_params = trading_stop.get_query_map();
        let response = call_api(query_params, TRADING_STOP_PATH, POST);
        if response.ret_code != 0 {
            println!("Error: {}:{}", response.ret_code, response.ret_msg);
        }
        response.ret_code == 0
    }

    fn position(symbol: &String) -> Option<PositionInformation> {
        let pr = PositionRequest::new(symbol);
        let query_params = pr.get_query_map();
        let response = call_api(query_params, POSITION_LIST_PATH, HttpMethod::GET);
        let pi = if response.ret_code == 0 {
            let value = response.result.get(0).unwrap();
            Option::Some(PositionInformation::from_value(value))
        } else {
            Option::None
        };
        return pi;
    }

    fn is_in_position(symbol: &String) -> bool {
        let option_position_info = Market::position(symbol);
        return if option_position_info.is_none() {
            false
        } else {
            if option_position_info.unwrap().entry_price > 0.0 {
                true
            } else {
                false
            }
        };
    }

    fn wallet_available_balance(coin: String) -> f64 {
        let wi = WalletInformation::new(&coin);
        let query_params = wi.get_query_map();
        let response = call_api(query_params, WALLET_BALANCE_PATH, HttpMethod::GET);
        return if response.ret_code == 0 {
            response.result.get(&coin).unwrap().get("available_balance").unwrap().as_f64().unwrap()
        } else {
            println!("Error: {}:{}", response.ret_code, response.ret_msg);
            0.0
        };
    }

    fn leverage(symbol: &String, leverage: i32) -> bool {
        let pr = LeverageRequest::new(&symbol.to_string(), leverage, leverage);
        let query_params = pr.get_query_map();
        let response = call_api(query_params, LEVERAGE_PATH, HttpMethod::POST);
        response.ret_code == 0 || response.ret_code == 34036
    }

    fn switch_isolated(symbol: &String, isolated: bool, leverage: i32) -> bool {
        let pr = SwitchIsolatedRequest::new(&symbol.to_string(), isolated, leverage, leverage);
        let query_params = pr.get_query_map();
        let response = call_api(query_params, SWITCH_ISOLATED_PATH, HttpMethod::POST);
        response.ret_code == 0 || response.ret_code == 130056
    }
}

fn call_api(query_params: HashMap<String, Value>, api_path: &str, method: HttpMethod) -> ApiResponse {
    let mut map = query_params.clone();

    // Create sign key
    let key = hmac::Key::new(hmac::HMAC_SHA256, bybit_api_secret().as_bytes());

    //sort query parameters
    let mut query = sort_query_parameters(&query_params);

    //sign query
    let sign_query = hmac::sign(&key, query.as_ref());

    // add sign to query
    map.insert(String::from("sign"), Value::from(sign_query.as_ref().to_hex()));

    //sort again
    query = sort_query_parameters(&map);

    //create url
    let url = match method {
        HttpMethod::GET => format!("{}?{}", validate_url(api_path), query),
        HttpMethod::DELETE => format!("{}?{}", validate_url(api_path), query),
        HttpMethod::POST => format!("{}?{}", validate_url(api_path), query),
        HttpMethod::PUT => validate_url(api_path)
    };

    // Set Headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "Content-Type: application/json".parse().unwrap());

    // Send request
    let client = reqwest::blocking::Client::builder().use_rustls_tls().build().unwrap();

    let resp = match method {
        HttpMethod::GET => client.get(url).headers(headers).send(),
        HttpMethod::POST => client.post(url).send(),
        HttpMethod::DELETE => client.delete(url).send(),
        HttpMethod::PUT => client.put(url).send(),
    };
    resp.unwrap().json::<ApiResponse>().unwrap()
}

fn sort_query_parameters(query_map: &HashMap<String, Value>) -> String {
    let mut key_list = Vec::new();

    for item in query_map {
        key_list.push(item.0);
    }
    key_list.sort();

    let mut query = String::new();
    for key in key_list {
        let value = &query_map.get(key).unwrap();
        let p = if value.is_string() {
            format!("{}={}&", key, value.as_str().unwrap())
        } else if value.is_boolean() {
            format!("{}={}&", key, value.as_bool().unwrap())
        } else if value.is_f64() {
            format!("{}={}&", key, value.as_f64().unwrap())
        } else {
            format!("{}={}&", key, value.as_i64().unwrap())
        };
        query.push_str(p.as_str());
    }

    //remove last '&' character
    query.pop();
    query
}
