#[cfg(test)]
mod tests {
    use crate::*;
    use std::env::temp_dir;

    #[test]
    fn test_save_and_restore_keypair() {
        let keypair = Keypair::generate();
        let path: String = temp_dir().join("keypair.json").to_string_lossy().into();

        assert!(keypair.save_to_path(&path).is_ok());
        let restored_keypair = Keypair::try_restore_from_path(&path).unwrap();

        assert_eq!(keypair.public_key, restored_keypair.public_key);
        assert_eq!(keypair.private_key, restored_keypair.private_key);
    }

    #[test]
    fn try_restore_from_path_with_invalid_path() {
        let path = "invalid_path.json";
        let result = Keypair::try_restore_from_path(path);

        assert!(result.is_err());
    }

    #[test]
    fn try_save_to_path_with_invalid_path() {
        let keypair = Keypair::generate();
        let path = "~!/invalid/invalid_path.json";
        let result = keypair.save_to_path(path);

        assert!(result.is_err());
    }

    #[test]
    fn sign_and_verify() {
        let keypair = Keypair::generate();
        let message = b"secret message";
        let signature = keypair.sign(message);

        assert!(keypair.verify(message, &signature));
    }

    #[test]
    fn sign_and_verify_with_different_message() {
        let keypair = Keypair::generate();
        let message1 = b"secret message";
        let message2 = b"tampered message";
        let signature1 = keypair.sign(message1);

        assert!(keypair.verify(message1, &signature1));
        assert!(!keypair.verify(message2, &signature1));
    }

    #[test]
    fn sign_and_verify_with_different_keypair() {
        let keypair1 = Keypair::generate();
        let keypair2 = Keypair::generate();
        let message = b"secret message";
        let signature1 = keypair1.sign(message);

        assert!(!keypair2.verify(message, &signature1));
    }

    #[test]
    fn sign_and_verify_with_different_signature() {
        let keypair = Keypair::generate();
        let message = b"secret message";
        let signature1 = keypair.sign(message);
        let signature2 = keypair.sign(message);

        assert!(keypair.verify(message, &signature1));
        assert!(keypair.verify(message, &signature2));
    }
}
