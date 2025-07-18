use serde::Deserialize; // переделать json в структуру
use validator::Validate; // вводим правила

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
