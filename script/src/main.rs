//! A simple script to generate and verify the proof of a given program.

use serde::{Deserialize, Serialize};
use sp1_core::utils::setup_logger;
use sp1_prover::utils::get_cycles;
use sp1_sdk::{ProverClient, SP1ProofWithPublicValues, SP1Stdin};
use tlsn_substrings_verifier::{
    self,
    proof::{SessionHeader, SubstringsProof},
};

#[derive(Serialize, Deserialize, Debug)]
struct ZkParam {
    header: SessionHeader,
    substrings: SubstringsProof,
}

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let proof_params = std::fs::read_to_string("../inputs/zk_params.json").unwrap();
    let proof_params: ZkParam = serde_json::from_str(proof_params.as_str()).unwrap();

    let (input1, input2) = (
        serde_json::to_string(&proof_params.header).unwrap(),
        serde_json::to_string(&proof_params.substrings).unwrap(),
    );

    let input3 = std::fs::read_to_string("../inputs/random_values.json").unwrap();

    stdin.write(&input1);
    stdin.write(&input2);
    stdin.write(&input3);

    setup_logger();

    // let cycles = get_cycles(&ELF, &stdin);
    // println!("Total cycles : {}", cycles);

    let client = ProverClient::new();
    // let mut public_values = client.execute(ELF, stdin).unwrap();
    let (pk, vk) = client.setup(ELF);
    let proof = client.prove_groth16(&pk, stdin).expect("proving failed");

    // Read output.
    // let c: String = public_values.read::<String>();
    // println!("req: {}", c);
    println!("successfully executed the program!");

    // Verify proof.
    client
        .verify_groth16(&proof, &vk)
        .expect("verification failed");

    // // Save proof.
    proof
        .save("groth16-proof-with-pis.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    #[test]
    fn load_proof_file() {
        let proof = SP1ProofWithPublicValues::load(Path::new("proof-with-io.json")).unwrap();
        let client = ProverClient::new();
        let (_pk, vk) = client.setup(ELF);
        client.verify(&proof, &vk).expect("verification failed");
    }
}
