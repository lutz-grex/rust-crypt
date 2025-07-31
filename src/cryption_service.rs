
use std::error::Error;

use aes_gcm::aead::Aead;
use argon2::{Argon2};
use argon2::password_hash::{SaltString};
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use rand::rngs::OsRng;
use rand::RngCore;

use crate::helper::format_error;

pub fn derive_key_from_password(password: &str, salt: &SaltString) -> Result<Key<Aes256Gcm>, Box<dyn Error>> {
    let argon2 = Argon2::default();
    let mut key_bytes = [0u8; 32];
    argon2.hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut key_bytes)
        .map_err(|e| format_error("Error deriving key", e))?;
    
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    Ok(*key)
}

pub fn encrypt_aes(key: &Key<Aes256Gcm>, input: &str) -> Result<(Vec<u8>, [u8; 12]), Box<dyn Error>> {
    let cipher = Aes256Gcm::new(key);
    
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let encrypted = cipher.encrypt(nonce, input.as_bytes())
        .map_err(|e| format_error("Verschlüsselungsfehler", e))?;
    
    Ok((encrypted, nonce_bytes))
}

pub fn decrypt_aes(key: &Key<Aes256Gcm>, encrypted_data: &[u8], nonce: &[u8; 12]) -> Result<String, Box<dyn Error>> {
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);
    
    let decrypted = cipher.decrypt(nonce, encrypted_data)
        .map_err(|e| format_error("Entschlüsselungsfehler", e))?;
    
    String::from_utf8(decrypted)
        .map_err(|e| format_error("UTF-8 Konvertierungsfehler", e))
}