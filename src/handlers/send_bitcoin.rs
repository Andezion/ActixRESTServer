use actix_web::{post, web, HttpResponse, Responder};
use bitcoin::{Address, Network};
use std::str::FromStr;
use bdk::blockchain::Blockchain;
use bdk::SignOptions;
use crate::state::AppState;

#[derive(serde::Deserialize)]
pub struct SendRequest {
    to_address: String,
    amount: u64,
}

#[post("/send")]
pub async fn send_bitcoin(
    data: web::Data<AppState>,
    req: web::Json<SendRequest>,
) -> impl Responder {
    if req.amount == 0 {
        return HttpResponse::BadRequest().body("Amount must be greater than 0");
    }

    let wallet = data.wallet.lock().unwrap();
    let blockchain = data.blockchain.lock().unwrap();

    let script_pubkey = match Address::from_str(&req.to_address)
        .and_then(|addr| addr.require_network(Network::Testnet)) {
        Ok(addr) => addr.script_pubkey(),
        Err(_) => return HttpResponse::BadRequest()
            .body("Invalid address or wrong network"),
    };

    let script= script_pubkey.to_string();

    let mut builder = wallet.build_tx();
    builder.add_recipient(script.parse().unwrap(), req.amount);

    let (mut psbt, _) = match builder.finish() {
        Ok(r) => r,
        Err(e) => return HttpResponse::InternalServerError()
            .body(format!("Build error: {}", e)),
    };

    if let Err(e) = wallet.sign(&mut psbt, SignOptions::default()) {
        return HttpResponse::InternalServerError()
            .body(format!("Sign error: {}", e));
    }

    let tx = psbt.extract_tx();
    match blockchain.broadcast(&tx) {
        Ok(_) => HttpResponse::Ok().body(format!("Sent! TXID: {}", tx.txid())),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Broadcast error: {}", e)),
    }
}