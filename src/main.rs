use actix_web::{web, App, HttpServer};
use crate::state::AppState;

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

    // Твоя "глобальная переменная" — секретный ключ для JWT
    let app_state = AppState {
        jwt_secret: "super_secret_key".to_string(), // временно хардкодим
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
