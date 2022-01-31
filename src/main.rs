use std::io;

use actix_web::{App, HttpServer};
use log::info;
use simplelog::{Config, LevelFilter, SimpleLogger};

mod exchange;
mod common;
mod robot;
mod rest_api;

#[actix_web::main]
async fn main() -> io::Result<()> {
    info!("Start trader robot");

    init();
    start_http_server().await
}

async fn start_http_server() -> io::Result<()> {
    info!("Start http server");
    HttpServer::new(|| {
        App::new()
            .service(rest_api::signal_handler)
    })
        .bind("0.0.0.0:2525")?
        .run()
        .await
}

fn init() {
    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();
    // TermLogger::init(LevelFilter::Warn, Config::default(), TerminalMode::Stdout, ColorChoice::Always).unwrap();
}