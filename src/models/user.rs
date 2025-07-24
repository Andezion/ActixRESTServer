use serde::{Deserialize, Serialize}; // переделать json в структуру
use validator::Validate;
use crate::models::role::Roles;
// вводим правила

/*
  Крутой СпрингБут но в расте, debug для того чтобы круто выводить,
  deserialize для того чтобы автоматически получить структуру, и
  валидация для проверки верности данных.
 */
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)] // аналог @Email в java
    pub email: String, // проверяем сразу на валидность почты

    // аналогично @Size(min = 6) в Spring
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String, // и задаём ещё текст ошибки
}

/*
    Структура для того чтобы Админ мог выводить всех пользователей
    Тут у нас индекс пользователя, его почта и собственно сама роль
 */

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: u32,
    pub email: String,
    pub role: Roles,
}