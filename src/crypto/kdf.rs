use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};
use crate::error::{OmniShellError, Result};

/// Derive a key from a password using Argon2id
pub fn derive_key(password: &str, salt: Option<&[u8]>) -> Result<([u8; 32], Vec<u8>)> {
    let argon2 = Argon2::default();
    
    let salt_string = if let Some(s) = salt {
        SaltString::encode_b64(s)
            .map_err(|e| OmniShellError::Crypto(format!("Salt encoding failed: {}", e)))?
    } else {
        SaltString::generate(&mut OsRng)
    };
    
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt_string)
        .map_err(|e| OmniShellError::Crypto(format!("Key derivation failed: {}", e)))?;
    
    // Extract the hash bytes
    let hash_bytes = password_hash.hash
        .ok_or_else(|| OmniShellError::Crypto("No hash generated".to_string()))?;
    
    let mut key = [0u8; 32];
    key.copy_from_slice(&hash_bytes.as_bytes()[..32]);
    
    Ok((key, salt_string.as_str().as_bytes().to_vec()))
}

/// Verify a password against a stored hash
pub fn verify_password(password: &str, hash_str: &str) -> Result<bool> {
    let argon2 = Argon2::default();
    
    let parsed_hash = PasswordHash::new(hash_str)
        .map_err(|e| OmniShellError::Crypto(format!("Invalid hash format: {}", e)))?;
    
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

/// Simple key derivation using SHA-256 (for non-password use cases)
pub fn derive_key_simple(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_derivation() {
        let password = "super_secret_password";
        let (key1, salt) = derive_key(password, None).unwrap();
        
        assert_eq!(key1.len(), 32);
        assert!(!salt.is_empty());
        
        // Same password and salt should produce same key
        let (key2, _) = derive_key(password, Some(&salt)).unwrap();
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_simple_derivation() {
        let input = b"some input data";
        let key = derive_key_simple(input);
        
        assert_eq!(key.len(), 32);
        
        // Should be deterministic
        assert_eq!(key, derive_key_simple(input));
    }
}
