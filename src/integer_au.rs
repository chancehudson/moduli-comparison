use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Shl;
use std::ops::Shr;
use std::ops::Sub;

use num_bigint::BigUint;
use rand::Rng;

/// Arbitrary precision unsigned integer using 64 bit limbs
/// All operations are done naively
#[derive(Debug, Clone)]
pub struct IntegerAU {
    pub limbs: Vec<u64>,
}

impl IntegerAU {
    /// Samples a random value uniformly between 0 and upper (exclusive)
    /// Returns None if upper is zero
    pub fn random_below(upper: &Self) -> Self {
        // Handle zero upper bound
        if upper.limbs.len() == 1 && upper.limbs[0] == 0 {
            panic!("Upper bound must be non-zero")
        }

        let mut rng = rand::thread_rng();
        let bit_len = upper.bit_len();

        // We'll use rejection sampling to get uniform distribution
        loop {
            // Calculate how many limbs we need
            let num_limbs = (bit_len + 63) / 64;
            let mut limbs = Vec::with_capacity(num_limbs);

            // Generate random limbs
            for i in 0..num_limbs {
                let mut limb = rng.gen::<u64>();

                // For the most significant limb, mask off extra bits
                if i == num_limbs - 1 {
                    let extra_bits = (num_limbs * 64) - bit_len;
                    limb &= u64::MAX >> extra_bits;
                }
                limbs.push(limb);
            }

            // Remove leading zeros
            while limbs.len() > 1 && limbs[limbs.len() - 1] == 0 {
                limbs.pop();
            }

            // Create the random number
            let result = Self { limbs };

            // Check if it's within range
            if result < *upper {
                return result;
            }
            // If not, loop and try again
        }
    }

    pub fn to_biguint(&self) -> BigUint {
        let mut result = BigUint::from(0u64);
        for &limb in self.limbs.iter().rev() {
            result <<= 64;
            result += limb;
        }
        result
    }

    pub fn from_biguint(v: BigUint) -> Self {
        let limbs: Vec<u64> = v.to_u64_digits();
        IntegerAU {
            limbs: if limbs.is_empty() { vec![0] } else { limbs },
        }
    }

    /// Returns the number of bits needed to represent this number
    /// A zero value has bit length 0
    pub fn bit_len(&self) -> usize {
        // Handle zero case
        if self.limbs.len() == 1 && self.limbs[0] == 0 {
            return 0;
        }

        // Get the most significant non-zero limb
        let msb_limb = *self.limbs.last().unwrap();

        // Calculate bits from complete limbs
        let complete_limbs_bits = (self.limbs.len() - 1) * 64;

        // Add bits from the most significant limb
        // leading_zeros() returns u32, but we're working with usize
        let msb_bits = 64 - msb_limb.leading_zeros() as usize;

        complete_limbs_bits + msb_bits
    }

    /// Performs modular reduction self mod m
    /// Returns None if m is zero
    pub fn modulo(&self, m: &Self) -> Option<Self> {
        if m.limbs.len() == 1 && m.limbs[0] == 0 {
            return None; // Division by zero
        }

        // If self < m, return self directly
        if self < m {
            return Some(Self {
                limbs: self.limbs.clone(),
            });
        }

        let mut result = self.clone();

        // Compute largest multiple of m that's <= self
        let mut shifts = Vec::new();
        let mut current = m.clone();

        // Double until we exceed result
        while current <= result {
            shifts.push(current.clone());
            let mut next = current.clone();
            next = &next + &next;
            // If adding caused overflow or exceeded result, break
            if next > result {
                break;
            }
            current = next;
        }

        // Subtract from largest to smallest
        for shifted_m in shifts.iter().rev() {
            if shifted_m <= &result {
                result = &result - &shifted_m;
            }
        }

        Some(result)
    }

    // Helper function to compare two numbers
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.limbs.len() != other.limbs.len() {
            return self.limbs.len().cmp(&other.limbs.len());
        }

