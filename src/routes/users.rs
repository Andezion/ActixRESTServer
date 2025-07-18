use actix_web::{get, post, web, Responder}; // наши марштуры, веб для обёрток и чтобы возвращать http
use crate::handlers::user_handler::{greet_user, register_user}; // наши функции
use crate::models::user::RegisterRequest; // структура

/*
  Условно это наша карта, у нас есть 2 варианта куда можно пойти.
  Нам это нужно потому что Actix требует чтобы все маршруты были зарегестрированы заранее!

 */
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(greet)
        .service(register);
}


/*
  Обработка нашего get запроса, внутрь пихаем параметры url, типа имени,
  и все параметры у нас становятся ключ-значение, дальше если у нас есть
  нужный параметр - то достаём его, а cloned() нужен чтобы дальше передать в
  greet_user() Option<String>
 */
#[get("/hello")] // если отправляем hello - работает эта функция
async fn greet(query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    greet_user(query.get("name").cloned()).await
}


/*
  Тут же у нас обработка post запроса, внутрь пихаем тело запроса нашего(структуру),
  и потом переделываем то, что вошло на RegisterRequest. А возрващаемый тип в обоих случаях
  это любой объект? который можно переделать потом в http ответ
 */
#[post("/register")]
async fn register(json: web::Json<RegisterRequest>) -> impl Responder {
    register_user(json.into_inner()).await
}
