use actix_web::{App, HttpServer}; // самон важное берём
mod routes; // берём наши папки
mod handlers;
mod models;
mod utils;


/*
  Создаём новый сервер, создаём "карту" дорог (Арр) - вносим туда
  все дороги, потом говорим куда смотреть - адресс и порт и
  впихнули ещё ? - для обработки ошибок. Запускаем и оно работает.
 */
#[actix_web::main] // тут наша асинхронность
async fn main() -> std::io::Result<()> {
    println!("server at http://127.0.0.1:8080"); // чтобы понять, что сервер запустился
    HttpServer::new(|| {
        App::new()
            .configure(routes::config) // конфигурируем все маршруты
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
