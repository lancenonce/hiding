use ark_ec::PairingEngine;
use ark_bn254::{Bn254, G1Projective};
use ark_poly::{univariate::DensePolynomial, UVPolynomial};
use ark_poly::Polynomial;
use std::io;

fn main() {
    let poly = input_polynomial();

    let commitment = commit_polynomial::<Bn254>(&poly); // todo: implement this and remove the mock
    let mock_commitment = G1Projective::new(
        ark_bn254::Fq::from(0),
        ark_bn254::Fq::from(0),
        ark_bn254::Fq::from(0),
    );
    // todo: the evaluation point should be taken from a different terminal process / thread
    let eval_point = input_evaluation_point();

    let evaluation = prove_evaluation::<Bn254>(&poly, eval_point);

    let is_valid = verify_proof::<Bn254>(mock_commitment, eval_point, evaluation);

    if is_valid {
        println!("The sample passed!");
    } else {
        println!("The sample failed!");
    }

    println!("The polynomial was evaluated at point: {}", eval_point);
    println!("The evaluation result is: {}", evaluation);
}

fn input_polynomial() -> DensePolynomial<ark_bn254::Fr> {
    let mut coeffs = Vec::new();
    println!("Enter the polynomial coefficients (u64), separated by spaces:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid u64 numbers"))
        .collect();

    for coeff in inputs {
        coeffs.push(ark_bn254::Fr::from(coeff));
    }

    DensePolynomial::from_coefficients_vec(coeffs)
}

fn input_evaluation_point() -> ark_bn254::Fr {
    println!("Verifier, enter the point you want to secretly evaluate at (u64):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let eval_point: u64 = input.trim().parse().expect("Please enter a valid u64 number");

    ark_bn254::Fr::from(eval_point)
}

fn commit_polynomial<E: PairingEngine>(poly: &DensePolynomial<E::Fr>)  {
    // todo... should return a G1Projective
}

fn prove_evaluation<E: PairingEngine>(poly: &DensePolynomial<E::Fr>, point: E::Fr) -> E::Fr {
    poly.evaluate(&point)
}

fn verify_proof<E: PairingEngine>(commitment: E::G1Projective, point: E::Fr, evaluation: E::Fr) -> bool {
    // todo: implement true verification
    true
}