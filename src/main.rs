use zkp_demo::{Prover, Verifier};

fn main() {
    let secret = 3; // The secret is known only to the prover.
    let salt = 42; // Normally, this would be randomly generated.

    println!("Zero-Knowledge Proof Demonstration with Hashing");
    println!("------------------------------------------------");
    println!("Prover knows a secret: {}", secret);

    let prover = Prover::new(secret);
    let proof = prover.generate_proof(salt);
    println!("Prover generates a proof without revealing the secret.");

    let public_value = secret; // In a real scenario, this would be a known value related to the secret.
    let verified = Verifier::verify(&proof, public_value);

    if verified {
        println!("Verifier has verified the proof successfully without knowing the secret.");
    } else {
        println!("Verification failed.");
    }
}
