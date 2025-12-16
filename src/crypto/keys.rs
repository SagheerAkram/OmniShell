use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;
use blake2::Digest; // Add this import for digest() method
use crate::error::{OmniShellError, Result};

const WORDLIST: &[&str] = &[
    "ALPHA", "BRAVO", "CHARLIE", "DELTA", "ECHO", "FOXTROT", "GOLF", "HOTEL",
    "INDIA", "JULIET", "KILO", "LIMA", "MIKE", "NOVEMBER", "OSCAR", "PAPA",
    "QUEBEC", "ROMEO", "SIERRA", "TANGO", "UNIFORM", "VICTOR", "WHISKEY", "XRAY",
    "YANKEE", "ZULU",
];

#[derive(Clone)]
pub struct KeyPair {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl KeyPair {
    pub fn public_key(&self) -> PublicKey {
        PublicKey {
            key: self.verifying_key,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.signing_key.to_bytes().to_vec()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let bytes_array: [u8; 32] = bytes
            .try_into()
            .map_err(|_| OmniShellError::InvalidKey("Invalid key length".to_string()))?;
        
        let signing_key = SigningKey::from_bytes(&bytes_array);
        let verifying_key = signing_key.verifying_key();
        
        Ok(Self {
            signing_key,
            verifying_key,
        })
    }
}

impl Drop for KeyPair {
    fn drop(&mut self) {
        // Zeroize sensitive key material
        let mut bytes = self.signing_key.to_bytes();
        bytes.zeroize();
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PublicKey {
    #[serde(with = "public_key_serde")]
    pub key: VerifyingKey,
}

impl PublicKey {
    pub fn to_string(&self) -> String {
        format!("omni:{}", hex::encode(self.key.to_bytes()))
    }

    pub fn from_string(s: &str) -> Result<Self> {
        if !s.starts_with("omni:") {
            return Err(OmniShellError::InvalidKey("Key must start with 'omni:'".to_string()));
        }

        let hex_part = &s[5..];
        let bytes = hex::decode(hex_part)
            .map_err(|_| OmniShellError::InvalidKey("Invalid hex encoding".to_string()))?;
        
        let bytes_array: [u8; 32] = bytes
            .try_into()
            .map_err(|_| OmniShellError::InvalidKey("Invalid key length".to_string()))?;
        
        let key = VerifyingKey::from_bytes(&bytes_array)
            .map_err(|e| OmniShellError::InvalidKey(e.to_string()))?;
        
        Ok(Self { key })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.key.to_bytes().to_vec()
    }

    pub fn fingerprint(&self) -> String {
        let bytes = self.key.to_bytes();
        let hash = blake2::Blake2b512::digest(&bytes);
        
        // Take first 8 bytes and convert to human-readable format
        let mut parts = Vec::new();
        
        // First word from wordlist
        parts.push(WORDLIST[(hash[0] as usize) % WORDLIST.len()].to_string());
        
        // Second word from wordlist
        parts.push(WORDLIST[(hash[1] as usize) % WORDLIST.len()].to_string());
        
        // 4-digit number
        let num = u16::from_be_bytes([hash[2], hash[3]]) % 10000;
        parts.push(format!("{:04}", num));
        
        // Third word
        parts.push(WORDLIST[(hash[4] as usize) % WORDLIST.len()].to_string());
        
        // Fourth word
        parts.push(WORDLIST[(hash[5] as usize) % WORDLIST.len()].to_string());
        
        parts.join("-")
    }

    pub fn visual_hash(&self) -> String {
        // Generate a visual hash using Unicode block characters
        let bytes = self.key.to_bytes();
        let hash = blake2::Blake2b512::digest(&bytes);
        
        let blocks = ['█', '▓', '▒', '░', ' '];
        let mut visual = String::new();
        
        for chunk in hash[..16].chunks(2) {
            let idx = (chunk[0] as usize) % blocks.len();
            visual.push(blocks[idx]);
        }
        
        visual
    }
}

// Custom serde for VerifyingKey
mod public_key_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(key: &VerifyingKey, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&key.to_bytes())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<VerifyingKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = serde::Deserialize::deserialize(deserializer)?;
        let bytes_array: [u8; 32] = bytes
            .try_into()
            .map_err(|_| serde::de::Error::custom("Invalid key length"))?;
        
        VerifyingKey::from_bytes(&bytes_array)
            .map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

pub fn generate_keypair() -> KeyPair {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    
    KeyPair {
        signing_key,
        verifying_key,
    }
}

pub fn generate_device_id() -> String {
    use rand::Rng;
    let mut rng = OsRng;
    let bytes: [u8; 16] = rng.gen();
    hex::encode(bytes)
}

pub fn generate_fingerprint(public_key_bytes: &[u8]) -> String {
    let hash = blake2::Blake2b512::digest(public_key_bytes);
    
    // Take first 8 bytes and convert to human-readable format
    let mut parts = Vec::new();
    
    // First word from wordlist
    parts.push(WORDLIST[(hash[0] as usize) % WORDLIST.len()].to_string());
    
    // Second word from wordlist
    parts.push(WORDLIST[(hash[1] as usize) % WORDLIST.len()].to_string());
    
    // 4-digit number
    let num = u16::from_be_bytes([hash[2], hash[3]]) % 10000;
    parts.push(format!("{:04}", num));
    
    // Third word
    parts.push(WORDLIST[(hash[4] as usize) % WORDLIST.len()].to_string());
    
    // Fourth word
    parts.push(WORDLIST[(hash[5] as usize) % WORDLIST.len()].to_string());
    
    parts.join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = generate_keypair();
        let public_key = keypair.public_key();
        
        assert_eq!(public_key.to_bytes().len(), 32);
    }

    #[test]
    fn test_public_key_serialization() {
        let keypair = generate_keypair();
        let public_key = keypair.public_key();
        
        let serialized = public_key.to_string();
        assert!(serialized.starts_with("omni:"));
        
        let deserialized = PublicKey::from_string(&serialized).unwrap();
        assert_eq!(public_key.to_bytes(), deserialized.to_bytes());
    }

    #[test]
    fn test_fingerprint() {
        let keypair = generate_keypair();
        let public_key = keypair.public_key();
        
        let fingerprint = public_key.fingerprint();
        assert!(fingerprint.contains("-"));
        
        // Should be consistent
        assert_eq!(fingerprint, public_key.fingerprint());
    }

    #[test]
    fn test_visual_hash() {
        let keypair = generate_keypair();
        let public_key = keypair.public_key();
        
        let visual = public_key.visual_hash();
        assert_eq!(visual.chars().count(), 8);
    }
}
