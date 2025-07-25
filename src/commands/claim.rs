use anyhow::{Result, anyhow, bail};
use ark_circom::{CircomReduction};
use ark_groth16::{ Groth16};
use ark_bn254::{Bn254};
use ark_crypto_primitives::snark::SNARK;
use hex;

use zktool::verifier_utils::{load_verification_data,
    read_string, read_bytes,
    EMAIL_ADDRESS_OFFSET,
    PUBKEY_HASH_OFFSET, PUBKEY_HASH_COUNT,
    COMMAND_OFFSET};

// E.g., Claim ENS name for address 0xafBD210c60dD651892a61804A989eEF7bD63CBA0 with resolver resolver.eth 
pub fn exec(verification_key: &str, proof: &str, inp: &str, dkim_pk: &str, address: &str, resolver: &str, email: &str) -> Result<()> {
    let (vkey, proof, public_inputs) = load_verification_data(
        verification_key, proof, inp)?;

    // Verify given email matches that in public inputs
    let expected_email = read_string(&public_inputs, EMAIL_ADDRESS_OFFSET, 12)?;
    if expected_email != email {
        bail!("Emails do not match");
    }

    let expected_command = read_string(&public_inputs,COMMAND_OFFSET, 20)?;
    let actual_command = format!("Claim ENS name for address {address} with resolver {resolver}");
    if expected_command != actual_command {
        bail!("Commands do not much");
    }
 
    let expected_pubkeyhash = hex::encode(read_bytes(&public_inputs,PUBKEY_HASH_OFFSET, PUBKEY_HASH_COUNT)?);

    if expected_pubkeyhash != dkim_pk {
        bail!("Mismatching pubkey hash");
    }

    let verification_result = Groth16::<Bn254, CircomReduction>::verify(&vkey, &public_inputs, &proof)
        .map_err(|e| anyhow::anyhow!("An error ocurred while verifying the proof {}", e))?;

    match verification_result {
        false => Err(anyhow!("Proof verification failed")),
        true => Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_claim_proof_verification_ok() -> Result<()> {
        exec("fixtures/vkey.json",
            "fixtures/proof.json",
            "fixtures/public.json",
            "0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788",
            "0xafBD210c60dD651892a61804A989eEF7bD63CBA0",
            "resolver.eth",
            "thezdev1@gmail.com")
    }

    #[test]
    #[should_panic]
    fn test_claim_proof_verification_wrong_resolver() {
        exec("fixtures/vkey.json",
            "fixtures/proof.json",
            "fixtures/public.json",
            "0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788",
            "0xafBD210c60dD651892a61804A989eEF7bD63CBA0",
            "resolver2.eth",
            "thezdev1@gmail.com").unwrap()
    }

    #[test]
    #[should_panic]
    fn test_claim_proof_verification_wrong_addr() {
        exec("fixtures/vkey.json",
            "fixtures/proof.json",
            "fixtures/public.json",
            "0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788",
            "0xafBD210c60dD651892a61804A989eEF7bD63CBA1",
            "resolver2.eth",
            "thezdev1@gmail.com").unwrap()
    }
}
