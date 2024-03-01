use super::cipher::{ChaCha20Poly1305Cipher, CipherSuit};
use actix_web::web::Data;
use std::sync::Mutex;

/// app data cipher
/// used in actix app_data
pub type AppDataCipher = Data<Mutex<Box<dyn CipherSuit>>>;

pub fn new_app_data_cipher() -> AppDataCipher {
    Data::new(Mutex::new(Box::new(ChaCha20Poly1305Cipher::new())))
}
