// File mostly obtained from
// https://github.com/zkemail/zk-email-verify/blob/main/packages/rust-verifier/src/verifier_utils.rs

use ark_bn254::Bn254;
use ark_bn254::Config;
use ark_bn254::Fq2;
use ark_bn254::FrConfig;
use ark_bn254::G1Affine;
use ark_bn254::G2Affine;
use ark_circom::CircomReduction;
use ark_ec::bn::Bn;
use ark_ff::Fp;
use ark_ff::MontBackend;
use ark_groth16::Groth16;
use ark_groth16::Proof;
use ark_groth16::VerifyingKey;
use ark_serialize::CanonicalSerialize;
use primitive_types::U256;
use serde::Deserialize;
use std::fs;
use std::ops::Deref;
use std::str::FromStr;
use anyhow::{anyhow, Result, Context};

pub const COMMAND_FIELDS: usize = 20;
pub const COMMAND_OFFSET:usize = 12;
pub const PUBKEY_HASH_OFFSET:usize = 9;
pub const PUBKEY_HASH_COUNT:usize = 1;
// #9: email_address CEIL(256 bytes / 31 bytes per field) = 9 fields -> idx 51-59
pub const EMAIL_ADDRESS_OFFSET:usize = 51;

pub type GrothBn = Groth16<Bn254, CircomReduction>;
pub type GrothBnProof = Proof<Bn<Config>>;
pub type GrothBnVkey = VerifyingKey<Bn254>;
pub type GrothFp = Fp<MontBackend<FrConfig, 4>, 4>;

#[derive(Debug, Deserialize)]
struct SnarkJsProof {
    pi_a: [String; 3],
    pi_b: [[String; 2]; 3],
    pi_c: [String; 3],
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct SnarkJsVkey {
    vk_alpha_1: [String; 3],
    vk_beta_2: [[String; 2]; 3],
    vk_gamma_2: [[String; 2]; 3],
    vk_delta_2: [[String; 2]; 3],
    IC: Vec<[String; 3]>,
}

#[derive(Debug)]
pub struct PublicInputs<const N: usize> {
    pub inputs: [GrothFp; N],
}

// helper struct for deserializing public inputs count
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct PublicInputsCount {
    pub nPublic: usize,
}

pub trait JsonDecoder {
    fn from_json(json: &str) -> Result<Self>
    where
        Self: Sized;
    fn from_json_file(file_path: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let json = fs::read_to_string(file_path)?;
        Self::from_json(&json)
    }
}

impl JsonDecoder for GrothBnProof {
    fn from_json(json: &str) -> Result<Self> {
        let snarkjs_proof: SnarkJsProof = serde_json::from_str(json)?;
        let a = G1Affine {
            x: Fp::from_str(snarkjs_proof.pi_a[0].as_str()).unwrap(),
            y: Fp::from_str(snarkjs_proof.pi_a[1].as_str()).unwrap(),
            infinity: false,
        };
        let b = G2Affine {
            x: Fq2::new(
                Fp::from_str(snarkjs_proof.pi_b[0][0].as_str()).unwrap(),
                Fp::from_str(snarkjs_proof.pi_b[0][1].as_str()).unwrap(),
            ),
            y: Fq2::new(
                Fp::from_str(snarkjs_proof.pi_b[1][0].as_str()).unwrap(),
                Fp::from_str(snarkjs_proof.pi_b[1][1].as_str()).unwrap(),
            ),
            infinity: false,
        };
        let c = G1Affine {
            x: Fp::from_str(snarkjs_proof.pi_c[0].as_str()).unwrap(),
            y: Fp::from_str(snarkjs_proof.pi_c[1].as_str()).unwrap(),
            infinity: false,
        };
        Ok(Proof { a, b, c })
    }
}

impl JsonDecoder for GrothBnVkey {
    fn from_json(json: &str) -> Result<Self> {
        let snarkjs_vkey: SnarkJsVkey = serde_json::from_str(json).unwrap();
        let vk_alpha_1 = G1Affine {
            x: Fp::from_str(snarkjs_vkey.vk_alpha_1[0].as_str()).unwrap(),
            y: Fp::from_str(snarkjs_vkey.vk_alpha_1[1].as_str()).unwrap(),
            infinity: false,
        };
        let vk_beta_2 = G2Affine {
            x: Fq2::new(
                Fp::from_str(snarkjs_vkey.vk_beta_2[0][0].as_str()).unwrap(),
                Fp::from_str(snarkjs_vkey.vk_beta_2[0][1].as_str()).unwrap(),
            ),
            y: Fq2::new(
                Fp::from_str(snarkjs_vkey.vk_beta_2[1][0].as_str()).unwrap(),
                Fp::from_str(snarkjs_vkey.vk_beta_2[1][1].as_str()).unwrap(),
            ),
            infinity: false,
        };
        let vk_gamma_2 = G2Affine {
            x: Fq2::new(
                Fp::from_str(snarkjs_vkey.vk_gamma_2[0][0].as_str()).unwrap(),
                Fp::from_str(snarkjs_vkey.vk_gamma_2[0][1].as_str()).unwrap(),
            ),
            y: Fq2::new(
                Fp::from_str(snarkjs_vkey.vk_gamma_2[1][0].as_str()).unwrap(),
                Fp::from_str(snarkjs_vkey.vk_gamma_2[1][1].as_str()).unwrap(),
            ),
            infinity: false,
        };
        let vk_delta_2 = G2Affine {
            x: Fq2::new(
                Fp::from_str(snarkjs_vkey.vk_delta_2[0][0].as_str()).unwrap(),
                Fp::from_str(snarkjs_vkey.vk_delta_2[0][1].as_str()).unwrap(),
            ),
            y: Fq2::new(
                Fp::from_str(snarkjs_vkey.vk_delta_2[1][0].as_str()).unwrap(),
                Fp::from_str(snarkjs_vkey.vk_delta_2[1][1].as_str()).unwrap(),
            ),
            infinity: false,
        };

        let ic = snarkjs_vkey
            .IC
            .iter()
            .map(|ic| G1Affine {
                x: Fp::from_str(ic[0].as_str()).unwrap(),
                y: Fp::from_str(ic[1].as_str()).unwrap(),
                infinity: false,
            })
            .collect();

        Ok(VerifyingKey {
            alpha_g1: vk_alpha_1,
            beta_g2: vk_beta_2,
            gamma_g2: vk_gamma_2,
            delta_g2: vk_delta_2,
            gamma_abc_g1: ic,
        })
    }
}

impl JsonDecoder for PublicInputsCount {
    fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json).unwrap())
    }
}

