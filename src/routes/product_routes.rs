use actix_web::web;
use crate::handlers::product_handler::show_products;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_products);
}
