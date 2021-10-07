use std::thread;

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use log::info;
use serde::Deserialize;
use simplelog::*;

use crate::env::{get_access_id, get_base_url, get_initial_balance, get_leverage, get_position_type, get_secret_key, get_tpp};

mod utils;
mod coinex;
mod robot;
mod env;

#[derive(Deserialize)]
pub struct Signal {
    pub symbol: String,
    pub operation: String,
}

#[get("/")]
async fn signal_handler(signal: web::Query<Signal>) -> impl Responder {
    let opr: String = signal.operation.parse().unwrap();
    let sym_len = signal.symbol.len();
    let perp_len = "PERP".len();
    let market: String = signal.symbol.chars().skip(0).take(sym_len - perp_len).collect();

    let mut side: i8 = 1;
    if opr.eq("LONG") {
        side = 2;
    }

    thread::spawn(move || {
        robot::execute(&market, &side);
    });
    HttpResponse::Ok().body(format!("Receive signal symbol:{} operation:{}", signal.symbol, signal.operation))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init();
    print_env();

    HttpServer::new(|| {
        App::new()
            .service(signal_handler)
    })
        .bind("0.0.0.0:2525")?
        .run()
        .await
}

fn print_env() {
    info!("BASE_URL: {}", get_base_url());
    info!("ACCESS_ID: {}", get_access_id());
    info!("SECRET_KEY: {}", get_secret_key());
    info!("POSITION_TYPE: {}", get_position_type());
    info!("TPP: {}", get_tpp());
    info!("INITIAL_BALANCE: {}", get_initial_balance());
    info!("CONCURRENT_POSITION: {}", get_position_type());
    info!("LEVERAGE: {}", get_leverage());
}

fn init() {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Stdout, ColorChoice::Always).unwrap();
}