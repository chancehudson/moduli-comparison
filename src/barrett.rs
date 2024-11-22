use super::integer_au::IntegerAU;

pub struct Barrett {
    prime: IntegerAU,
    prime_bit_length: usize,
    barrett_mu: IntegerAU,
}

impl Barrett {
    pub fn new(prime: IntegerAU) -> Self {
        let barrett_mu = &(IntegerAU::from(1u64) << (2 * prime.bit_len())) / &prime;
        Self {
            prime: prime.clone(),
            prime_bit_length: prime.bit_len(),
            barrett_mu,
        }
    }

    pub fn reduce(&self, x: &IntegerAU) -> IntegerAU {
        let q = &(&(x >> self.prime_bit_length) * &self.barrett_mu) >> self.prime_bit_length;
        let mut r = x - &(&q * &self.prime);
        while r >= self.prime {
            r = &r - &self.prime;
        }
        r
    }
}
