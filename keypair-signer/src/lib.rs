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

/// Signer trait responsible for signing and verifying messages.
pub trait Signer {
    /// Sign a message using the private key.
    fn sign(&self, msg: &[u8]) -> Vec<u8>;
    /// Verify if the signature matches the message and the keypair.
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

/// Generator trait responsible for generating and saving keypairs.
pub trait Generator {
    /// Generate a new random keypair.
    fn generate() -> Keypair;
    /// Try to restore a json keypair from a path.
    fn try_restore_from_path(path: &str) -> Result<Keypair, String>;
    /// Save the keypair as json to a path.
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
        let keypair: Keypair = serde_json::from_slice(&data)
            .map_err(|_| "Failed to deserialize keypair".to_string())?;
        // Saving the public key to the json file is unnecessary as
        // it is derived from the private key, so I'm just ignoring it and deriving it again.
        // Ideally, writing a custom impl of `Serialize` and `Deserialize` for the `Keypair` struct
        // would be the best approach to never save the public key to the json file.
        let public_key = Sha256::digest(keypair.private_key).into();
        Ok(Keypair {
            private_key: keypair.private_key,
            public_key,
        })
    }

    fn save_to_path(&self, path: &str) -> Result<(), String> {
        let data = serde_json::to_vec(self).map_err(|e| e.to_string())?;
        std::fs::write(path, data).map_err(|e| e.to_string())
    }
}
