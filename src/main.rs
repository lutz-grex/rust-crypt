use std::{error::Error, path::Path};


use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use clap::Parser;
use argon2::password_hash::{
        rand_core::OsRng, SaltString,
    };



mod cli_input;
mod file_service;
mod hash_service;
mod cryption_service;
mod helper;

use crate::cli_input::{Cli};
use crate::cryption_service::{decrypt_aes, derive_key_from_password, encrypt_aes};
use crate::file_service::{write_file};
use crate::hash_service::{handle_hash};
use crate::helper::format_error;


fn main() -> Result<(), Box<dyn Error>> {

    let cli = Cli::parse();

    let input_type = match &cli.command {
        cli_input::Commands::Encrypt { cmd } | cli_input::Commands::Decrypt { cmd, .. } => &cmd.input_type,
        cli_input::Commands::Hash { input_type, .. } => input_type,
    };

    let input_content = input_type.extract_input_type_content()?;

    match &cli.command {
        cli_input::Commands::Encrypt { cmd } => {
            let salt = SaltString::generate(&mut OsRng);
            let key = derive_key_from_password(&cmd.password.as_str(), &salt)?;
            match encrypt_aes(&key, &input_content) {
                Ok(encrypted) => {
                    let file_content = format!(
                        "Salt: {:?}\nCiphertext: {}\nNonce: {}",
                        salt.as_str(),
                        BASE64_STANDARD.encode(&encrypted.0),
                        BASE64_STANDARD.encode(&encrypted.1)
                    );

                    write_file(Path::new(&cmd.output), file_content)?
                },
                Err(_) => {}
            };
        },
        cli_input::Commands::Decrypt { cmd, salt, nonce } => {
            let salt = SaltString::from_b64(&salt)
                .map_err(|e| format_error("Salt parsing error", e))?;

            let key = derive_key_from_password(cmd.password.as_str(), &salt)?;

            let encrypted_data = BASE64_STANDARD.decode(input_content)?;
            let nonce_vec = BASE64_STANDARD.decode(&nonce)?;

            let nonce_bytes: [u8; 12] = nonce_vec
                .try_into()
                .map_err(|_| "Nonce muss genau 12 Bytes lang sein")?;

            let plaintext = decrypt_aes(&key, &encrypted_data, &nonce_bytes)
                .map_err(|e| format_error("Entschlüsselungsfehler", e))?;

            println!("{}", plaintext);
        }
        cli_input::Commands::Hash { algorithm, .. } => {
            let result = handle_hash(algorithm, &input_content);
            println!("Result: {}", result);
        }
    };

    Ok(())
}
