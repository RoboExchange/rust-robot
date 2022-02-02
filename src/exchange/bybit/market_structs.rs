use std::collections::HashMap;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::common::utils::get_current_timestamp;
use crate::exchange::structs::{Order, OrderSide, OrderType, PositionInformation, TimeInForce};

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderRequest {
    #[serde(rename = "order_type")]
    pub _order_type: String,
    #[serde(rename = "price")]
    pub _price: f64,
    #[serde(rename = "qty")]
    pub _qty: f64,
    #[serde(rename = "symbol")]
    pub _symbol: String,
    #[serde(rename = "side")]
    pub _side: String,
    #[serde(rename = "time_in_force")]
    pub _time_in_force: String,
    #[serde(rename = "take_profit")]
    pub _take_profit: Option<f64>,
    #[serde(rename = "stop_loss")]
    pub _stop_loss: Option<f64>,
    #[serde(rename = "reduce_only")]
    pub _reduce_only: Option<bool>,
    #[serde(rename = "close_on_trigger")]
    pub _close_on_trigger: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradingStop {
    #[serde(rename = "symbol")]
    pub _symbol: String,
    #[serde(rename = "side")]
    pub _side: String,
    #[serde(rename = "take_profit")]
    pub _take_profit: Option<f64>,
    #[serde(rename = "stop_loss")]
    pub _stop_loss: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SwitchIsolatedRequest {
    #[serde(rename = "symbol")]
    pub _symbol: String,
    #[serde(rename = "is_isolated")]
    pub _is_isolated: bool,
    #[serde(rename = "buy_leverage")]
    pub _buy_leverage: i32,
    #[serde(rename = "sell_leverage")]
    pub _sell_leverage: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeverageRequest {
    #[serde(rename = "symbol")]
    pub _symbol: String,
    #[serde(rename = "buy_leverage")]
    pub _buy_leverage: i32,
    #[serde(rename = "sell_leverage")]
    pub _sell_leverage: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionRequest {
    #[serde(rename = "symbol")]
    pub _symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletInformation {
    #[serde(rename = "coin")]
    pub _coin: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractRequest {
    #[serde(rename = "symbol")]
    pub _symbol: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub ext_code: String,
    pub ext_info: String,
    pub result: Value,
}

impl std::fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeInForce::GoodTillCancel => write!(f, "GoodTillCancel"),
            TimeInForce::ImmediateOrCancel => write!(f, "ImmediateOrCancel"),
            TimeInForce::FillOrKill => write!(f, "FillOrKill"),
            TimeInForce::PostOnly => write!(f, "PostOnly"),
        }
    }
}

impl std::fmt::Display for OrderSide {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderSide::Short => write!(f, "Sell"),
            OrderSide::Long => write!(f, "Buy")
        }
    }
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::Market => write!(f, "Market"),
            OrderType::Limit => write!(f, "Limit")
        }
    }
}

impl OrderRequest {
    pub fn new(order: Order) -> Self {
        OrderRequest {
            _order_type: order.order_type.to_string(),
            _price: order.price,
            _qty: order.qty,
            _symbol: order.symbol.to_string(),
            _side: order.side.to_string(),
            _time_in_force: order.time_in_force.to_string(),
            _take_profit: order.take_profit,
            _stop_loss: order.stop_loss,
            _reduce_only: order.reduce_only,
            _close_on_trigger: order.close_on_trigger,
        }
    }

    pub fn get_query_map(&self, api_key: String) -> HashMap<String, Value> {
        //Initial data
        let timestamp = get_current_timestamp();
        let mut query_map = HashMap::<String, Value>::new();
        query_map.insert(String::from("api_key"), Value::from(api_key));
        query_map.insert(String::from("timestamp"), Value::from(timestamp.to_string()));
        query_map.insert(String::from("time_in_force"), Value::from(self._time_in_force.to_string()));
        query_map.insert(String::from("symbol"), Value::from(self._symbol.to_string()));
        query_map.insert(String::from("order_type"), Value::from(self._order_type.to_string()));
        query_map.insert(String::from("side"), Value::from(self._side.to_string()));
        query_map.insert(String::from("qty"), Value::from(self._qty.to_string()));

        if self._order_type.eq("Limit") {
            query_map.insert(String::from("price"), Value::from(self._price.to_string()));
        }
        if self._take_profit.is_some() {
            query_map.insert(String::from("take_profit"), Value::from(self._take_profit.unwrap()));
        }
        if self._stop_loss.is_some() {
            query_map.insert(String::from("stop_loss"), Value::from(self._stop_loss.unwrap()));
        }
        if self._reduce_only.is_some() {
            query_map.insert(String::from("reduce_only"), Value::from(self._reduce_only.unwrap()));
        }
        if self._close_on_trigger.is_some() {
            query_map.insert(String::from("close_on_trigger"), Value::from(self._close_on_trigger.unwrap()));
        }

        return query_map;
    }
}

impl PositionRequest {
    pub fn new(symbol: &String) -> Self {
        PositionRequest { _symbol: symbol.to_string() }
    }

