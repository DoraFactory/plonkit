#![allow(clippy::unit_arg)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate hex_literal;
extern crate bellman_vk_codegen;
extern crate byteorder;
extern crate franklin_crypto;
extern crate itertools;
extern crate num_bigint;
extern crate num_traits;
extern crate rand;

pub mod circom_circuit;
pub mod plonk;
pub mod r1cs_file;
pub mod reader;
pub mod recursive;
pub mod transpile;
pub mod utils;

pub use franklin_crypto::bellman as bellman_ce;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProofStr {
    pub num_inputs: usize,
    pub n: usize,
    pub input_values: Vec<String>,
    pub wire_commitments: Vec<String>,
    pub grand_product_commitment: String,
    pub quotient_poly_commitments: Vec<String>,
    pub wire_values_at_z: Vec<String>,
    pub wire_values_at_z_omega: Vec<String>,
    pub grand_product_at_z_omega: String,
    pub quotient_polynomial_at_z: String,
    pub linearization_polynomial_at_z: String,
    pub permutation_polynomials_at_z: Vec<String>,
    pub opening_at_z_proof: String,
    pub opening_at_z_omega_proof: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerificationKeyStr {
    pub n: usize,
    pub num_inputs: usize,
    pub selector_commitments: Vec<String>,
    pub next_step_selector_commitments: Vec<String>,
    pub permutation_commitments: Vec<String>,
    pub non_residues: Vec<String>,
    pub g2_elements: Vec<String>,
}