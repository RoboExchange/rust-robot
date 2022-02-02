use std::str;
use std::thread;

use actix_web::*;
use actix_web::http::HeaderValue;
use log::info;
use serde::Deserialize;
use serde_json::Value;

use crate::exchange::structs::OrderSide;
use crate::robot;

#[derive(Deserialize)]
pub struct Signal {
    #[serde(default)]
    pub symbol: String,
    pub operation: String,
    pub price: f64,
    pub take_profit: f64,
    pub stop_loss: f64,
    pub leverage: i32,
}

#[get("/api/signal")]
pub async fn signal_handler(request: HttpRequest) -> impl Responder {
    let signal = web::Query::<Signal>::from_query(request.query_string()).unwrap();

    let header: Option<&HeaderValue> = request.headers().get("METADATA");

    if header == None {
        return HttpResponse::NoContent().body("Please send metadata as header.");
    } else {
        let metadata = extract_metadata(header);


        let side = if signal.operation.to_uppercase().eq("SHORT") { OrderSide::Short } else { OrderSide::Long };

        let symbol: String = get_symbol(&signal);
        let price = signal.price;
        let tpp = signal.take_profit;
        let slp = signal.stop_loss;
        let leverage = signal.leverage;

        let msg = format!("Receive signal symbol:{} side:{} price:{} tpp:{} slp:{}", symbol, &side, price, tpp, slp);
        info!("{}", msg);

        thread::spawn(move || {
            robot::trade(symbol, side, price, tpp, slp, leverage, metadata);
        });
        HttpResponse::Ok().body(msg)
    }
}

fn extract_metadata(header: Option<&HeaderValue>) -> Value {
    let metadata_header = base64::decode(header.unwrap());
    let json = String::from_utf8_lossy(metadata_header.unwrap().as_slice()).to_string();
    let metadata: serde_json::error::Result<Value> = serde_json::from_str(json.as_str());
    metadata.unwrap()
}

fn get_symbol(signal: &web::Query<Signal>) -> String {
    let sym_len = signal.symbol.len();
    let perp_len = "PERP".len();

    signal.symbol.chars().skip(0).take(sym_len - perp_len).collect()
}