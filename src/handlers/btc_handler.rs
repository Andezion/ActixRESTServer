use actix_web::{get, HttpResponse, Responder}; // для того чтобы пустили в интернеты
use serde::Deserialize; // для парсинга

#[derive(Debug, Deserialize)]
struct PriceBitcoin { // тут мы храним нашу структуру с ценами
    bitcoin: CurrencyData,
}

#[derive(Debug, Deserialize)]
struct CurrencyData { // наша структура с ценами
    usd: f64,
    pln: f64,
    uah: f64
}

// Собственно наша дорога прайс
#[get("/price")]
pub async fn get_btc_price() -> impl Responder {
    // Сначала у нас есть адресс на который мы топаем
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd,pln,uah";
    // умоляем адресс и ждём от него ответа
    match reqwest::get(url).await {
        // если адресс сказал что всё гуд - парсим ответ
        Ok(response) => {
            if let Ok(parsed) = response.json::<PriceBitcoin>().await {
                HttpResponse::Ok().json(serde_json::json!({ // теперь пробуем полученную информацию разбить
                    // на нужные нам валюты
                    "BTC_USD": parsed.bitcoin.usd,
                    "BTC_PLN": parsed.bitcoin.pln,
                    "BTC_UAN": parsed.bitcoin.uah
                }))
            } else { // если что пошло не так - ошибка
                HttpResponse::InternalServerError().body("Failed to parse response")
            }
        }
        Err(e) => { // если уже с запросом что-то не пошло
            eprintln!("HTTP request error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch BTC price")
        }
    }
}