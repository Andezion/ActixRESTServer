use crate::models::product::Product;
use actix_web::{post, HttpResponse};

#[post("/show")]
pub async fn show_products() -> HttpResponse {
    let products = vec![
        Product { name: "Smth1".to_string(), counter: 10 },
        Product { name: "Smth2".to_string(), counter: 5 },
        Product { name: "Smth3".to_string(), counter: 7 },
        Product { name: "Smth4".to_string(), counter: 12 },
    ];

    HttpResponse::Ok().json(products)
}