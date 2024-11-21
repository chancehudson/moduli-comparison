use std::str::FromStr;
use std::time::Instant;

use num_bigint::BigUint;

mod barrett;
mod integer_au;
mod montgomery;
mod scalar_field;

use barrett::Barrett;
use integer_au::IntegerAU;
use montgomery::Montgomery;

fn main() -> anyhow::Result<()> {
    let primes = vec![
        IntegerAU::from_biguint(BigUint::from_str("2013265921")?), // baby bear 32 bit
        IntegerAU::from_biguint(BigUint::from_str("18446744069414584321")?), // oxfoi 64 bit
        IntegerAU::from_biguint(BigUint::from_str(
            "170141183460469231731687303715884105727",
        )?), // prime just below 2^128
        IntegerAU::from_biguint(BigUint::from_str(
            "340282366920938463463374607431768211507",
        )?), // prime just above 2^128
        IntegerAU::from_biguint(BigUint::from_str(
            "57896044618658097711785492504343953926634992332820282019728792003956564819949",
        )?), // closest prime to 2^255
    ];
    benchmark_muls(&primes)?;
    Ok(())
}

/// Benchmark a sequence of multiplications between random values
/// For montgomery we assume the values are already in montgomery form
/// and extract the final value into base field representation
///
/// Barrett generally performs better here
fn benchmark_muls(primes: &Vec<IntegerAU>) -> anyhow::Result<()> {
    let iterations = 1000;
    println!("\nBenchmarking multiplications between random values.");
    println!("For Montgomery we assume the inputs are already in");
    println!("Montgomery form. The final result is converted to");
    println!("field representation.");
    for p in primes {
        println!("\n===== modulus {p} ({} bits) =====", p.bit_len());
        let barrett_reducer = Barrett::new(p.clone());
        // sample the integers before we starting timing
        // rejection sampling smh
        let values = (0..iterations)
            .into_iter()
            .map(|_| {
                let x = IntegerAU::random_below(&p);
                let y = IntegerAU::random_below(&p);
                (x, y)
            })
            .collect::<Vec<_>>();
        let start = Instant::now();
        let mut expected = Vec::with_capacity(iterations);
        for (x, y) in &values {
            // the value before reduction
            let z = x.clone() * y.clone();
            expected.push(z.clone() % p.clone());
        }
        println!(
            "Naïve time for {iterations} multiplications: {:?}",
            start.elapsed()
        );
        let mut barrett_result = Vec::with_capacity(iterations);
        let start = Instant::now();
        for (x, y) in &values {
            // the value before reduction
            let z = x.clone() * y.clone();
            let reduced = barrett_reducer.reduce(z.clone());
            barrett_result.push(reduced);
        }
        println!(
            "Barrett time for {iterations} multiplications: {:?}",
            start.elapsed()
        );
        let mut mont_result = Vec::with_capacity(iterations);
        let montgomery = Montgomery::new(&p);
        let mont_vals = values
            .iter()
            .map(|(x, y)| (montgomery.to_mont(x.clone()), montgomery.to_mont(y.clone())))
            .collect::<Vec<_>>();
        let start = Instant::now();
        for (x, y) in &mont_vals {
            // the value before reduction
            let z = montgomery.from_mont(montgomery.redc(x.clone() * y.clone()));
            mont_result.push(z);
        }
        println!(
            "Montgomery time for {iterations} multiplications: {:?}",
            start.elapsed()
        );
        for i in 0..iterations {
            assert_eq!(
                expected[i], barrett_result[i],
                "barrett reduction mismatches naive reduction"
            );
            assert_eq!(
                expected[i], mont_result[i],
                "montgomery reduction mismatches naive reduction"
            );
        }
    }
    Ok(())
}
