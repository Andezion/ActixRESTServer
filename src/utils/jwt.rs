use chrono::{Utc, Duration}; // для проверки срока действия
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation}; // для генерации токена
use crate::models::auth::Claims;
use crate::models::role::Roles;
// берём нашу модель

/*
    Создаём токен по id пользователя и секретному ключу,
    сначала задаём задаём время окончания действия, потом 
    создаём структуру с id и временем действия.
    
    Ну и потом возвращаем готовый токен либо ошибку, если не пошло
 */
pub fn generate_token(user_id: &str, role: Roles, secret_key: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(60))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration, // Наш срок действия!!!
        role // добавил переменную которая хранит роль
    };
    
    // Пробуем закодировать
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_bytes()))
}


/*
    Эта же функция проверяет валидность нашего токена.
    То есть берём наши данные, пихаем в функцию декода - если 
    выдаёт ошибку то мы об этом узнаем из-за '?'
 */
pub fn validate_token(token: &str, secret_key: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::default(),
    )?;

    // Возвращаем наши данные если нет никаких ошибок (то есть структуру)
    Ok(token_data.claims)
}
