use k256::schnorr::{
    signature::{Signer, Verifier},
    SigningKey, VerifyingKey, Signature,
};
use k256::elliptic_curve::rand_core::OsRng;

fn test() {
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    let message = b"Hello, Bitcoin Schnorr!";

    let signature: Signature = signing_key.sign(message);
    println!("Подпись: {:?}", signature);

    match verifying_key.verify(message, &signature) {
        Ok(_) => println!("Подпись корректна"),
        Err(e) => println!("Ошибка: {:?}", e),
    }
}