        // Compare limbs from most significant to least significant
        for i in (0..self.limbs.len()).rev() {
            if self.limbs[i] != other.limbs[i] {
                return self.limbs[i].cmp(&other.limbs[i]);
            }
        }
        std::cmp::Ordering::Equal
    }

    pub fn trim(mut self) -> Self {
        // Remove leading zeros
        while self.limbs.len() > 1 && self.limbs[self.limbs.len() - 1] == 0 {
            self.limbs.pop();
        }
        self
    }
}

impl From<u64> for IntegerAU {
    fn from(v: u64) -> Self {
        IntegerAU { limbs: vec![v] }
    }
}

impl Display for IntegerAU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_biguint())
    }
}

impl Rem for IntegerAU {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        self.modulo(&other).unwrap()
    }
}

// Also implement reference version to avoid moving values
impl<'a, 'b> Rem<&'b IntegerAU> for &'a IntegerAU {
    type Output = Option<IntegerAU>;

    fn rem(self, other: &'b IntegerAU) -> Self::Output {
        self.modulo(other)
    }
}

impl PartialEq for IntegerAU {
    fn eq(&self, other: &Self) -> bool {
        // First check lengths
        if self.limbs.len() != other.limbs.len() {
            return false;
        }

        // Compare all limbs
        self.limbs == other.limbs
    }
}

// PartialOrd requires PartialEq
impl PartialOrd for IntegerAU {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // First compare lengths
        if self.limbs.len() != other.limbs.len() {
            return Some(self.limbs.len().cmp(&other.limbs.len()));
        }

        // Compare limbs from most significant to least significant
        for i in (0..self.limbs.len()).rev() {
            if self.limbs[i] != other.limbs[i] {
                return Some(self.limbs[i].cmp(&other.limbs[i]));
            }
        }

        // Numbers are equal
        Some(std::cmp::Ordering::Equal)
    }
}

impl AddAssign<&IntegerAU> for IntegerAU {
    fn add_assign(&mut self, other: &IntegerAU) {
        *self = &*self + other;
    }
}

impl<'a, 'b> Add<&'b IntegerAU> for &'a IntegerAU {
    type Output = IntegerAU;

    fn add(self, other: &'b IntegerAU) -> IntegerAU {
        let max_len = std::cmp::max(self.limbs.len(), other.limbs.len());
        let mut result = Vec::with_capacity(max_len + 1);
        let mut carry = 0u64;

        for i in 0..max_len {
            let a = self.limbs.get(i).copied().unwrap_or(0);
            let b = other.limbs.get(i).copied().unwrap_or(0);

            // Add with carry
            let sum = a.wrapping_add(b).wrapping_add(carry);
            carry = if (sum < a) || (sum < b) || ((sum == a) && (carry == 1)) {
                1
            } else {
                0
            };
            result.push(sum);
        }

        // Push final carry if needed
        if carry > 0 {
            result.push(carry);
        }

        IntegerAU { limbs: result }.trim()
    }
}

impl<'a, 'b> Sub<&'b IntegerAU> for &'a IntegerAU {
    type Output = IntegerAU;

    fn sub(self, other: &'b IntegerAU) -> IntegerAU {
        if self.cmp(other) == std::cmp::Ordering::Less {
            panic!("Subtraction underflow");
        }

        let mut result = Vec::with_capacity(self.limbs.len());
        let mut borrow = false;

        for i in 0..self.limbs.len() {
            let mut a = self.limbs[i];
            let b = other.limbs.get(i).copied().unwrap_or(0);

            if borrow {
                if a == 0 {
                    a = u64::MAX;
                } else {
                    a -= 1;
                    borrow = false;
                }
            }

            if a >= b {
                result.push(a - b);
            } else {
                result.push(u64::MAX - (b - a - 1));
                borrow = true;
            }
        }

        IntegerAU { limbs: result }.trim()
    }
}

impl<'a, 'b> Mul<&'b IntegerAU> for &'a IntegerAU {
    type Output = IntegerAU;

