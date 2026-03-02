use secp256k1::hashes::{sha256, Hash};
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Message, Secp256k1};
use sp1_sdk::{
    blocking::{Prover, ProverClient},
    include_elf, Elf, SP1Stdin,
};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
const FIBONACCI_ELF: Elf = include_elf!("fibonacci-program");

fn main() {
    sp1_sdk::utils::setup_logger();

    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    let digest = sha256::Hash::hash("Hello World!".as_bytes());
    let message = Message::from_digest(digest.to_byte_array());
    let sig = secp.sign_ecdsa(&message, &secret_key);

    let client = ProverClient::from_env();
    let mut stdin = SP1Stdin::new();
    stdin.write(&sig.serialize_compact().to_vec());
    stdin.write(message.as_ref());
    stdin.write(&public_key.serialize().to_vec());

    let (_output, report) = client.execute(FIBONACCI_ELF, stdin).run().unwrap();
    println!("Exit code: {}", report.exit_code);
}
