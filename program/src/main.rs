//! A simple program to be proven inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use tlsn_substrings_verifier::proof::{SessionHeader, SubstringsProof};

pub fn main() {
    let a = sp1_zkvm::io::read::<String>();
    let b = sp1_zkvm::io::read::<String>();

    let session_header: SessionHeader = serde_json::from_str(a.as_str()).unwrap();
    let substrings: SubstringsProof = serde_json::from_str(b.as_str()).unwrap();

    let (sent, recv) = substrings.verify(&session_header).unwrap();

    let is_req = !sent.data().to_vec().is_empty();
    let is_res = !recv.data().to_vec().is_empty();

    sp1_zkvm::io::commit(&is_req);
    // sp1_zkvm::io::commit(&"done".to_string());
}