    fn mul(self, other: &'b IntegerAU) -> IntegerAU {
        if self.limbs.is_empty() || other.limbs.is_empty() {
            return IntegerAU { limbs: vec![0] };
        }

        let m = self.limbs.len();
        let n = other.limbs.len();
        let mut result = vec![0u64; m + n];

        for i in 0..m {
            let mut carry = 0u64;
            for j in 0..n {
                let mut temp = result[i + j] as u128;
                temp += (self.limbs[i] as u128) * (other.limbs[j] as u128);
                temp += carry as u128;

                result[i + j] = temp as u64;
                carry = (temp >> 64) as u64;
            }

            if carry > 0 {
                result[i + n] = carry;
            }
        }

        IntegerAU { limbs: result }.trim()
    }
}

// Implement for owned values by delegating to reference implementation
impl Div for IntegerAU {
    type Output = IntegerAU;

    fn div(self, divisor: IntegerAU) -> IntegerAU {
        &self / &divisor
    }
}

impl<'a, 'b> Div<&'b IntegerAU> for &'a IntegerAU {
    type Output = IntegerAU;

    fn div(self, divisor: &'b IntegerAU) -> IntegerAU {
        // Check for division by zero
        if divisor.limbs.len() == 1 && divisor.limbs[0] == 0 {
            panic!("divide by 0");
        }

        // If dividend is smaller than divisor, return 0
        if self < divisor {
            return IntegerAU { limbs: vec![0] };
        }

        // If numbers are equal, return 1
        if self == divisor {
            return IntegerAU { limbs: vec![1] };
        }

        let mut quotient = IntegerAU { limbs: vec![0] };
        let mut remainder = self.clone();

        // Calculate initial shift needed
        let mut shifted_divisor = divisor.clone();
        let mut total_shifts = 0;

        while shifted_divisor <= remainder && total_shifts < remainder.bit_len() {
            shifted_divisor = &IntegerAU::from(2u64) * &shifted_divisor;
            total_shifts += 1;
        }

        // Adjust if we went one step too far
        if shifted_divisor > remainder {
            shifted_divisor = &shifted_divisor >> 1;
            total_shifts = total_shifts.saturating_sub(1);
        }

        // Start the division process
        for current_shift in (0..=total_shifts).rev() {
            if remainder >= shifted_divisor {
                // Subtract shifted divisor from remainder
                remainder = (&remainder - &shifted_divisor);
                // Set the corresponding bit in quotient
                quotient = quotient | (&IntegerAU::from(1u64) << current_shift);
            }
            shifted_divisor = &shifted_divisor >> 1;
        }

        quotient
    }
}

impl<'a, 'b> BitOr<&'b IntegerAU> for &'a IntegerAU {
    type Output = IntegerAU;

    fn bitor(self, other: &'b IntegerAU) -> IntegerAU {
        let max_len = std::cmp::max(self.limbs.len(), other.limbs.len());
        let mut result = Vec::with_capacity(max_len);

        for i in 0..max_len {
            let a = self.limbs.get(i).copied().unwrap_or(0);
            let b = other.limbs.get(i).copied().unwrap_or(0);
            result.push(a | b);
        }

        IntegerAU { limbs: result }.trim()
    }
}

// Implement for owned values
impl BitOr for IntegerAU {
    type Output = IntegerAU;

    fn bitor(self, other: IntegerAU) -> IntegerAU {
        &self | &other
    }
}

impl<'a> BitOr<IntegerAU> for &'a IntegerAU {
    type Output = IntegerAU;

    fn bitor(self, other: IntegerAU) -> IntegerAU {
        self | &other
    }
}

impl<'a> BitOr<&'a IntegerAU> for IntegerAU {
    type Output = IntegerAU;

    fn bitor(self, other: &'a IntegerAU) -> IntegerAU {
        &self | other
    }
}

// Bitwise AND
impl<'a, 'b> BitAnd<&'b IntegerAU> for &'a IntegerAU {
    type Output = IntegerAU;

    fn bitand(self, other: &'b IntegerAU) -> IntegerAU {
        let min_len = std::cmp::min(self.limbs.len(), other.limbs.len());
        let mut result = Vec::with_capacity(min_len);

        for i in 0..min_len {
            result.push(self.limbs[i] & other.limbs[i]);
        }

        IntegerAU { limbs: result }.trim()
    }
}

