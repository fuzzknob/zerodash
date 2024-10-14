use argon2::{
    self,
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use uuid::Uuid;

pub fn hash_password(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let hashed_password = argon
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    hashed_password
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    let Ok(hash) = PasswordHash::new(hashed_password) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .is_ok()
}

pub fn get_unique_random_hash() -> String {
    Uuid::new_v4().to_string().replace("-", "")
}
