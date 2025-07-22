use actix_web::web; // нужно для конфигурации
use crate::handlers::auth::{login}; // чтобы уметь делать /login
use crate::handlers::user::me;

/*
    Регистрируем маршруты новой развилки
 */
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(login); // добавляем новую дорогу
    cfg.service(me); // чтобы можно было вытянуть имя пользователя
}
