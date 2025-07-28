use actix_web::{get, web, HttpResponse, Responder}; // для работы с вебом
use bdk::wallet::AddressIndex; // для работы с кошельками
use crate::state::AppState; // импорт нашей важной переменной

/*
    Метод get который вернёт нам адресс кошелька. /
    Сначала мы получаем состояние приложение через обёртку,
    ну и потом возрващаем полученно в http ответ.
    Потом ТОЛЬКО один поток получает доступ к кошельку,
    который мы тут распаковываем. После из этого кошелька просим адресс его.
    Если всё хорошо то возвращаем его адресс.
 */
#[get("/wallet/address")]
pub async fn get_address(data: web::Data<AppState>) -> impl Responder { // респондер - это html
    let wallet = data.wallet.lock().unwrap();
    let address = wallet.get_address(AddressIndex::New).unwrap();
    HttpResponse::Ok().body(address.to_string()) // вот тут возврат
}

/*
    Короче по модному - код реализует простую API точку, которая при get
    запросе на /wallet/address возвращает новый биткойн адрес из кошелька,
    хранящегося в состоянии приложения
 */