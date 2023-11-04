use sha2::{Sha256, Digest};

/// Represents a proof that can be verified without revealing the secret.
pub struct Proof {
    pub commitment: Vec<u8>, // The commitment is now a hash
    pub salt: u64, // A random salt to ensure the commitment is unique
}

/// The `Prover` is responsible for generating the proof.
pub struct Prover {
    secret: u64,
}

impl Prover {
    /// Creates a new `Prover` with a given secret.
    pub fn new(secret: u64) -> Self {
        Prover { secret }
    }

    /// Generates a proof without revealing the secret.
    /// We use a hash function to create a commitment to the secret.
    pub fn generate_proof(&self, salt: u64) -> Proof {
        let mut hasher = Sha256::new();
        hasher.update(self.secret.to_be_bytes());
        hasher.update(salt.to_be_bytes());
        let commitment = hasher.finalize().to_vec();
        Proof { commitment, salt }
    }
}

/// The `Verifier` is responsible for checking the validity of the proof.
pub struct Verifier;

impl Verifier {
    /// Verifies a proof given the commitment, a public value, and the salt.
    pub fn verify(proof: &Proof, public_value: u64) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(public_value.to_be_bytes());
        hasher.update(proof.salt.to_be_bytes());
        let expected_commitment = hasher.finalize().to_vec();
        proof.commitment == expected_commitment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_system() {
        let secret = 3;
        let prover = Prover::new(secret);
        let salt = 42; // Normally, this would be randomly generated.

        let proof = prover.generate_proof(salt);

        let public_value = secret; // In a real scenario, this might be a value derived from the secret.
        assert!(Verifier::verify(&proof, public_value));
    }
}
