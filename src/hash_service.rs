use sha2::{Sha256, Sha512, Digest};

use crate::cli_input::HashAlgorithm;




pub fn handle_hash(hash_type: &HashAlgorithm, input: &str) -> String {
    match hash_type {
        HashAlgorithm::Sha256 => hash_sha256(input),
        HashAlgorithm::Sha512 => hash_sha512(input)
    }
}

fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

fn hash_sha512(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}
