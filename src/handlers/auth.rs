use actix_web::{post, web, HttpResponse}; // для наших ответов и работы с данными
use serde::Deserialize; // для автоматического преобразования
use crate::utils::{hash::verify_password, jwt::generate_token}; // наш импорт
// use crate::models::user::RegisterRequest; // наша модель пользователя
use crate::state::AppState; // тут храним секретный ключ

/*
    Сюда записываем информацию о пользователе
    которая приходит с запроса
*/
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/*
    Это наш ответ на запрос Логин!
    Функция принимает json с информацией и глобальное состояние,
    ну а возвращаем уже наш http ответ.
 */
#[post("/login")]
pub async fn login(
    data: web::Json<LoginRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    // Пока базы нет - то решил сделать так :)
    let hardcoded_user_email = "test@example.com";
    let hardcoded_hash = "$argon2id$v=19$m=19456,t=2,p=1$B3e71rvLQvdduLtdAkB+xg$KM28kb2U12fvpujpleMlmCGa27su/8eHrJXv0vs+XcQ"; // пароль "password123"
    let hardcoded_user_id = "user-123";

    // Проверка почты
    if data.email != hardcoded_user_email {
        return HttpResponse::Unauthorized().body("Invalid email or password");
    }

    // Проверка пароля
    if let Err(_) = verify_password(hardcoded_hash, &data.password) {
        return HttpResponse::Unauthorized().body("Invalid email or password");
    }

    // Тут генерируем наш токен
    let token = match generate_token(hardcoded_user_id, &state.jwt_secret) {
        Ok(token) => token,
        Err(_) => return HttpResponse::InternalServerError().body("Token generation failed"),
    };

    // Если всё обошлось без ошибок - то возвращаем данные в jwt
    HttpResponse::Ok().json(serde_json::json!({ "token": token }))
}
