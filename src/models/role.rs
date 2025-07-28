use serde::{Serialize, Deserialize};

// Структура ролей с модными аннотациями, пока роли лишь 3, менеджер есть потому что слово смешное
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Roles {
    User,
    Manager, // этот тоже может
    Admin // Важно!! Может посмотреть всех пользователей
}