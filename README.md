# 🔐 crypto-rs

A simple command-line utility for encrypting and decrypting strings using **AES-256-GCM** with a 32-byte key. Designed to be easily integrated into other scripts or applications via standard input/output.

---

## 🚀 Features

- Symmetric encryption using **AES-256-GCM**
- Authenticated encryption with a random nonce
- Base64 output for safe transport/storage
- Easy to use in scripts or from other programs

---

## 📦 Installation

### Build from source

```sh
git clone https://github.com/fox5352/crypto-rs.git
cd crypto-rs
cargo build --release

Your binary will be in `target/release/crypto-rs`.

---

## 🛠️ Usage

```sh
crypto-rs <32-byte-key> <text> [--decrypt]
```

* `<32-byte-key>`: A 32-character key (exactly 32 bytes) for AES-256.
* `<text>`: The plaintext (to encrypt) or Base64-encoded ciphertext (to decrypt).
* `--decrypt`: Optional flag — decrypts instead of encrypting.

---

## 🔒 Examples

### Encrypt

```sh
crypto-rs "12345678901234567890123456789012" "hello world"
```

**Output:**

```
oJ3tTfOYZmspPlTWmrlu8UmU7k7qLgwdmbuE5W+j6ZQ=
```

---

### Decrypt

```sh
crypto-rs "12345678901234567890123456789012" "oJ3tTfOYZmspPlTWmrlu8UmU7k7qLgwdmbuE5W+j6ZQ=" --decrypt
```

**Output:**

```
hello world
```

---

## 🧪 Programmatic Use

You can use this tool from other languages by capturing `stdout`:

### In Rust:

```rust
use std::process::Command;

let output = Command::new("crypto-rs")
    .args(["12345678901234567890123456789012", "secret message"])
    .output()
    .expect("failed");

let encrypted = String::from_utf8(output.stdout).unwrap().trim().to_string();
```

---

## ⚠️ Notes

* The key **must be 32 bytes** (characters) long.
* The tool uses a **random nonce** every time, so encryption is **non-deterministic** (which is good for security).
* Output is Base64-encoded so it can be used in URLs, JSON, etc.

---
