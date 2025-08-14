use crate::models::product::Product;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use crate::models::auth::Claims;
use crate::models::role::Roles;
use crate::state::AppState;
use crate::utils::jwt::validate_token;

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

#[post("/add")]
async fn add_product(
    req: HttpRequest,
    state: web::Data<AppState>,
    json: web::Json<Product>
) -> impl Responder {
    let auth_header = match req.headers().get("Authorization") {
        Some(h) => h.to_str().unwrap_or(""),
        None => return HttpResponse::Unauthorized().body("No authorization header"),
    };

    let token = auth_header.strip_prefix("Bearer ").unwrap_or("");
    let claims: Claims = match validate_token(token, &state.jwt_secret) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid or expired token"),
    };

    // 2. Проверка роли
    match claims.role {
        Roles::Admin | Roles::Manager => {
            let mut products = state.products.lock().unwrap();
            products.push(json.into_inner());
            HttpResponse::Ok().body("Product added")
        }
        _ => HttpResponse::Forbidden().body("You are not allowed to add products"),
    }
}