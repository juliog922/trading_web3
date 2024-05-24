use rand::Rng;
use num_bigint::{BigUint, RandBigInt};

/// Structure representing Zero-Knowledge Proof parameters.
pub struct ZKP {
    pub p: BigUint,
    pub q: BigUint,
    pub alpha: BigUint,
    pub beta: BigUint,
}

impl ZKP {
    /// Computes a pair (alpha^exp mod p, beta^exp mod p).
    ///
    /// # Arguments
    ///
    /// * `alpha` - The alpha value.
    /// * `beta` - The beta value.
    /// * `p` - The modulus.
    /// * `password` - The password.
    ///
    /// # Returns
    ///
    /// A tuple containing the computed values.
    pub fn compute_pair(alpha: &BigUint, beta: &BigUint, p: &BigUint, password: &BigUint) -> (BigUint, BigUint) {
        let p1 = alpha.modpow(password, &p);
        let p2 = beta.modpow(password, &p);
        (p1, p2)
    }

    /// Solves the equation s = k - c * x mod q.
    ///
    /// # Arguments
    ///
    /// * `k` - The k value.
    /// * `c` - The c value.
    /// * `x` - The x value.
    ///
    /// # Returns
    ///
    /// The computed solution.
    pub fn solve(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
        if *k >= c * x {
            return (k - c * x).modpow(&BigUint::from(1u32), &self.q);
        }
        &self.q - (c * x - k).modpow(&BigUint::from(1u32), &self.q)
    }

    /// Verifies the validity of zero-knowledge proofs.
    ///
    /// # Arguments
    ///
    /// * `r1` - The r1 value.
    /// * `r2` - The r2 value.
    /// * `y1` - The y1 value.
    /// * `y2` - The y2 value.
    /// * `c` - The c value.
    /// * `s` - The s value.
    ///
    /// # Returns
    ///
    /// `true` if the proofs are valid, `false` otherwise
    pub fn verify(
        &self,
        r1: &BigUint,
        r2: &BigUint,
        y1: &BigUint,
        y2: &BigUint,
        c: &BigUint,
        s: &BigUint,
    ) -> bool {
        let cond1 = *r1
            == (&self.alpha.modpow(s, &self.p) * y1.modpow(c, &self.p))
                .modpow(&BigUint::from(1u32), &self.p);

        let cond2 = *r2
            == (&self.beta.modpow(s, &self.p) * y2.modpow(c, &self.p))
                .modpow(&BigUint::from(1u32), &self.p);

        cond1 && cond2
    }

    /// Generates a random number below a given bound.
    ///
    /// # Arguments
    ///
    /// * `bound` - The upper bound.
    ///
    /// # Returns
    ///
    /// The generated random number.
    pub fn generate_random_number_below(bound: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();

        rng.gen_biguint_below(bound)
    }

    /// Generates a random string of a given size.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the string.
    ///
    /// # Returns
    ///
    /// The generated random string.
    pub fn generate_random_string(size: usize) -> String {
        rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(size)
            .map(char::from)
            .collect()
    }

    /// Retrieves the constants used in the ZKP algorithm.
    ///
    /// # Returns
    ///
    /// A tuple containing the constants (alpha, beta, p, q).
    pub fn get_constants() -> (BigUint, BigUint, BigUint, BigUint) {
        let p = BigUint::from_bytes_be(&hex::decode("B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371").unwrap());
        let q = BigUint::from_bytes_be(
            &hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap(),
        );

        let alpha = BigUint::from_bytes_be(
            &hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").unwrap(),
        );

        // beta = alpha^i is also a generator
        let exp = BigUint::from_bytes_be(&hex::decode("266FEA1E5C41564B777E69").unwrap());
        let beta = alpha.modpow(&exp, &p);

        (alpha, beta, p, q)
    }
}