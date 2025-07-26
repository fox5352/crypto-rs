use aes_gcm::aead::{Aead, OsRng, rand_core::RngCore};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use base64::{Engine as _, engine::general_purpose};

fn encrypt(key: &[u8], plaintext: &str) -> Result<String, &'static str> {
    if key.len() != 32 {
        return Err("Key must be 32 bytes for AES-256");
    }

    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| "Invalid key")?;
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|_| "Encryption failed")?;

    let mut combined = Vec::new();
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);
    Ok(general_purpose::STANDARD.encode(&combined))
}

fn decrypt(key: &[u8], encrypted_b64: &str) -> Result<String, &'static str> {
    let combined = general_purpose::STANDARD
        .decode(encrypted_b64)
        .map_err(|_| "Base64 decode failed")?;

    if combined.len() < 12 {
        return Err("Encrypted data too short");
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| "Invalid key")?;

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "Decryption failed")?;

    String::from_utf8(plaintext).map_err(|_| "Invalid UTF-8")
}

fn main() {
    let mut args = std::env::args();
    args.next(); // skip program name

    let arg_count = args.len();
    if arg_count != 2 && arg_count != 3 {
        eprintln!("Error: expected 2 or 3 arguments: <key> <data> [--decrypt]");
        std::process::exit(1);
    }

    let key = args.next().unwrap();
    let input = args.next().unwrap();

    if key.len() != 32 {
        eprintln!("Error: Key must be 32 bytes for AES-256");
        std::process::exit(1);
    }

    let key_bytes = key.as_bytes();
    let output = if arg_count == 3 {
        match decrypt(key_bytes, &input) {
            Ok(text) => text,
            Err(err) => {
                eprintln!("Decryption error: {}", err);
                std::process::exit(1);
            }
        }
    } else {
        match encrypt(key_bytes, &input) {
            Ok(text) => text,
            Err(err) => {
                eprintln!("Encryption error: {}", err);
                std::process::exit(1);
            }
        }
    };

    // ⬇️ This is the important part
    println!("{}", output);
}
