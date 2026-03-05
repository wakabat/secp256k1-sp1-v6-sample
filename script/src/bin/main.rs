use secp256k1::rand::{rngs::OsRng, RngCore};
use secp256k1::{Message, Secp256k1};
use sp1_sdk::{
    blocking::{Prover, ProverClient},
    include_elf, Elf, SP1Stdin,
};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
const FIBONACCI_ELF: Elf = include_elf!("fibonacci-program");

const COUNT: u64 = 100;

fn main() {
    sp1_sdk::utils::setup_logger();

    let secp = Secp256k1::new();
    let mut stdin = SP1Stdin::new();
    stdin.write(&COUNT);

    for _ in 0..COUNT {
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        let digest = {
            let mut digest = [0u8; 32];
            OsRng.fill_bytes(&mut digest);
            digest
        };
        let message = Message::from_digest(digest);
        let sig = secp.sign_ecdsa(&message, &secret_key);

        stdin.write(&sig.serialize_compact().to_vec());
        stdin.write(message.as_ref());
        stdin.write(&public_key.serialize().to_vec());
    }

    let client = ProverClient::from_env();

    {
        let data = bincode::serialize(&stdin).expect("serialize");
        let target = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("input.bin");
        std::fs::write(target, data).expect("write");
    }

    let (_output, report) = client.execute(FIBONACCI_ELF, stdin).run().unwrap();
    println!("Exit report: {report}");
}
