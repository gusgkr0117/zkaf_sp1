use serde::{Deserialize, Serialize};
use sp1_helper::build_program;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use mpz_circuits::types::ValueType;
use mpz_core::hash::Hash;
use mpz_garble_core::{encoding, EncodedValue, Encoder};
use p256::pkcs8::DecodePublicKey;
use std::str;
use tlsn_core::proof::{SessionProof, TlsProof};
use tlsn_substrings_verifier::proof::SessionHeader;
use tlsn_substrings_verifier::proof::{SubstringsProof, SubstringsProofError};
use utils::range::{RangeDisjoint, RangeSet, RangeUnion, ToRangeSet};

#[derive(Serialize, Deserialize)]
struct ZkParam {
    header: SessionHeader,
    substrings: SubstringsProof,
}

fn precompute_random_values(
    header: &SessionHeader,
    substrings: &SubstringsProof,
) -> Result<(), SubstringsProofError> {
    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let random_values_path = "../inputs/random_values.bin";
    let path = base_path.join(random_values_path);
    let mut file = File::create(path).unwrap();

    let encoding_list = substrings.clone().extract_random_values(header).unwrap();
    let test = bincode::serialize(&encoding_list).unwrap();

    file.write_all(&test).unwrap();
    Ok(())
}

fn build_proof() -> Result<(), Box<dyn std::error::Error>> {
    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let proof = std::fs::read_to_string(base_path.join("fixtures/twitter_proof.json")).unwrap();
    let proof: TlsProof = serde_json::from_str(proof.as_str()).unwrap();

    let TlsProof {
        // The session proof establishes the identity of the server and the commitments
        // to the TLS transcript.
        session,
        // The substrings proof proves select portions of the transcript, while redacting
        // anything the Prover chose not to disclose.
        substrings,
    } = proof;

    // Verify the session proof against the Notary's public key
    //
    // This verifies the identity of the server using a default certificate verifier which trusts
    // the root certificates from the `webpki-roots` crate.
    session
        .verify_with_default_cert_verifier(notary_pubkey())
        .unwrap();

    let SessionProof {
        // The session header that was signed by the Notary is a succinct commitment to the TLS transcript.
        header,
        ..
    } = session;

    let dummy = serde_json::to_string(&substrings).unwrap();
    let my_substrings: SubstringsProof = serde_json::from_str(dummy.as_str()).unwrap();

    let dummy = serde_json::to_string(&header).unwrap();
    let my_header: SessionHeader = serde_json::from_str(dummy.as_str()).unwrap();

    precompute_random_values(&my_header, &my_substrings)?;

    // type conversion occurs here
    // we need to convert from the tlsn core definitions to the definitions from the verifier
    let params = ZkParam {
        header: my_header,
        substrings: my_substrings,
    };

    let json = serde_json::to_string(&params)?;

    let file_path = "../inputs/zk_params.json";
    let path = base_path.join(file_path);
    // Check if the parent directory exists, and create it if it does not.
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    // Open the file in write mode. This will create the file if it does not exist.
    let mut file = File::create(path)?;
    // Write content to the file.
    file.write_all(json.as_bytes())?;
    Ok(())
}

/// Returns a Notary pubkey trusted by this Verifier
fn notary_pubkey() -> p256::PublicKey {
    let pem_file = str::from_utf8(include_bytes!("./fixtures/notary.pub")).unwrap();
    p256::PublicKey::from_public_key_pem(pem_file).unwrap()
}

fn main() {
    let _ = build_proof();
    build_program("../program")
}
