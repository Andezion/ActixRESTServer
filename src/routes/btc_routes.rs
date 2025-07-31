use actix_web::web;
use crate::handlers::btc_handler::get_btc_price;
/*
    Тут всё просто, говорим о том что у нас есть такая дорога как get_btc_price
 */
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_btc_price);
}
