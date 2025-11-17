use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit, generic_array::GenericArray};
use blake3;
use std::fs;
use std::io::{self, Write};

pub struct TrollCrypt {
    ciphertext: Vec<u8>,
    salt: [u8; 32],
}

impl TrollCrypt {
    /// Create a new trollcrypted state from plaintext + key
    pub fn new(plaintext: &[u8], key: &[u8; 32]) -> Self {
        let key_array = GenericArray::from_slice(key);
        let cipher = Aes256Gcm::new(key_array);
        let nonce = Nonce::from_slice(&[0u8; 12]);
        let ciphertext = cipher.encrypt(nonce, plaintext).expect("encryption failed");

        let salt = blake3::hash(key).into();

        Self { ciphertext, salt }
    }

    /// Attempt to decrypt with a key
    /// On success: returns Ok(plaintext)
    /// On wrong key: mutates internal state, returns Err(())
    pub fn attempt(&mut self, key: &[u8; 32]) -> Result<Vec<u8>, ()> {
        let key_array = GenericArray::from_slice(key);
        let cipher = Aes256Gcm::new(key_array);
        let nonce = Nonce::from_slice(&[0u8; 12]);

        if let Ok(plaintext) = cipher.decrypt(nonce, self.ciphertext.as_ref()) {
            return Ok(plaintext);
        }

        // Wrong key → derive new key from salt + guessed key
        let derived = blake3::keyed_hash(&self.salt, key);
        let new_key = &derived.as_bytes()[..32];

        let new_key_array = GenericArray::from_slice(new_key);
        let new_cipher = Aes256Gcm::new(new_key_array);

        self.ciphertext = new_cipher
            .encrypt(nonce, self.ciphertext.as_ref())
            .unwrap();

        self.salt = blake3::hash(&[self.salt.as_ref(), key].concat()).into();

        Err(())
    }

    /// Save ciphertext to file (salt + ciphertext)
    pub fn save(&self, filename: &str) {
        let mut out = Vec::new();
        out.extend_from_slice(&self.salt);
        out.extend_from_slice(&self.ciphertext);
        fs::write(filename, &out).expect("failed to write file");
    }

    /// Load from file (salt + ciphertext)
    pub fn load(filename: &str) -> Self {
        let data = fs::read(filename).expect("failed to read file");
        let mut salt = [0u8; 32];
        salt.copy_from_slice(&data[..32]);
        let ciphertext = data[32..].to_vec();
        Self { ciphertext, salt }
    }
}

/// Hash any-length password to 32-byte AES key
fn password_to_key(password: &[u8]) -> [u8; 32] {
    let hash = blake3::hash(password);
    let mut key = [0u8; 32];
    key.copy_from_slice(&hash.as_bytes()[..32]);
    key
}

fn main() {
    print!("Encrypt or decrypt? (e/d): ");
    io::stdout().flush().unwrap();

    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim().to_lowercase();

    if mode == "e" {
        print!("Enter path to file to encrypt: ");
        io::stdout().flush().unwrap();
        let mut data_file = String::new();
        io::stdin().read_line(&mut data_file).unwrap();
        let data_file = data_file.trim();
        let data = fs::read(data_file).expect("failed to read file");

        print!("Enter encryption password: ");
        io::stdout().flush().unwrap();
        let mut password = String::new();
        io::stdin().read_line(&mut password).unwrap();
        let password = password.trim().as_bytes();

        let key = password_to_key(password);

        let troll = TrollCrypt::new(&data, &key);
        troll.save("trolled.bin");

        println!("File encrypted to trolled.bin");

    } else if mode == "d" {
        print!("Enter path to trollcrypted file: ");
        io::stdout().flush().unwrap();
        let mut file = String::new();
        io::stdin().read_line(&mut file).unwrap();
        let file = file.trim();

        let mut troll = TrollCrypt::load(file);

        print!("Enter key guess: ");
        io::stdout().flush().unwrap();
        let mut password = String::new();
        io::stdin().read_line(&mut password).unwrap();
        let password = password.trim().as_bytes();

        let key = password_to_key(password);

        match troll.attempt(&key) {
            Ok(plaintext) => {
                fs::write("decrypted.bin", &plaintext).expect("failed to write output");
                println!("Decryption successful! Saved to decrypted.bin");
            }
            Err(()) => {
                troll.save("trolled.bin");
                println!("Wrong key! File has been mutated.");
            }
        }

    } else {
        println!("Invalid mode. Choose 'e' or 'd'.");
    }
}
