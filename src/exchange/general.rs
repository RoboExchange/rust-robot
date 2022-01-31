use crate::exchange::structs::{OrderSide, PositionInformation};

use super::structs::Order;

pub trait MarketApi {
    fn order(order: Order) -> bool;
    fn take_profit(symbol: &String, qty: Option<f64>, side: &OrderSide, take_profit: Option<f64>, stop_loss: Option<f64>) -> bool;
    fn stop_loss(symbol: &String, qty: Option<f64>, side: &OrderSide, take_profit: Option<f64>, stop_loss: Option<f64>) -> bool;
    fn position(symbol: &String) -> Option<PositionInformation>;
    fn is_in_position(symbol: &String) -> bool;
    fn wallet_available_balance(coin: String) -> f64;
    fn leverage(symbol: &String, leverage: i32) -> bool;
    fn switch_isolated(symbol: &String, isolated: bool, leverage: i32) -> bool;
}



