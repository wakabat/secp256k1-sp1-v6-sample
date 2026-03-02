#![no_main]
sp1_zkvm::entrypoint!(main);

use secp256k1::{Message, PublicKey, Secp256k1};

pub fn main() {
    let sig_bytes: Vec<u8> = sp1_zkvm::io::read();
    let msg_bytes: [u8; 32] = sp1_zkvm::io::read();
    let pk_bytes: Vec<u8> = sp1_zkvm::io::read();

    let secp = Secp256k1::verification_only();
    let sig = secp256k1::ecdsa::Signature::from_compact(&sig_bytes).expect("invalid signature");
    let msg = Message::from_digest(msg_bytes);
    let pk = PublicKey::from_slice(&pk_bytes).expect("invalid public key");

    secp.verify_ecdsa(&msg, &sig, &pk)
        .expect("verification failed");

    sp1_zkvm::io::commit(&true);
}
