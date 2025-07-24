use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder}; // наши марштуры, веб для обёрток и чтобы возвращать http
use crate::handlers::user_handler::{greet_user, register_user};
use crate::models::auth::Claims;
use crate::models::role::Roles;
// наши функции
use crate::models::user::{RegisterRequest, UserResponse};
use crate::state::AppState;
use crate::utils::jwt::validate_token;
// структура

/*
  Условно это наша карта, у нас есть 2 варианта куда можно пойти.
  Нам это нужно потому что Actix требует чтобы все маршруты были зарегестрированы заранее!

 */
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(greet)
        .service(register)
        .service(get_users);
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

/*
    На радость админам добавил возможность посмотреть всех пользователей.
    В теории нужно тянуть это всё из базы, но пока базы нет - то решил сделать
    уже так. Пользователи захардкодены!
    
    Сначала проверяем токен auth и если его нет - то ошибка!
    Потом распаковываем его, или - ошибка!
    Потом валидация токена или - ошибка!
    Ну а дальше проверяем вообще кто просит, и если просит начальство - 
    то показываем вектор пользователей, если не начальство - то вежливо
    отказываем...
 */
#[get("/all")]
async fn get_users(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let auth_header = match req.headers().get("Authorization") {
        Some(h) => h.to_str().unwrap_or(""),
        None => return HttpResponse::Unauthorized().body("No authorization header"),
    };

    // разбираем наш токен 
    let token = auth_header.strip_prefix("Bearer ").unwrap_or("");
    let claims: Claims = match validate_token(token, &state.jwt_secret) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid or expired token"),
    };

    match claims.role {
        Roles::Admin | Roles::Manager => { // проверка роли
            // наши работяги
            let users = vec![
                UserResponse {
                    id: 1,
                    email: "admin@example.com".to_string(),
                    role: Roles::Admin,
                },
                UserResponse {
                    id: 2,
                    email: "manager@example.com".to_string(),
                    role: Roles::Manager,
                },
                UserResponse {
                    id: 3,
                    email: "user@example.com".to_string(),
                    role: Roles::User,
                },
            ];

            HttpResponse::Ok().json(users)
        }
        _ => HttpResponse::Forbidden().body("No. Loser"),
    }
}