impl Shl<usize> for IntegerAU {
    type Output = IntegerAU;

    fn shl(self, shift: usize) -> IntegerAU {
        &self << shift
    }
}

// Left shift
impl<'a> Shl<usize> for &'a IntegerAU {
    type Output = IntegerAU;

    fn shl(self, shift: usize) -> IntegerAU {
        // Handle zero case
        if self.limbs.len() == 1 && self.limbs[0] == 0 {
            return IntegerAU { limbs: vec![0] };
        }

        let word_shifts = shift / 64;
        let bit_shifts = shift - word_shifts * 64;

        // Create result vector with enough space
        let mut result = vec![0u64; self.limbs.len() + word_shifts + 1];

        // Copy original number shifted by words
        for i in 0..self.limbs.len() {
            result[i + word_shifts] = self.limbs[i];
        }

        // Handle bit shifts
        if bit_shifts > 0 {
            let bit_shifts_diff = 64 - bit_shifts;
            let mut carry = 0u64;
            for i in word_shifts..result.len() {
                let new_carry = if i < result.len() - 1 {
                    result[i] >> bit_shifts_diff
                } else {
                    0
                };
                result[i] = (result[i] << bit_shifts) | carry;
                carry = new_carry;
            }
        }

        IntegerAU { limbs: result }.trim()
    }
}

impl Shr<usize> for IntegerAU {
    type Output = IntegerAU;

    fn shr(self, shift: usize) -> IntegerAU {
        &self >> shift
    }
}

// Right shift
impl<'a> Shr<usize> for &'a IntegerAU {
    type Output = IntegerAU;

