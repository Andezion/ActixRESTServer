use secp256k1::rand::rngs::OsRng;
use secp256k1::{Message, Secp256k1};
use sha2::{Digest, Sha256};

fn generate_signature() {
    let secp = Secp256k1::new();
    let mut rng = OsRng;

    let (secret_key, public_key) = secp.generate_keypair(&mut rng);
    println!("Secret key: {secret_key:?}");
    println!("Public key: {public_key}");

    let message_bytes = b"Hello, secp256k1!";
    let digest = Sha256::digest(message_bytes);

    let message = Message::from_slice(&digest).expect("32 bytes");

    let sig = secp.sign_ecdsa(&message, &secret_key);
    println!("Signature: {sig}");

    match secp.verify_ecdsa(&message, &sig, &public_key) {
        Ok(_) => println!("Подпись успешно проверена!"),
        Err(e) => println!("Ошибка проверки: {e}"),
    }
}