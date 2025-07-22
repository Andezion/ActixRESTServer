/*
    Структура чтобы хранить наш секретный ключ,
    в будущем нужно переместить в .env!!!
 */

#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
}