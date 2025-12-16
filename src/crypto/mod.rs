pub mod keys;
pub mod encryption;
pub mod signing;
pub mod kdf;

pub use keys::{KeyPair, generate_keypair};
pub use encryption::{encrypt_message, decrypt_message};
pub use signing::verify_signature;

// Wrapper function for sign_message (used in testing)
pub fn sign_message(data: &[u8], keypair: &KeyPair) -> Vec<u8> {
    signing::sign_data(data, keypair)
}

// Wrapper function for derive_key_from_password (used in backup)
pub fn derive_key_from_password(password: &str) -> crate::error::Result<[u8; 32]> {
    let (key, _salt) = kdf::derive_key(password, None)?;
    Ok(key)
}
