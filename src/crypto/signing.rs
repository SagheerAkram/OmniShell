use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use crate::crypto::keys::KeyPair;
use crate::error::{OmniShellError, Result};

pub fn sign_data(data: &[u8], keypair: &KeyPair) -> Vec<u8> {
    let signature = keypair.signing_key.sign(data);
    signature.to_bytes().to_vec()
}

pub fn verify_signature(data: &[u8], signature_bytes: &[u8], public_key: &VerifyingKey) -> Result<()> {
    let signature_array: [u8; 64] = signature_bytes
        .try_into()
        .map_err(|_| OmniShellError::Crypto("Invalid signature length".to_string()))?;
    
    let signature = Signature::from_bytes(&signature_array);
    
    public_key
        .verify(data, &signature)
        .map_err(|e| OmniShellError::Crypto(format!("Signature verification failed: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::generate_keypair;

    #[test]
    fn test_sign_and_verify() {
        let keypair = generate_keypair();
        let data = b"Important message to sign";
        
        let signature = sign_data(data, &keypair);
        assert_eq!(signature.len(), 64);
        
        let result = verify_signature(data, &signature, &keypair.verifying_key);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_signature() {
        let keypair = generate_keypair();
        let data = b"Important message to sign";
        
        let signature = sign_data(data, &keypair);
        
        // Tamper with data
        let tampered_data = b"Tampered message";
        let result = verify_signature(tampered_data, &signature, &keypair.verifying_key);
        assert!(result.is_err());
    }
}
