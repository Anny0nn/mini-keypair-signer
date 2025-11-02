//! Super simple cli for signing and verifying messages using a keypair.

use clap::Parser;
use mini_keypair_signer::{Generator, Keypair, Signer};

/// Simple program to greet a person.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the keypair file.
    #[arg(short, long, default_value_t = String::new())]
    load_keypair: String,

    /// Message to sign.
    #[arg(short, long, default_value_t = String::new())]
    message: String,

    /// Verify a signed message from a hex string.
    #[arg(long, default_value_t = String::new())]
    verify_signature: String,

    /// Save the keypair to a file path.
    #[arg(short, long, default_value_t = String::new())]
    save: String,
}

fn main() {
    let args = Args::parse();

    let keypair = {
        if args.load_keypair.is_empty() {
            Keypair::generate()
        } else {
            Keypair::try_restore_from_path(&args.load_keypair).unwrap()
        }
    };

    if !args.save.is_empty() {
        keypair.save_to_path(&args.save).unwrap();
    }

    if !args.message.is_empty() {
        if !args.verify_signature.is_empty() {
            let verified = keypair.verify(
                args.message.as_bytes(),
                &hex::decode(args.verify_signature.clone()).unwrap(),
            );
            println!(
                "Message {:?} and signature {:?}, signed by the keypair? {}",
                &args.message, &args.verify_signature, verified
            );
        } else {
            let signature = keypair.sign(args.message.as_bytes());
            println!(
                "Message Signature: \n hex: {} \n bytes: {:?}",
                hex::encode(signature.clone()),
                signature
            );
        }
    }
}
