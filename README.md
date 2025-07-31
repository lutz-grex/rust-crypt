# 🔐 Crypto CLI Tool

Ein CLI-Tool in Rust zur **Verschlüsselung**, **Entschlüsselung** und **Hash-Berechnung** von Texten oder Dateien. Unterstützt AES-256-GCM und Hash-Algorithmen wie SHA-256 und SHA-512.

---

## 📦 Features

- 🔐 **Verschlüsselung** mit AES-256-GCM (inkl. Salt & Nonce)
- 🔓 **Entschlüsselung** über Passwort, Salt und Nonce
- #️⃣ **Hashing** mit SHA-256 oder SHA-512
- 📄 Eingabe als Text oder Datei
- 🧾 Ausgabe als Datei oder Standardausgabe

---

## 🚀 Installation

```bash
cargo build --release
```

---

## 🛠️ CLI-Nutzung

### 🔐 Verschlüsselung

```bash
./crypto_tool encrypt --algorithm aes256-gcm --password <PASSWORT> --text "Hallo Welt" --output encrypted.txt
```

### 🔓 Entschlüsselung

```bash
./crypto_tool decrypt --algorithm aes256-gcm --password <PASSWORT> --file encrypted.txt --salt <BASE64_SALT> --nonce <BASE64_NONCE> --output decrypted.txt
```

### 🧾 Hash-Berechnung

```bash
./crypto_tool hash --algorithm sha256 --text "Hallo Welt"
```

---

## 🧠 Architektur

Das Tool besteht aus folgenden Modulen:

- `cli_input`: CLI-Definition mit `clap`
- `cryption_service`: AES-Verschlüsselung/-Entschlüsselung
- `hash_service`: SHA256/SHA512 Hashing
- `file_service`: Datei-Handling (lesen/schreiben)
- `helper`: Fehlerformatierung

---

## 🔑 Schlüsselableitung (Key Derivation)

Für die Verschlüsselung wird der Schlüssel mittels **Argon2** aus dem Passwort und einem zufällig generierten Salt abgeleitet.

```rust
let key = derive_key_from_password(password, &salt)?;
```

---

## 📦 Abhängigkeiten

- `clap`: CLI Argument Parsing
- `aes-gcm`: Authenticated Encryption
- `argon2`: Passwortbasierte Schlüsselableitung
- `sha2`: Hashing-Algorithmen
- `base64`: Codierung von Salt/NONCE/Ciphertext
- `rand`: Sichere Zufallsgenerierung

---

## 📁 Beispielausgabe einer verschlüsselten Datei

```text
Salt: "f2l0mDnd8QkPl5EJoQyYAA"
Ciphertext: <BASE64_VERSCHLÜSSELTER_TEXT>
Nonce: <BASE64_NONCE>
```

---

## 📌 Hinweis

- Entschlüsselung funktioniert nur mit dem **exakten Salt und Nonce**.
- Entweder `--text` oder `--file` muss angegeben werden, nie beides.

---

## ✅ Beispiel

```bash
./crypto_tool encrypt --algorithm aes256-gcm --password geheim --text "Vertraulich" --output secret.txt
```

```bash
./crypto_tool decrypt --algorithm aes256-gcm --password geheim --file secret.txt --salt <SALT> --nonce <NONCE>
```

---

## 📝 Lizenz

MIT License
