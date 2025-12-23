# Getting started with REST API Web Services in Rust using Axum, JWT, SQLx, PostgreSQL# `paseto-auth`

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)

A lightweight, secure, and reusable Rust library for creating and verifying **PASETO v4.public tokens** using Ed25519 signatures. Designed for microservice architectures where a single authentication service issues tokens and multiple services verify them.

> 🔐 **No dependencies on external PASETO crates** – built on stable, well-maintained libraries (`ed25519-dalek`, `serde`, `base64`)

---

## ✨ Features

- ✅ **PASETO v4.public compliant** (Ed25519-signed, unencrypted tokens)
- 🔑 **Single issuer, multi-verifier** architecture
- 🛡️ Built-in **footer validation** to prevent token misuse
- ⏱️ Automatic **expiration (`exp`) and issuance (`iat`) checks**
- 📦 Zero unsafe code, pure Rust
- 🧪 Full error handling with `thiserror`
- 🧩 Reusable across Axum, Actix, or any Rust service

---

## 📦 Installation

Add this to your `Cargo.toml`:

### From Git (recommended while unpublished)

```toml
[dependencies]
paseto-auth = "0.1"
```
---

## 🔑 Key Management
- Generate an Ed25519 keypair once (e.g., using the provided helper or any secure method).
- Store the private key ONLY in your auth service (e.g., via `.env`, *Vault*, or **Kubernetes Secret*).
- Distribute the public key to all verifier services.



> 🔒 **Never share the private key outside the token issuer!**

## 🚀 Usage

### 1. Issuing a Token (Auth Service)

```rust
use paseto_auth::{Claims, create_paseto_v4_public};

// Load your 32-byte private key (e.g., from hex string)
let private_key: [u8; 32] = hex::decode("...")?.try_into()?;

let claims = Claims::new("user_123".into(), 3600) // 1-hour token
    .with_scope("read:profile".into());

let token = create_paseto_v4_public(&claims, &private_key)?;
println!("Token: {}", token);
```

### 2. Verifying a Token (Any Service)
```rust
use paseto_auth::{verify_paseto_v4_public, Claims};

// Load your 32-byte public key
let public_key: [u8; 32] = hex::decode("...")?.try_into()?;

let token = "v4.public...."; // from Authorization header
let claims: Claims = verify_paseto_v4_public(token, &public_key)?;

println!("Valid token for user: {}", claims.sub);
```

### 🛠️ Helper: Generate Keys (Run Once)
```rust
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use rand::RngCore;

let mut sk_bytes = [0u8; 32];
OsRng.fill_bytes(&mut sk_bytes);

let signing_key = SigningKey::from_bytes(&sk_bytes);
let pk_bytes = signing_key.verifying_key().to_bytes();

println!("Private Key (hex): {}", hex::encode(sk_bytes));
println!("Public Key  (hex): {}", hex::encode(pk_bytes));
```
---
### ⚠️ Security Notes
- Tokens are signed but not encrypted (like JWT). Do not store secrets in claims.
- Always use HTTPS in production.
- Keep token TTL short (e.g., 15–60 minutes).
- Use a consistent footer (`myapp` by default) to bind tokens to your application.
- Validate system clocks via NTP to ensure `exp`/`iat` work correctly.
---

### 🙌 Contributing
Contributions welcome! Please open an issue or PR on GitHub.

---
> Built with ❤️ for secure, distributed systems.
Inspired by the [PASETO specification](https://paseto.io/).