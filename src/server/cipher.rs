//! Cipher
//! included [CipherSuit] trait and default implemention with ChaCha20Poly1305
//! if you would like to implement other cipher suit, please implement [CipherSuit] trait

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,Key
};
use once_cell::sync::Lazy;

static CHACHA_KEY: Lazy<[u8; 32]> = Lazy::new(|| {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut arr = [0u8; 32];
    for v in arr.iter_mut() {
        *v = rng.gen();
    }

    arr
});

static CHACHA_NONCE: Lazy<[u8; 12]> = Lazy::new(|| {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut arr = [0u8; 12];
    for v in arr.iter_mut() {
        *v = rng.gen();
    }

    arr
});

/// The result of both encrypt or decrypt
pub type CipherResult = Result<Vec<u8>, ()>;

pub trait CipherSuit {
    fn encrypt(&mut self, plaintext: &Vec<u8>) -> CipherResult;
    fn decrypt(&mut self, ciphertext: &Vec<u8>) -> CipherResult;
}

/// default implemention of [CipherSuit]
pub struct ChaCha20Poly1305Cipher {
    cipher: ChaCha20Poly1305,
    nonce: Nonce,
}

impl ChaCha20Poly1305Cipher {
    pub fn new() -> Self {

        let key = Key::clone_from_slice(&*CHACHA_KEY);
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = Nonce::clone_from_slice(&*CHACHA_NONCE);
        Self { cipher, nonce }
    }
}

impl CipherSuit for ChaCha20Poly1305Cipher {

    fn encrypt(&mut self, plaintext: &Vec<u8>) -> CipherResult {
        self.cipher
            .encrypt(&self.nonce, plaintext.as_ref())
            .map_err(|_| ())
    }

    fn decrypt(&mut self, ciphertext: &Vec<u8>) -> CipherResult {
        self.cipher
            .decrypt(&self.nonce, ciphertext.as_ref())
            .map_err(|_| ())
    }
}
