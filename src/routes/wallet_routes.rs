use actix_web::{web}; // импортируем интернет вещи

use crate::handlers::wallet_handler::get_address;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_address);
}