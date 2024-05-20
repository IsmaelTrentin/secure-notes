use aes_gcm::{
    aead::{AeadMut, OsRng},
    aes::Aes256,
    AeadCore, Aes256Gcm, Key, KeyInit, Nonce,
};
use sha2::{Digest, Sha256};

use crate::Error;

const NONCE_SIZE: usize = 12;
const AES_KEY_LEN: u8 = 32;
const SHA256_LEN: u8 = 64;
const HASH_RUNS: usize = 500;

pub const AUTH_CHECK_STR: &str = "The fox jumps.";

fn do_hash_runs(input: &str) -> String {
    let mut data = input.to_string();

    for _ in 0..=HASH_RUNS {
        data = hex::encode(Sha256::digest(data.as_bytes()));
    }

    data
}

fn pick_32_from_hash_key_64(index: u8, key_hash_64: &str) -> String {
    let safe_idx = index % (SHA256_LEN - 1);

    if safe_idx < 32 {
        return String::from_iter(
            key_hash_64
                .chars()
                .skip(safe_idx as usize)
                .take(AES_KEY_LEN as usize),
        );
    }

    let left = key_hash_64
        .chars()
        .skip(safe_idx as usize + 1)
        .take(((SHA256_LEN - 1) - safe_idx) as usize);
    let remainder = AES_KEY_LEN - ((SHA256_LEN - 1) - safe_idx);
    let right = key_hash_64.chars().take(remainder as usize);
    let new_hash = left.chain(right);

    String::from_iter(new_hash)
}

pub fn hash_key(key: &str) -> String {
    hex::encode(Sha256::digest(key.as_bytes()))
}

pub fn encrypt(key_hash_64: &str, plaintext: &str) -> Vec<u8> {
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let after_runs = do_hash_runs(key_hash_64);
    let key_str = pick_32_from_hash_key_64(nonce[0], &after_runs);

    let key = Key::<Aes256>::from_slice(key_str.as_bytes());

    let mut cipher = Aes256Gcm::new(key);
    let ciphered_data = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .expect("failed to encrypt");

    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);

    encrypted_data
}

pub fn decrypt(key_hash_64: &str, encrypted_data: Vec<u8>) -> Result<String, Error> {
    let (nonce_arr, ciphered_data) = encrypted_data.split_at(NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_arr);
    let after_runs = do_hash_runs(key_hash_64);
    let key_str = pick_32_from_hash_key_64(nonce[0], &after_runs);

    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());

    let mut cipher = Aes256Gcm::new(key);
    let plaintext = cipher.decrypt(nonce, ciphered_data);

    match plaintext {
        Ok(text) => String::from_utf8(text).map_err(|_| Error::FromUtf8Error),
        Err(_) => Err(Error::WrongPassword),
    }
}
