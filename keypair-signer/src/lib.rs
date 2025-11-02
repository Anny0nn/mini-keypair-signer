//! Mini Keypair Signer

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};

mod tests;

/// Simulated Keypair struct.
///
/// Using [u8; 32] instead of Vec<u8> given that the spec mentions
/// `generates random 32-byte keys` for the [`Keypair::generate()`]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Keypair {
    /// Private 32-byte key.
    pub private_key: [u8; 32],
    /// Public 32-byte key.
    pub public_key: [u8; 32],
}

pub trait Signer {
    fn sign(&self, msg: &[u8]) -> Vec<u8>;
    fn verify(&self, msg: &[u8], sig: &[u8]) -> bool;
}

impl Signer for Keypair {
    fn sign(&self, msg: &[u8]) -> Vec<u8> {
        let data = [msg, &self.private_key.as_slice()].concat();
        Sha256::digest(data).to_vec()
    }

    fn verify(&self, msg: &[u8], sig: &[u8]) -> bool {
        let digest = self.sign(msg);
        digest.as_slice() == sig
    }
}

pub trait Generator {
    fn generate() -> Keypair;
    fn try_restore_from_path(path: &str) -> Result<Keypair, String>;
    fn save_to_path(&self, path: &str) -> Result<(), String>;
}

impl Generator for Keypair {
    fn generate() -> Keypair {
        let mut rng = rand::rng();
        let mut private_key = [0u8; 32];
        rng.fill(&mut private_key);
        let public_key = Sha256::digest(private_key).into();
        Keypair {
            private_key,
            public_key,
        }
    }

    fn try_restore_from_path(path: &str) -> Result<Keypair, String> {
        let data = std::fs::read(path).map_err(|e| e.to_string())?;
        serde_json::from_slice(&data).map_err(|_| "Failed to deserialize keypair".to_string())
    }

    fn save_to_path(&self, path: &str) -> Result<(), String> {
        let data = serde_json::to_vec(self).map_err(|e| e.to_string())?;
        std::fs::write(path, data).map_err(|e| e.to_string())
    }
}