    pub fn get_query_map(&self, api_key: String) -> HashMap<String, Value> {
        //Initial data
        let timestamp = get_current_timestamp();
        let symbol = &self._symbol;
        let mut query_map = HashMap::new();
        query_map.insert(String::from("api_key"), Value::from(api_key));
        query_map.insert(String::from("symbol"), Value::from(symbol.to_string()));
        query_map.insert(String::from("timestamp"), Value::from(timestamp.to_string()));
        return query_map;
    }
}

impl WalletInformation {
    pub fn new(coin: &String) -> Self {
        WalletInformation { _coin: coin.to_string() }
    }

    pub fn get_query_map(&self, api_key: String) -> HashMap<String, Value> {
        //Initial data
        let timestamp = get_current_timestamp();
        let coin = &self._coin;
        let mut query_map = HashMap::new();
        query_map.insert(String::from("api_key"), Value::from(api_key));
        query_map.insert(String::from("coin"), Value::from(coin.to_string()));
        query_map.insert(String::from("timestamp"), Value::from(timestamp.to_string()));
        return query_map;
    }
}

impl SwitchIsolatedRequest {
    pub fn new(symbol: &String, is_isolated: bool, buy_leverage: i32, sell_leverage: i32) -> Self {
        SwitchIsolatedRequest {
            _symbol: symbol.to_string(),
            _is_isolated: is_isolated,
            _buy_leverage: buy_leverage,
            _sell_leverage: sell_leverage,
        }
    }

    pub fn get_query_map(&self, api_key: String) -> HashMap<String, Value> {
        //Initial data
        let timestamp = get_current_timestamp();
        let mut query_map = HashMap::new();
        query_map.insert(String::from("api_key"), Value::from(api_key));
        query_map.insert(String::from("timestamp"), Value::from(timestamp.to_string()));
        query_map.insert(String::from("symbol"), Value::from(self._symbol.to_string()));
        query_map.insert(String::from("is_isolated"), Value::from(self._is_isolated.to_string()));
        query_map.insert(String::from("buy_leverage"), Value::from(self._buy_leverage.to_string()));
        query_map.insert(String::from("sell_leverage"), Value::from(self._sell_leverage.to_string()));
        return query_map;
    }
}

impl LeverageRequest {
    pub fn new(symbol: &String, buy_leverage: i32, sell_leverage: i32) -> Self {
        LeverageRequest {
            _symbol: symbol.to_string(),
            _buy_leverage: buy_leverage,
            _sell_leverage: sell_leverage,
        }
    }

    pub fn get_query_map(&self, api_key: String) -> HashMap<String, Value> {
        //Initial data
        let timestamp = get_current_timestamp();
        let mut query_map = HashMap::new();
        query_map.insert(String::from("api_key"), Value::from(api_key));
        query_map.insert(String::from("timestamp"), Value::from(timestamp.to_string()));
        query_map.insert(String::from("symbol"), Value::from(self._symbol.to_string()));
        query_map.insert(String::from("buy_leverage"), Value::from(self._buy_leverage.to_string()));
        query_map.insert(String::from("sell_leverage"), Value::from(self._sell_leverage.to_string()));
        return query_map;
    }
}

impl PositionInformation {
    pub fn from_value(value: &Value) -> PositionInformation {
        let side = if value["side"].as_str().unwrap().eq("Buy") { OrderSide::Long } else { OrderSide::Short };

        PositionInformation {
            entry_price: value.get("entry_price").unwrap().as_f64().unwrap(),
            free_qty: value.get("free_qty").unwrap().as_f64().unwrap(),
            is_isolated: value.get("is_isolated").unwrap().as_bool().unwrap(),
            leverage: value.get("leverage").unwrap().as_i64().unwrap() as i32,
            liq_price: value.get("liq_price").unwrap().as_f64().unwrap(),
            side,
            size: value.get("size").unwrap().as_f64().unwrap(),
            symbol: value.get("symbol").unwrap().to_string(),
        }
    }
}

impl TradingStop {
    pub fn new(symbol: &String, side: &OrderSide, take_profit: Option<f64>, stop_loss: Option<f64>) -> Self {
        TradingStop {
            _symbol: symbol.to_string(),
            _side: side.to_string(),
            _take_profit: take_profit,
            _stop_loss: stop_loss,
        }
    }

    pub fn get_query_map(&self, api_key: String) -> HashMap<String, Value> {
        //Initial data
        let timestamp = get_current_timestamp();
        let mut query_map = HashMap::new();
        query_map.insert(String::from("api_key"), Value::from(api_key));
        query_map.insert(String::from("timestamp"), Value::from(timestamp.to_string()));

        query_map.insert(String::from("symbol"), Value::from(self._symbol.to_string()));
        query_map.insert(String::from("side"), Value::from(self._side.to_string()));

        if self._take_profit.is_some() {
            query_map.insert(String::from("take_profit"), Value::from(self._take_profit.unwrap()));
        }
        if self._stop_loss.is_some() {
            query_map.insert(String::from("stop_loss"), Value::from(self._stop_loss.unwrap()));
        }
        return query_map;
    }
}