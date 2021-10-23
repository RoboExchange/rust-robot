use std::thread;

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use log::info;
use serde::Deserialize;
use simplelog::*;

use crate::env::{get_access_id, get_base_url, get_initial_balance, get_leverage, get_position_type, get_secret_key, get_tpp, is_test};

mod utils;
mod coinex;
mod robot;
mod env;
mod kucoin;

#[derive(Deserialize)]
pub struct Signal {
    #[serde(default)]
    pub exchange: Option<String>,
    pub symbol: String,
    pub operation: String,
}

#[get("/")]
async fn signal_handler(signal: web::Query<Signal>) -> impl Responder {
    // let exchange: &String = signal.exchange.as_ref().unwrap();

    let mut exchange: String = String::from("NULL");
    if signal.exchange.is_some() {
        exchange = signal.exchange.as_ref().unwrap().to_string();
    };

    let mut operation: String = signal.operation.parse().unwrap();
    let symbol: String = signal.symbol.parse().unwrap();

    let sym_len = symbol.len();
    let perp_len = "PERP".len();
    let market: String = symbol.chars().skip(0).take(sym_len - perp_len).collect();

    let msg = format!("Receive signal exchange:{} symbol:{} operation:{}", exchange, symbol, &operation);
    info!("{}", msg);

    if !is_test() {
        thread::spawn(move || {
            robot::execute_coinex(&market, &operation);
        });
    }

    HttpResponse::Ok().body(msg)
}

// #[actix_web::main]
fn main() {
    init();
    print_env();

    let side = 1;
    let price = 4004.0;

    let result = kucoin::PositionRequest::new("XBTUSDM").send();

    /*HttpServer::new(|| {
        App::new()
            .service(signal_handler)
    })
        .bind("0.0.0.0:2525")?
        .run()
        .await*/
}

fn print_env() {
    info!("             BASE_URL:{}", get_base_url());
    info!("            ACCESS_ID:{}", get_access_id());
    info!("           SECRET_KEY:{}", get_secret_key());
    info!("        POSITION_TYPE:{}", get_position_type());
    info!("                  TPP:{}", get_tpp());
    info!("      INITIAL_BALANCE:{}", get_initial_balance());
    info!("  CONCURRENT_POSITION:{}", get_position_type());
    info!("             LEVERAGE:{}", get_leverage());
    info!("            TEST_MODE:{}", is_test());
}

fn init() {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Stdout, ColorChoice::Always).unwrap();
}