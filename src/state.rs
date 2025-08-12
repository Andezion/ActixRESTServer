/*
    Структура чтобы хранить наш секретный ключ,
    в будущем нужно переместить в .env!!!
 */

use std::sync::{Arc, Mutex};
use bdk::{Wallet, database::MemoryDatabase};
use bdk::blockchain::ElectrumBlockchain;

#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
    pub wallet: Arc<Mutex<Wallet<MemoryDatabase>>>, // наш кошелёк - тоже секретный и поэтому храниться тут!
    pub blockchain: Arc<Mutex<ElectrumBlockchain>>,
}