#![allow(unused)]

use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

pub enum HttpMethod {
    POST,
    GET,
    DELETE,
    PUT,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum OrderSide {
    Short,
    Long,
}

pub enum OrderType {
    Market,
    Limit,
}

pub enum TimeInForce {
    GoodTillCancel,
    ImmediateOrCancel,
    FillOrKill,
    PostOnly,
}

pub struct Order {
    pub symbol: String,
    pub time_in_force: TimeInForce,
    pub price: f64,
    pub qty: f64,
    pub reduce_only: Option<bool>,
    pub close_on_trigger: Option<bool>,
    pub order_type: OrderType,
    pub leverage: Option<i32>,
    pub side: OrderSide,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionInformation {
    pub entry_price: f64,
    pub free_qty: f64,
    pub is_isolated: bool,
    pub leverage: i32,
    pub liq_price: f64,
    pub side: OrderSide,
    pub size: f64,
    pub symbol: String,
}
