use actix_web::{get, web, HttpResponse, Responder}; // импортируем интернет вещи
use crate::state::AppState; // нашу крутую переменную
use bdk::wallet::AddressIndex; // наш адресс

/*
    Обрабатываем запрос на get address, создал случайно а удалять не буду
    По факту всё как в прошлой функции
 */
#[get("/address")]
pub async fn get_address(data: web::Data<AppState>) -> impl Responder {
    let wallet = data.wallet.lock().unwrap();
    let address = wallet.get_address(AddressIndex::New).unwrap();
    HttpResponse::Ok().body(address.to_string())
}

/*
    Добавили дорогу новую
 */
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_address);
}