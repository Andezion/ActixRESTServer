use actix_web::{HttpResponse}; // чтобы использовать структуру
use validator::Validate;
// чтобы использовать правила
use crate::models::user::RegisterRequest;
use crate::utils::hash::hash_password;
/*
  Модно передаём стринг, ибо мало-ли имя не указали.
  После собираем ответ в зависимости от указанного имени.
  Возвращаем структуру с кодом ОК и информацией нашей.
 */
pub async fn greet_user(name: Option<String>) -> HttpResponse {
    let response = match name {
        Some(n) => format!("Hello, {}!", n),
        None => "Hello?? Who you are???".to_string(),
    }; 
    HttpResponse::Ok().body(response)
}

/*
  Принимаем структуру из POST и начинаем проверку модную.
  Если почта и пароль нормальные(по валидатору) - то успех.
  Если что-то не так то видим ошибку от валидатора.
 */
pub async fn register_user(req: RegisterRequest) -> HttpResponse {
    match req.validate() {
        Ok(_) => /* HttpResponse::Ok().body("Registered successfully")*/ {
            match hash_password(&req.password) {
                Ok(hashed) => {
                    HttpResponse::Ok().body(format!("Registered with hash: {}", hashed))
                } // тут уже добавил не хардкод, а просто регистрацию
                Err(_) => HttpResponse::InternalServerError().body("Hashing failed"),
            }
        }, // прописал 100500 проверок на ошибки
        Err(e) => HttpResponse::BadRequest().body(format!("Validation error: {:?}", e)),
    }
}
