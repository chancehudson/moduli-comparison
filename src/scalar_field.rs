use num_bigint::BigUint;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Clone)]
struct ScalarField {
    prime: BigUint,
    value: BigUint,
}

impl ScalarField {
    pub fn new(value: &BigUint, prime: &BigUint) -> Self {
        Self {
            prime: prime.clone(),
            value: value.clone(),
        }
    }
}

impl Add for ScalarField {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Primes must be equal");
        }
        Self {
            prime: self.prime.clone(),
            value: (self.value + other.value) % self.prime,
        }
    }
}

impl Mul for ScalarField {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Primes must be equal");
        }
        Self {
            prime: self.prime.clone(),
            value: (self.value * other.value) % self.prime,
        }
    }
}

impl Sub for ScalarField {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Primes must be equal");
        }
        Self {
            prime: self.prime.clone(),
            // do an addition because i'm too lazy to test negation modulus
            value: (self.value - (other.value + self.prime.clone())) % self.prime,
        }
    }
}
