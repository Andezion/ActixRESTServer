use serde::{Serialize, Deserialize}; // для конвертации и обратно

/*
    Стуктура для описания нашего токена, то есть
    id пользователя и время когда токен больше не работает
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,  // типа наш срок действия
}
