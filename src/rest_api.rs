use std::thread;

use actix_web::{get, HttpResponse, Responder, web};
use log::info;
use serde::Deserialize;

use crate::exchange::structs::OrderSide;
use crate::robot;

#[derive(Deserialize)]
pub struct Signal {
    #[serde(default)]
    pub exchange: Option<String>,
    pub symbol: String,
    pub operation: String,
    pub price: f64,
    pub take_profit: f64,
    pub stop_loss: f64,
    pub leverage: i32,
}

#[get("/api/signal")]
pub async fn signal_handler(signal: web::Query<Signal>) -> impl Responder {
    // let exchange: &String = signal.exchange.as_ref().unwrap();

    let mut exchange: String = String::from("NULL");
    if signal.exchange.is_some() {
        exchange = signal.exchange.as_ref().unwrap().to_string();
    };

    let side = if signal.operation.to_uppercase().eq("SHORT") { OrderSide::Short } else { OrderSide::Long };

    let symbol: String = get_symbol(&signal);
    let tpp = signal.take_profit;
    let slp = signal.stop_loss;
    let leverage = signal.leverage;
    let price = signal.price;

    let msg = format!("Receive signal exchange:{} symbol:{} operation:{}", exchange, symbol, &side);
    info!("{}", msg);


    thread::spawn(move || {
        robot::trade(symbol, side, price,tpp, slp, leverage);
    });


    HttpResponse::Ok().body(msg)
}

fn get_symbol(signal: &web::Query<Signal>) -> String {
    let sym_len = signal.symbol.len();
    let perp_len = "PERP".len();

    signal.symbol.chars().skip(0).take(sym_len - perp_len).collect()
}