# TrollCryption

A chaotic encryption tool that punishes incorrect keys with progressive drift.

Trollcryption is an experimental Rust encryption tool built around a simple idea:

Using the wrong key shouldn’t just fail — it should make things worse.

Unlike normal ciphers where a wrong key simply produces gibberish, Trollcryption mutates the ciphertext each time a decryption attempt is made with an incorrect key. The more wrong guesses, the further the data spirals away from its original state.

This project is for educational and entertainment purposes only — a demonstration of how cryptography, key derivation, and deterministic transformations can be combined to create anti‑bruteforce behavior.
It is not meant for securing real data.

✨ Features

🔐 AES‑256‑GCM encryption for actual confidentiality

🎯 Key hashing via BLAKE3 to normalize user‑provided keys

🤡 Progressive mutation:
Every wrong key triggers a deterministic drift in the ciphertext

🧪 Fully reversible:
Decryption succeeds only with the original correct key

📁 Simple command‑line interface for encryption/decryption of files

💥 Resistant to brute forcing — incorrect attempts make things worse

⚠️ Disclaimer

This tool is intentionally chaotic and should not be used for real‑world encryption, data protection, production systems, or anything involving actual security.

You are solely responsible for any use or misuse of the software. The author(s) assume no liability for damage, data loss, or cosmic horrors unleashed by incorrect keys.

📦 Installation
git clone https://github.com/yourname/trollcryption
cd trollcryption
cargo build --release


The compiled binary will be in:

target/release/trollcryption

🚀 Usage
Encrypt
./trollcryption e


You will be prompted to enter:

the file to encrypt

a 32‑byte key (ASCII or raw bytes)

This produces file.troll and deletes nothing automatically.

Decrypt
./trollcryption d


Using the correct key restores the original file.
Using the wrong key mutates the ciphertext — future attempts may become even harder.

🧩 How It Works (Conceptually)

User‑provided key → hashed to 32 bytes using BLAKE3

AES‑256‑GCM encrypts the file

On failed decryption:

A new derived key is produced using the failed attempt

The ciphertext is re‑encrypted with this derived key

The file is permanently mutated until the original key is used

On correct decryption:

Data is restored

No further mutations occur

This creates a deterministic "entropy spiral" effect.

🛠️ Code Structure
trollcryption/
├── Cargo.toml
└── src/
    └── main.rs


Everything is contained in a single main.rs for simplicity.

🤝 Contributing

Pull requests, suggestions, and improvements are welcome — whether to make Trollcryption more chaotic or more polished.

📜 License

MIT License
Do whatever you want — but not with real sensitive data.