impl<const N: usize> JsonDecoder for PublicInputs<N> {
    fn from_json(json: &str) -> Result<Self> {
        let inputs: Vec<String> = serde_json::from_str(json).unwrap();
        let inputs: Vec<GrothFp> = inputs
            .iter()
            .map(|input| Fp::from_str(input).unwrap())
            .collect();
        Ok(Self {
            inputs: inputs.try_into().unwrap(),
        })
    }
}

impl<const N: usize> PublicInputs<N> {
    pub fn from(inputs: [&str; N]) -> Self {
        let inputs: Vec<GrothFp> = inputs
            .iter()
            .map(|input| Fp::from_str(input).unwrap())
            .collect();
        Self {
            inputs: inputs.try_into().unwrap(),
        }
    }
}

impl<const N: usize> Deref for PublicInputs<N> {
    type Target = [GrothFp];

    fn deref(&self) -> &Self::Target {
        &self.inputs
    }
}

impl<const N: usize> CanonicalSerialize for PublicInputs<N> {
    fn serialize_with_mode<W: ark_serialize::Write>(
        &self,
        writer: W,
        compress: ark_serialize::Compress,
    ) -> Result<(), ark_serialize::SerializationError> {
        self.inputs.serialize_with_mode(writer, compress)
    }

    fn serialized_size(&self, compress: ark_serialize::Compress) -> usize {
        self.inputs.serialized_size(compress)
    }
}

pub fn load_verification_data(path_vkey: &str, path_proof: &str, path_inputs: &str) -> Result<(GrothBnVkey, GrothBnProof, PublicInputs<60>)>{
    let vkey = GrothBnVkey::from_json_file(path_vkey)
        .with_context(|| format!("Reading vkey file: {path_vkey}"))?; 
    let proof = GrothBnProof::from_json_file(path_proof)
        .with_context(|| format!("Reading proof json file: {path_proof}"))?;
    let public_inputs: PublicInputs<60> = PublicInputs::from_json_file(path_inputs)
        .with_context(|| format!("Reading pub inputs file: {path_inputs}"))?;

    Ok((vkey, proof, public_inputs))
}

pub fn read_string(public_inputs: &PublicInputs<60>,
    start_idx: usize, count_fields: usize) -> Result<String> {
    let command_bytes: Vec<u8> = public_inputs
        .iter()
        .skip(start_idx)
        .take(count_fields)
        .map(|item| U256::from_str_radix(&item.to_string(), 10).unwrap())
        .flat_map(|item| item.to_little_endian())
        .collect();

    // Bytes to string, removing null bytes
    let command = String::from_utf8(command_bytes.into_iter().filter(|&b| b != 0u8).collect())
        .map_err(|e| anyhow!("Failed to convert bytes to string: {}", e))?;

    Ok(command)
}

pub fn read_bytes(public_inputs: &PublicInputs<60>,
    start_idx: usize, count_fields: usize) -> Result<Vec<u8>> {
    let command_bytes: Vec<u8> = public_inputs
        .iter()
        .skip(start_idx)
        .take(count_fields)
        .map(|item| U256::from_str_radix(&item.to_string(), 10).unwrap())
        .flat_map(|item| item.to_big_endian())
        .collect();
    // Bytes to string, removing null bytes
    Ok(command_bytes)
}
