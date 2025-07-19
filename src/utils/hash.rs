use argon2::{ // длинное добавление всех нужных вещей
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier
    },
    Argon2
};
use argon2::password_hash::SaltString;
use rand::rng;

/*
  Функция которая хэширует наш пароль потому что крутые парни
  делают именно так, пихаем туда обычный срез а возвращаем
  наш переделанный пароль либо ошибку.
 */
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::from_rng(&mut rng());
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}


/*
  На будущее, этим будем проверять верность пароля при логине!
 */
pub fn verify_password(hash: &str, password: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}