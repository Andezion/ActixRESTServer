pub mod users;
pub mod auth;
mod wallet_routes;

use actix_web::web;

/*
  Тут мы говорим какие вообще дороги у нас есть,
  и вносим все наши дороги в основной список. Удобно для\
  расширения в будущем.
 */
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users") // дорога к пользователям
        .configure(users::config));
    cfg.service(web::scope("/auth") // дорога к аутентификации
        .configure(auth::configure));
    cfg.service(web::scope("/wallet") // дорога к деньгам
        .configure(wallet_routes::config));
}
