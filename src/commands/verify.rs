use anyhow::{Result, anyhow};
use ark_circom::{CircomReduction};
use ark_groth16::{ Groth16};
use ark_bn254::{Bn254};
use ark_crypto_primitives::snark::SNARK;
use zktool::verifier_utils::load_verification_data;


pub fn exec(verification_key: &str, proof: &str, inp: &str) -> Result<()> {
    let (vkey, proof, public_inputs) = load_verification_data(
        verification_key, proof, inp)?;

    let verification_result = Groth16::<Bn254, CircomReduction>::verify(&vkey, &public_inputs, &proof)
        .map_err(|e| anyhow::anyhow!("An error ocurred while verifying the proof {}", e))?;

    match verification_result {
        false => Err(anyhow!("Proof verification failed")),
        true => Ok(())
    }
}
