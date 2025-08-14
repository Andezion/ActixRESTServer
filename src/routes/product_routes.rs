use actix_web::web;
use crate::handlers::product_handler::{add_product, show_products};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_products).service(add_product);
}
