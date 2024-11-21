use super::IntegerAU;

pub struct Montgomery {
    r: IntegerAU,
    r_bitmask: IntegerAU,
    r_bits: usize,
    n_prime: IntegerAU,
    prime: IntegerAU,
}

impl Montgomery {
    pub fn new(prime: &IntegerAU) -> Self {
        let r = &IntegerAU::from(1) << prime.bit_len();
        let r_minus_prime = (&r - &prime).unwrap();
        let n_prime =
            IntegerAU::from_biguint(r_minus_prime.to_biguint().modinv(&r.to_biguint()).unwrap());
        // let n_prime = (prime_inv_r + r.clone()) % r.clone();
        Self {
            r_bitmask: (&r - &IntegerAU::from(1)).unwrap(),
            r_bits: prime.bit_len(),
            r,
            n_prime,
            prime: prime.clone(),
        }
    }

    pub fn to_mont(&self, v: &IntegerAU) -> IntegerAU {
        (v << self.r_bits) % self.prime.clone()
    }

    pub fn from_mont(&self, v: &IntegerAU) -> IntegerAU {
        self.redc(v)
    }

    pub fn reduce_naive(&self, v: IntegerAU) -> IntegerAU {
        let mut out = v;
        while out >= self.prime {
            out = (&out - &self.prime).unwrap();
        }
        out
    }

    pub fn redc(&self, v: &IntegerAU) -> IntegerAU {
        let m = &(&(v & &self.r_bitmask) * &self.n_prime) & &self.r_bitmask;
        let t = &(v + &(&m * &self.prime)) >> self.r_bits;
        if t >= self.prime {
            (&t - &self.prime).unwrap()
        } else {
            t
        }
    }
}
