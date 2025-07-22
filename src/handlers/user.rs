use actix_web::{get, HttpResponse, HttpRequest, web}; // набор интернет приколов
use crate::utils::jwt::validate_token; // для проверки токена
use crate::models::auth::Claims; // структура с данными токена
use crate::state::AppState; // импортируем секрет

/*
    Для метода GET /me, получаем наш запрос и наш пароль,
    потом пытаемся извлечь заголовки, если нет - ошибка.
    Потом убираем префикс - получая тем самым токен.
    Далее проверям валидность токена. Если всё хорошо - возвращаем
    user_id
 */
#[get("/me")]
pub async fn me(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    // Пытаемся получить токен сырой пока что
    let auth_header = match req.headers().get("Authorization") {
        Some(header) => header.to_str().unwrap_or(""),
        None => return HttpResponse::Unauthorized().body("No authorization header"),
    };

    // Условно убираем префикс нашего токена
    let token = auth_header.strip_prefix("Bearer ").unwrap_or("");
    if token.is_empty() {
        return HttpResponse::Unauthorized().body("Invalid token format");
    }

    // Проверка нашего токена
    let claims: Claims = match validate_token(token, &state.jwt_secret) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid or expired token"),
    };

    // Как раз последняя проверка
    HttpResponse::Ok().json(serde_json::json!({
        "user_id": claims.sub
    }))
}
