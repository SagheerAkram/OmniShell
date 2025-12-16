use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use chacha20poly1305::{ChaCha20Poly1305, Key as ChaChaKey};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use crate::error::{OmniShellError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CipherType {
    Aes256Gcm,
    ChaCha20Poly1305,
}

impl CipherType {
    pub fn from_string(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "aes256-gcm" | "aes-256-gcm" | "aes256" => Ok(Self::Aes256Gcm),
            "chacha20-poly1305" | "chacha20" => Ok(Self::ChaCha20Poly1305),
            _ => Err(OmniShellError::Crypto(format!("Unknown cipher: {}", s))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    pub cipher: CipherType,
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
}

pub fn encrypt_message(plaintext: &[u8], key: &[u8], cipher_type: CipherType) -> Result<EncryptedMessage> {
    match cipher_type {
        CipherType::Aes256Gcm => encrypt_aes256_gcm(plaintext, key),
        CipherType::ChaCha20Poly1305 => encrypt_chacha20(plaintext, key),
    }
}

pub fn decrypt_message(encrypted: &EncryptedMessage, key: &[u8]) -> Result<Vec<u8>> {
    match encrypted.cipher {
        CipherType::Aes256Gcm => decrypt_aes256_gcm(&encrypted.ciphertext, key, &encrypted.nonce),
        CipherType::ChaCha20Poly1305 => decrypt_chacha20(&encrypted.ciphertext, key, &encrypted.nonce),
    }
}

fn encrypt_aes256_gcm(plaintext: &[u8], key: &[u8]) -> Result<EncryptedMessage> {
    if key.len() != 32 {
        return Err(OmniShellError::Crypto("AES-256 requires 32-byte key".to_string()));
    }

    let key_array: [u8; 32] = key.try_into()
        .map_err(|_| OmniShellError::Crypto("Invalid key length".to_string()))?;
    
    let cipher = Aes256Gcm::new(&key_array.into());
    
    // Generate random nonce (96 bits for GCM)
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| OmniShellError::Crypto(format!("Encryption failed: {}", e)))?;
    
    Ok(EncryptedMessage {
        cipher: CipherType::Aes256Gcm,
        nonce: nonce_bytes.to_vec(),
        ciphertext,
    })
}

fn decrypt_aes256_gcm(ciphertext: &[u8], key: &[u8], nonce_bytes: &[u8]) -> Result<Vec<u8>> {
    if key.len() != 32 {
        return Err(OmniShellError::Crypto("AES-256 requires 32-byte key".to_string()));
    }

    let key_array: [u8; 32] = key.try_into()
        .map_err(|_| OmniShellError::Crypto("Invalid key length".to_string()))?;
    
    let cipher = Aes256Gcm::new(&key_array.into());
    
    let nonce = Nonce::from_slice(nonce_bytes);
    
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| OmniShellError::Crypto(format!("Decryption failed: {}", e)))?;
    
    Ok(plaintext)
}

fn encrypt_chacha20(plaintext: &[u8], key: &[u8]) -> Result<EncryptedMessage> {
    if key.len() != 32 {
        return Err(OmniShellError::Crypto("ChaCha20 requires 32-byte key".to_string()));
    }

    let key_array: [u8; 32] = key.try_into()
        .map_err(|_| OmniShellError::Crypto("Invalid key length".to_string()))?;
    
    let cipher = ChaCha20Poly1305::new(ChaChaKey::from_slice(&key_array));
    
    // Generate random nonce (96 bits)
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = chacha20poly1305::Nonce::from_slice(&nonce_bytes);
    
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| OmniShellError::Crypto(format!("Encryption failed: {}", e)))?;
    
    Ok(EncryptedMessage {
        cipher: CipherType::ChaCha20Poly1305,
        nonce: nonce_bytes.to_vec(),
        ciphertext,
    })
}

fn decrypt_chacha20(ciphertext: &[u8], key: &[u8], nonce_bytes: &[u8]) -> Result<Vec<u8>> {
    if key.len() != 32 {
        return Err(OmniShellError::Crypto("ChaCha20 requires 32-byte key".to_string()));
    }

    let key_array: [u8; 32] = key.try_into()
        .map_err(|_| OmniShellError::Crypto("Invalid key length".to_string()))?;
    
    let cipher = ChaCha20Poly1305::new(ChaChaKey::from_slice(&key_array));
    
    let nonce = chacha20poly1305::Nonce::from_slice(nonce_bytes);
    
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| OmniShellError::Crypto(format!("Decryption failed: {}", e)))?;
    
    Ok(plaintext)
}

/// Derive a symmetric encryption key from two Ed25519 keys using X25519 key exchange
pub fn derive_shared_key(our_private: &[u8], their_public: &[u8]) -> Result<[u8; 32]> {
    use x25519_dalek::{StaticSecret, PublicKey as X25519PublicKey};
    
    if our_private.len() != 32 || their_public.len() != 32 {
        return Err(OmniShellError::Crypto("Invalid key length for X25519".to_string()));
    }

    // Convert Ed25519 keys to X25519 (this is a simplification - in production,
    // you'd want separate X25519 keys or proper conversion)
    let secret = StaticSecret::from(<[u8; 32]>::try_from(our_private).unwrap());
    let public = X25519PublicKey::from(<[u8; 32]>::try_from(their_public).unwrap());
    
    let shared_secret = secret.diffie_hellman(&public);
    
    // Derive key using HKDF
    use sha2::Sha256;
    use hkdf::Hkdf;
    
    let hk = Hkdf::<Sha256>::new(None, shared_secret.as_bytes());
    let mut okm = [0u8; 32];
    hk.expand(b"omnishell-message-key", &mut okm)
        .map_err(|e| OmniShellError::Crypto(format!("Key derivation failed: {}", e)))?;
    
    Ok(okm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes256_gcm_encryption() {
        let key = [0u8; 32];
        let plaintext = b"Hello, OmniShell!";
        
        let encrypted = encrypt_aes256_gcm(plaintext, &key).unwrap();
        let decrypted = decrypt_aes256_gcm(&encrypted.ciphertext, &key, &encrypted.nonce).unwrap();
        
        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_chacha20_encryption() {
        let key = [0u8; 32];
        let plaintext = b"Hello, OmniShell!";
        
        let encrypted = encrypt_chacha20(plaintext, &key).unwrap();
        let decrypted = decrypt_chacha20(&encrypted.ciphertext, &key, &encrypted.nonce).unwrap();
        
        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_encryption_roundtrip() {
        let key = [42u8; 32];
        let plaintext = b"Secret message for testing";
        
        let encrypted = encrypt_message(plaintext, &key, CipherType::Aes256Gcm).unwrap();
        let decrypted = decrypt_message(&encrypted, &key).unwrap();
        
        assert_eq!(plaintext.to_vec(), decrypted);
    }
}
