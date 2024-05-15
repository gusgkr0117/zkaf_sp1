//! A simple program to be proven inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use mpz_garble_core::{encoding, encoding_state::Full, EncodedValue, Encoder};
use tlsn_substrings_verifier::proof::{SessionHeader, SubstringsProof};

pub fn main() {
    let a = sp1_zkvm::io::read::<String>();
    let b = sp1_zkvm::io::read::<String>();
    let c = sp1_zkvm::io::read::<String>();

    let session_header: SessionHeader = serde_json::from_str(a.as_str()).unwrap();
    let substrings: SubstringsProof = serde_json::from_str(b.as_str()).unwrap();
    let encodings_list: Vec<Vec<EncodedValue<Full>>> = serde_json::from_str(c.as_str()).unwrap();

    let _ = substrings
        .verify_with_precompute(&session_header, encodings_list)
        .unwrap();

    // let is_req = !sent.data().to_vec().is_empty();
    // let is_res = !recv.data().to_vec().is_empty();

    // sp1_zkvm::io::commit(&is_req);
    sp1_zkvm::io::commit(&"done".to_string());
}
