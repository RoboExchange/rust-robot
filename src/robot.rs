use log::{debug, info};

use crate::exchange::bybit::Market;
use crate::exchange::general::MarketApi;
use crate::exchange::structs::{Order, OrderSide, OrderType, TimeInForce};

pub fn trade(symbol: String, side: OrderSide, price: f64, take_profit: f64, stop_loss: f64, leverage: i32) {
    let coin = String::from("USDT");
    let available_balance = Market::wallet_available_balance(coin);
    let is_in_position = Market::is_in_position(&symbol);
    info!("Available balance USDT:{}",available_balance);

    if available_balance > 10.0 && !is_in_position {
        info!("Switch to Isolated");
        let isolated_changed = Market::switch_isolated(&symbol, true, leverage);

        info!("Change Leverage");
        let leverage_changed = Market::leverage(&symbol, leverage);

        if isolated_changed && leverage_changed {
            let base = available_balance * leverage as f64 / price;
            let qty = format!("{:.4}", base).parse::<f64>().unwrap();
            info!("Order size:{}", qty);

            let order = Order {
                symbol: symbol.to_string(),
                time_in_force: TimeInForce::GoodTillCancel,
                price: 0.0,
                qty,
                reduce_only: Some(false),
                close_on_trigger: Some(false),
                order_type: OrderType::Market,
                leverage: Some(20),
                side,
                take_profit: None,
                stop_loss: None,
            };

            info!("Send order symbol:{} tpp:{} slp:{}",&symbol,&take_profit,&stop_loss);
            if Market::order(order) {
                info!("Get position information symbol:{}",&symbol);
                let pi = Market::position(&symbol).unwrap();
                if &pi.entry_price > &0.0 {
                    let size = Option::Some(pi.size);

                    info!("Set stop loss symbol:{} side:{}",&symbol,&side);
                    Market::stop_loss(&symbol, size, &side, Option::None, Option::Some(stop_loss));

                    info!("Set take profit symbol:{} qty:{}",&symbol,size.unwrap());
                    Market::take_profit(&symbol, size, &side, Option::Some(take_profit), Option::None);
                }
            } else {
                debug!("Market Order not completed")
            }
        } else {
            debug!("Switch isolated or change leverage not completed")
        }
    }
}