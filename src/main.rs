use actix_web::{web, App, HttpServer};
use crate::state::AppState;
use crate::models::wallet::create_wallet;
use std::sync::{Arc, Mutex};
use bdk::blockchain::{ElectrumBlockchain};
use bdk::electrum_client::Client;

// самое важное берём
mod routes; // берём наши папки
mod handlers;
mod models;
mod utils;
mod state;
/*
  Создаём новый сервер, создаём "карту" дорог (Арр) - вносим туда
  все дороги, потом говорим куда смотреть - адресс и порт и
  впихнули ещё ? - для обработки ошибок. Запускаем и оно работает.
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server at http://127.0.0.1:8081");

    let wallet = create_wallet();

    let electrum_url = "ssl://electrum.blockstream.info:60002";

    let client = Client::new(electrum_url).expect("Failed to connect Electrum client");
    let blockchain = ElectrumBlockchain::from(client);

    let app_state = AppState {
        jwt_secret: "super_secret_key".to_string(), // временно хардкодим
        wallet: Arc::new(Mutex::new(wallet)),
        blockchain: Arc::new(Mutex::new(blockchain)),
        products: Arc::new(Mutex::new(vec![])),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone())) // передаём глобальное состояние
            .configure(routes::config) // все маршруты настраиваются тут
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
