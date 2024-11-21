use num_bigint::BigUint;

use super::integer_au::IntegerAU;

pub struct Barrett {
    prime: IntegerAU,
    prime_bit_length: usize,
    barret_mu: IntegerAU,
}

impl Barrett {
    pub fn new(prime: IntegerAU) -> Self {
        let barrett_mu = BigUint::from(2u64).pow(2 * u32::try_from(prime.bit_len()).unwrap())
            / prime.clone().to_biguint();
        Self {
            prime: prime.clone(),
            prime_bit_length: prime.bit_len(),
            barret_mu: IntegerAU::from_biguint(barrett_mu),
        }
    }

    pub fn reduce(&self, x: IntegerAU) -> IntegerAU {
        let q = (&(x.clone() >> self.prime_bit_length) * &self.barret_mu) >> self.prime_bit_length;
        let mut r = (x - (&q * &self.prime)).unwrap();
        while r >= self.prime {
            r = (r - self.prime.clone()).unwrap();
        }
        r
    }
}
