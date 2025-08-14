use serde::{Deserialize, Serialize}; // переделать json в структуру
use validator::Validate;
// use crate::models::role::Roles; // это на будущее!

#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
pub struct Product {
    pub name: String,
    pub counter: i32,
}