    fn shr(self, shift: usize) -> IntegerAU {
        // Handle zero case
        if self.limbs.len() == 1 && self.limbs[0] == 0 {
            return IntegerAU { limbs: vec![0] };
        }

        let word_shifts = shift / 64;
        let bit_shifts = shift - word_shifts * 64;

        // If we're shifting by more than the number of words we have, return zero
        if word_shifts >= self.limbs.len() {
            return IntegerAU { limbs: vec![0] };
        }

        // Create result vector
        let result = if bit_shifts == 0 {
            // If only shifting by whole words, just truncate
            self.limbs[word_shifts..].to_vec()
        } else {
            let mut res = Vec::with_capacity(self.limbs.len() - word_shifts);
            let bit_shifts_diff = 64 - bit_shifts;
            for window in self.limbs[word_shifts..].windows(2) {
                res.push((window[0] >> bit_shifts) | (window[1] << bit_shifts_diff));
            }
            // Handle the last word
            if word_shifts < self.limbs.len() {
                res.push(self.limbs.last().unwrap() >> bit_shifts);
            }
            res
        };

        IntegerAU { limbs: result }.trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;
    use rand::Rng;
    use std::str::FromStr;

    #[test]
    fn test_addition() {
        let test_cases = vec![
            // Small numbers
            ("0", "0"),
            ("1", "1"),
            ("42", "58"),
            // Numbers that require carrying
            ("18446744073709551615", "1"), // 2^64 - 1 + 1
            ("18446744073709551615", "2"), // 2^64 - 1 + 2
            // Large numbers
            ("34893458934589345893458934", "89345893458934589345893458"),
            // Powers of 2
            ("18446744073709551616", "18446744073709551616"), // 2^64 + 2^64
        ];

        for (a_str, b_str) in test_cases {
            let a_big = BigUint::from_str(a_str).unwrap();
            let b_big = BigUint::from_str(b_str).unwrap();
            let expected = &a_big + &b_big;

            let a = IntegerAU::from_biguint(a_big);
            let b = IntegerAU::from_biguint(b_big);
            let result = &a + &b;

            assert_eq!(
                result.to_biguint(),
                expected,
                "Failed addition test: {} + {}",
                a_str,
                b_str
            );
        }
    }

    #[test]
    fn test_subtraction() {
        let test_cases = vec![
            // Small numbers
            ("1", "1"),
            ("42", "12"),
            ("100", "1"),
            // Numbers that require borrowing
            ("18446744073709551616", "1"), // 2^64 - 1
            ("18446744073709551616", "18446744073709551615"), // 2^64 - (2^64 - 1)
            // Large numbers
            ("89345893458934589345893458", "34893458934589345893458934"),
        ];

        for (a_str, b_str) in test_cases {
            let a_big = BigUint::from_str(a_str).unwrap();
            let b_big = BigUint::from_str(b_str).unwrap();

            // Only test if a >= b
            if a_big >= b_big {
                let expected = &a_big - &b_big;

                let a = IntegerAU::from_biguint(a_big);
                let b = IntegerAU::from_biguint(b_big);
                let result = (&a - &b);

                assert_eq!(
                    result.to_biguint(),
                    expected,
                    "Failed subtraction test: {} - {}",
                    a_str,
                    b_str
                );
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_subtraction_underflow() {
        let a = IntegerAU { limbs: vec![5] };
        let b = IntegerAU { limbs: vec![10] };
        let _ = &a - &b;
    }

    #[test]
    fn test_multiplication() {
        let test_cases = vec![
            // Small numbers
            ("0", "0"),
            ("1", "1"),
            ("42", "58"),
            // Powers of 2
            ("18446744073709551616", "2"), // 2^64 * 2
            // Large numbers requiring multiple limbs
            ("18446744073709551615", "18446744073709551615"),
            ("34893458934589345893458934", "2"),
            // Really large numbers
            ("34893458934589345893458934", "89345893458934589345893458"),
        ];

        for (a_str, b_str) in test_cases {
            let a_big = BigUint::from_str(a_str).unwrap();
            let b_big = BigUint::from_str(b_str).unwrap();
            let expected = &a_big * &b_big;

            let a = IntegerAU::from_biguint(a_big);
            let b = IntegerAU::from_biguint(b_big);
            let result = &a * &b;

            assert_eq!(
                result.to_biguint(),
                expected,
                "Failed multiplication test: {} * {}",
                a_str,
                b_str
            );
        }
    }

    #[test]
    fn test_ordering() {
        let test_cases = vec![
            ("0", "0", std::cmp::Ordering::Equal),
            ("1", "0", std::cmp::Ordering::Greater),
            ("0", "1", std::cmp::Ordering::Less),
            (
                "18446744073709551615",
                "18446744073709551616",
                std::cmp::Ordering::Less,
            ),
            (
                "34893458934589345893458934",
                "89345893458934589345893458",
                std::cmp::Ordering::Less,
            ),
            (
                "89345893458934589345893458",
                "34893458934589345893458934",
                std::cmp::Ordering::Greater,
            ),
        ];

        for (a_str, b_str, expected) in test_cases {
            let a_big = BigUint::from_str(a_str).unwrap();
            let b_big = BigUint::from_str(b_str).unwrap();

            let a = IntegerAU::from_biguint(a_big);
            let b = IntegerAU::from_biguint(b_big);

            assert_eq!(
                a.partial_cmp(&b),
                Some(expected),
                "Failed ordering test: {} vs {}",
                a_str,
                b_str
            );
        }
    }

    #[test]
    fn test_modulo() {
        let test_cases = vec![
            ("10", "3", "1"),
            ("7", "4", "3"),
            ("18446744073709551615", "18446744073709551614", "1"),
            ("18446744073709551615", "2", "1"),
            (
                "34893458934589345893458934",
                "89345893458934589345893458",
                "34893458934589345893458934",
            ),
        ];

        for (a_str, m_str, expected_str) in test_cases {
            let a_big = BigUint::from_str(a_str).unwrap();
            let m_big = BigUint::from_str(m_str).unwrap();
            let expected = BigUint::from_str(expected_str).unwrap();

            let a = IntegerAU::from_biguint(a_big);
            let m = IntegerAU::from_biguint(m_big);
            let result = a.modulo(&m).unwrap();

            assert_eq!(
                result.to_biguint(),
                expected,
                "Failed modulo test: {} mod {}",
                a_str,
                m_str
            );
        }
        // Random test cases
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            // Generate random number of limbs (1-4)
            let a_limbs = rng.gen_range(1..=4);
            let m_limbs = rng.gen_range(1..=4);

            // Generate random limbs
            let mut a_vec = Vec::with_capacity(a_limbs);
            let mut m_vec = Vec::with_capacity(m_limbs);

            for _ in 0..a_limbs {
                a_vec.push(rng.gen::<u64>());
            }
            for _ in 0..m_limbs {
                m_vec.push(rng.gen::<u64>());
            }

            // Ensure modulus is not zero by setting at least one bit
            if m_vec.iter().all(|&x| x == 0) {
                m_vec[0] = 1;
            }

            // Remove leading zeros
            while a_vec.len() > 1 && a_vec[a_vec.len() - 1] == 0 {
                a_vec.pop();
            }
            while m_vec.len() > 1 && m_vec[m_vec.len() - 1] == 0 {
                m_vec.pop();
            }

            let a = IntegerAU {
                limbs: a_vec.clone(),
            };
            let m = IntegerAU {
                limbs: m_vec.clone(),
            };

            // Convert to BigUint for comparison
            let mut a_big = BigUint::from(0u64);
            let mut m_big = BigUint::from(0u64);

            for &limb in a_vec.iter().rev() {
                a_big <<= 64;
                a_big += limb;
            }
            for &limb in m_vec.iter().rev() {
                m_big <<= 64;
                m_big += limb;
            }

            let expected = &a_big % &m_big;
            let result = a.modulo(&m).unwrap();

            assert_eq!(
                result.to_biguint(),
                expected,
                "Failed random modulo test: \na: {:?} \nm: {:?}",
                a_vec,
                m_vec
            );
        }
    }

    #[test]
    fn test_bitwise_operations() {
        let test_cases = vec![
            // (a, b) pairs as strings
            ("15", "7"),                                      // 1111 & 0111
            ("255", "15"),                                    // 11111111 & 00001111
            ("18446744073709551615", "18446744073709551614"), // 2^64 - 1, 2^64 - 2
        ];

        for (a_str, b_str) in test_cases {
            let a_big = BigUint::from_str(a_str).unwrap();
            let b_big = BigUint::from_str(b_str).unwrap();

            let a = IntegerAU::from_biguint(a_big.clone());
            let b = IntegerAU::from_biguint(b_big.clone());

            // Test AND
            let and_result = &a & &b;
            assert_eq!(
                and_result.to_biguint(),
                &a_big & &b_big,
                "Failed AND test: {} & {}",
                a_str,
                b_str
            );
        }
    }

    #[test]
    fn test_shifts() {
        let test_cases = vec![
            ("15", 2),                    // 1111 << 2
            ("255", 4),                   // 11111111 << 4
            ("18446744073709551615", 1),  // 2^64 - 1 << 1
            ("18446744073709551615", 64), // 2^64 - 1 << 64
        ];

        for (a_str, shift) in test_cases {
            let a_big = BigUint::from_str(a_str).unwrap();
            let a = IntegerAU::from_biguint(a_big.clone());

            // Test left shift
            let shl_result = &a << shift;
            assert_eq!(
                shl_result.to_biguint(),
                &a_big << shift,
                "Failed left shift test: {} << {}",
                a_str,
                shift
            );

            // Test right shift
            let shr_result = &a >> shift;
            assert_eq!(
                shr_result.to_biguint(),
                &a_big >> shift,
                "Failed right shift test: {} >> {}",
                a_str,
                shift
            );
        }
    }

    #[test]
    fn test_edge_cases() {
        // Test shifting by more than total bits
        let a = IntegerAU {
            limbs: vec![0xFFFFFFFFFFFFFFFF],
        };
        assert_eq!((&a << 128).limbs, vec![0, 0, 0xFFFFFFFFFFFFFFFF]); // Shift left by 2 words
        assert_eq!((&a >> 128).limbs, vec![0]); // Should be zero for right shift

        // Test shifting by exactly one word
        let b = IntegerAU {
            limbs: vec![0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF],
        };
        assert_eq!(
            (&b << 64).limbs,
            vec![0, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF]
        ); // Moving all bits left by one word
        assert_eq!((&b >> 64).limbs, vec![0xFFFFFFFFFFFFFFFF]); // Removing a word
    }
    #[test]
    fn test_bit_len() {
        // Test cases with known results
        let test_cases = vec![
            (vec![0], 0),                                        // Zero
            (vec![1], 1),                                        // Single bit
            (vec![2], 2),                                        // Two bits
            (vec![0xFF], 8),                                     // Eight bits
            (vec![0xFFFF], 16),                                  // Sixteen bits
            (vec![0xFFFFFFFF], 32),                              // Thirty-two bits
            (vec![0xFFFFFFFFFFFFFFFF], 64),                      // Full limb
            (vec![0, 1], 65),                                    // Just into second limb
            (vec![0, 0xFF], 72),                                 // More in second limb
            (vec![0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF], 128), // Two full limbs
        ];

        for (limbs, expected_bits) in test_cases {
            let num = IntegerAU {
                limbs: limbs.clone(),
            };
            assert_eq!(
                num.bit_len(),
                expected_bits,
                "Failed for number with limbs {:?}",
                limbs
            );
        }
    }

    #[test]
    fn test_random_bit_len() {
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            // Generate 1-4 random limbs
            let num_limbs = rng.gen_range(1..=4);
            let mut limbs = Vec::with_capacity(num_limbs);

            // Fill all but the last limb with random values
            for _ in 0..num_limbs - 1 {
                limbs.push(rng.gen::<u64>());
            }

            // Generate the most significant limb with a known number of bits
            let msb_bits = rng.gen_range(1..=64);
            let msb = if msb_bits == 64 {
                0xFFFFFFFFFFFFFFFF
            } else {
                (1u64 << msb_bits) - 1
            };
            limbs.push(msb);

            let num = IntegerAU {
                limbs: limbs.clone(),
            };
            let expected_bits = (num_limbs - 1) * 64 + msb_bits;

            assert_eq!(
                num.bit_len(),
                expected_bits,
                "Failed for random number with limbs {:?}, expected {} bits",
                limbs,
                expected_bits
            );
        }
    }

    #[test]
    fn test_basic_division() {
        let test_cases = vec![
            ("10", "2", "5"),    // Basic division
            ("100", "10", "10"), // Larger numbers
            ("7", "2", "3"),     // Non-exact division
            ("0", "5", "0"),     // Zero dividend
            ("5", "10", "0"),    // Dividend smaller than divisor
            ("42", "42", "1"),   // Equal numbers
        ];

        for (a_str, b_str, expected_str) in test_cases {
            let a = IntegerAU::from_biguint(BigUint::from_str(a_str).unwrap());
            let b = IntegerAU::from_biguint(BigUint::from_str(b_str).unwrap());
            let expected = IntegerAU::from_biguint(BigUint::from_str(expected_str).unwrap());

            assert_eq!(&a / &b, expected);
        }
    }

    #[test]
    fn test_large_numbers() {
        let test_cases = vec![
            // 2^64 / 2
            ("18446744073709551616", "2", "9223372036854775808"),
            // (2^64 - 1) / (2^32 - 1)
            ("18446744073709551615", "4294967295", "4294967297"),
        ];

        for (a_str, b_str, expected_str) in test_cases {
            let a = IntegerAU::from_biguint(BigUint::from_str(a_str).unwrap());
            let b = IntegerAU::from_biguint(BigUint::from_str(b_str).unwrap());
            let expected = IntegerAU::from_biguint(BigUint::from_str(expected_str).unwrap());

            assert_eq!(&a / &b, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_division_by_zero() {
        let a = IntegerAU::from_biguint(BigUint::from(42u64));
        let zero = IntegerAU::from_biguint(BigUint::from(0u64));
        &a / &zero;
    }

    #[test]
    fn test_division_with_remainder() {
        let test_cases = vec![
            ("100", "3"),    // 100 = 33 * 3 + 1
            ("1000", "7"),   // Has remainder
            ("12345", "67"), // Larger numbers
        ];

        for (a_str, b_str) in test_cases {
            let a_big = BigUint::from_str(a_str).unwrap();
            let b_big = BigUint::from_str(b_str).unwrap();
            let expected = &a_big / &b_big;

            let a = IntegerAU::from_biguint(a_big);
            let b = IntegerAU::from_biguint(b_big);
            let result = &a / &b;

            assert_eq!(
                IntegerAU::from_biguint(expected),
                result,
                "Failed division test: {} / {}",
                a_str,
                b_str
            );
        }
    }

    #[test]
    fn test_random_division() {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            // Generate random numbers (making sure divisor isn't zero)
            let a = rng.gen_range(1..=1000000) as u64;
            let b = rng.gen_range(1..=1000000) as u64;

            let a_int = IntegerAU::from(a);
            let b_int = IntegerAU::from(b);

            let expected = a / b;
            let result = &a_int / &b_int;

            assert_eq!(
                result,
                IntegerAU::from_biguint(BigUint::from(expected)),
                "Failed random division test: {} / {}",
                a,
                b
            );
        }
    }

    #[test]
    fn test_bitor_basic() {
        // Basic test cases
        let test_cases = vec![
            (vec![0], vec![0], vec![0]),                // 0 | 0 = 0
            (vec![1], vec![0], vec![1]),                // 1 | 0 = 1
            (vec![0xFF], vec![0xF0], vec![0xFF]),       // 11111111 | 11110000 = 11111111
            (vec![0xF0], vec![0x0F], vec![0xFF]),       // 11110000 | 00001111 = 11111111
            (vec![0xFFFF], vec![0xFF00], vec![0xFFFF]), // Test with larger numbers
        ];

        for (a_limbs, b_limbs, expected) in test_cases {
            let a = IntegerAU {
                limbs: a_limbs.clone(),
            };
            let b = IntegerAU {
                limbs: b_limbs.clone(),
            };
            let result = &a | &b;
            assert_eq!(result.limbs, expected);

            // Test that originals are unchanged
            assert_eq!(a.limbs, a_limbs);
            assert_eq!(b.limbs, b_limbs);
        }
    }

    #[test]
    fn test_bitor_different_lengths() {
        // Test with different length numbers
        let a = IntegerAU {
            limbs: vec![0xFFFF, 0xFFFF],
        };
        let b = IntegerAU {
            limbs: vec![0xFFFF],
        };
        let result = &a | &b;
        assert_eq!(result.limbs, vec![0xFFFF, 0xFFFF]);

        let c = IntegerAU { limbs: vec![0] };
        let d = IntegerAU {
            limbs: vec![0xFFFF, 0xFFFF],
        };
        let result2 = &c | &d;
        assert_eq!(result2.limbs, vec![0xFFFF, 0xFFFF]);
    }

    #[test]
    fn test_bitor_large_numbers() {
        // Test with full u64 values
        let a = IntegerAU {
            limbs: vec![0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF],
        };
        let b = IntegerAU {
            limbs: vec![0xFFFFFFFFFFFFFFFF, 0],
        };
        let result = &a | &b;
        assert_eq!(result.limbs, vec![0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF]);
    }

    #[test]
    fn test_bitor_all_variants() {
        // Test all combinations of owned and borrowed values
        let a = IntegerAU { limbs: vec![0xF0] };
        let b = IntegerAU { limbs: vec![0x0F] };
        let expected = vec![0xFF];

        // &T | &T
        assert_eq!((&a | &b).limbs, expected);

        // T | T
        assert_eq!((a.clone() | b.clone()).limbs, expected);

        // &T | T
        assert_eq!((&a | b.clone()).limbs, expected);

        // T | &T
        assert_eq!((a.clone() | &b).limbs, expected);
    }

    #[test]
    fn test_bitor_with_zero() {
        let zero = IntegerAU { limbs: vec![0] };
        let a = IntegerAU {
            limbs: vec![0xFFFF],
        };

        // x | 0 = x
        assert_eq!((&a | &zero).limbs, vec![0xFFFF]);
        assert_eq!((&zero | &a).limbs, vec![0xFFFF]);

        // 0 | 0 = 0
        assert_eq!((&zero | &zero).limbs, vec![0]);
    }
}
