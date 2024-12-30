use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::Rng;

const NONCE_SIZE: usize = 12;

pub fn encrypt(data: &[u8]) -> Result<Vec<u8>, String> {
    let key = Key::<Aes256Gcm>::from_slice(&[0u8; 32]); // Explicit type for key
    let cipher = Aes256Gcm::new(key);

    let nonce_bytes: [u8; NONCE_SIZE] = rand::thread_rng().gen();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, data).map_err(|e| e.to_string())?;
    let mut encrypted_data = nonce_bytes.to_vec();
    encrypted_data.extend(ciphertext);

    Ok(encrypted_data)
}

pub fn decrypt(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < NONCE_SIZE {
        return Err("Invalid data: too short".to_string());
    }

    let key = Key::<Aes256Gcm>::from_slice(&[0u8; 32]);
    let cipher = Aes256Gcm::new(key);

    let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher.decrypt(nonce, ciphertext).map_err(|_| "Decryption failed".to_string())
}
