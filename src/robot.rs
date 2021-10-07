use log::{error, info, warn};

#[allow(dead_code)]
use crate::coinex::{AdjustLeverage, PendingPositionRequest, PutLimitRequest, TickerRequest};
use crate::coinex::{CloseLimitRequest, OrderStatusRequest};
use crate::env::get_concurrent_position;

pub fn execute(market: &str, side: &i8) {
    info!("Execute Signal: market:{} side:{}", market, side);
    let mut pp_resp = PendingPositionRequest::new_with_market(Option::from(market)).send();

    match pp_resp {
        Ok(resp) => {
            if resp.data.is_array() && !resp.data.as_array().unwrap().is_empty() {
                println!("Previous position not closed yet market:{}", &market);
            } else {
                pp_resp = PendingPositionRequest::new().send();
                match pp_resp {
                    Ok(resp) => {
                        let concurrent_position = get_concurrent_position();
                        let arr_len = resp.data.as_array().unwrap().len();
                        if arr_len < concurrent_position as usize {
                            let adj_lev_resp = AdjustLeverage::new(&market).send().unwrap();
                            info!("Leverage -> {} leverage:{} position_type:{}", &market, adj_lev_resp.data["leverage"].as_str().unwrap(), adj_lev_resp.data["position_type"]);
                            let ticker_resp = TickerRequest::new(&market).send().unwrap();
                            let last_price = &ticker_resp.data["ticker"]["last"].as_str().unwrap().parse::<f32>().unwrap();
                            let put_limit_resp = PutLimitRequest::new(&market, &side, &last_price).send().unwrap();
                            let order_id = put_limit_resp.data["order_id"].as_f64().unwrap();
                            if put_limit_resp.code.eq(&0) {
                                let current_pos = PendingPositionRequest::new_with_market(Option::from(market)).send().unwrap();
                                let position_id = current_pos.data.get(0).unwrap()["position_id"].as_f64().unwrap();

                                let order_status = OrderStatusRequest::new(&market, &order_id).send().unwrap();
                                let status = order_status.data["status"].as_str().unwrap();
                                let amount = order_status.data["amount"].as_str().unwrap().parse::<f32>().unwrap();
                                let price = order_status.data["price"].as_str().unwrap().parse::<f32>().unwrap();
                                info!("Enter Position -> market:{} id:{} positionId:{} status:{} amount:{} price:{} {}/{}", &market, &order_id, &position_id, status, amount, price, arr_len + 1, concurrent_position);

                                let take_profit_resp = CloseLimitRequest::new(&market, &side, &position_id, &last_price, &amount).send().unwrap();
                                if take_profit_resp.code.eq(&0) {
                                    info!("TakeProfit -> market:{} id:{} amount:{} price:{}", &market, &order_id, amount, price);
                                } else {
                                    warn!("TakeProfit market:{} code:{} message:{}", &market, take_profit_resp.code, take_profit_resp.message);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(err) => { error!("Error: {}", err); }
    }
}
