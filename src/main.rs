use std::str::FromStr;
use std::time::Instant;

use num_bigint::BigUint;

mod barrett;
mod integer_au;
mod montgomery;

use barrett::Barrett;
use integer_au::IntegerAU;
use montgomery::Montgomery;

static PRIMES: [&str; 5] = [
    "2013265921",
    "18446744069414584321",
    "170141183460469231731687303715884105727",
    "340282366920938463463374607431768211507",
    "57896044618658097711785492504343953926634992332820282019728792003956564819949",
];

fn main() -> anyhow::Result<()> {
    // Run registered benchmarks.
    divan::main();

    let primes = PRIMES
        .iter()
        .map(|p| IntegerAU::from_biguint(BigUint::from_str(p).unwrap()))
        .collect::<Vec<_>>();
    benchmark_muls(&primes)?;
    benchmark_muls_sum(&primes)?;
    Ok(())
}

#[divan::bench(args = PRIMES)]
fn bench_barrett(bencher: divan::Bencher, prime_str: &str) {
    let p = IntegerAU::from_biguint(BigUint::from_str(prime_str).unwrap());
    let barrett_reducer = Barrett::new(p.clone());
    let x = &IntegerAU::random_below(&p);
    let y = &IntegerAU::random_below(&p);
    bencher.bench_local(move || {
        let _reduced = barrett_reducer.reduce(&(x * y));
    });
}

#[divan::bench(args = PRIMES)]
fn bench_biguint(bencher: divan::Bencher, prime_str: &str) {
    let p = BigUint::from_str(prime_str).unwrap();
    let p_int = IntegerAU::from_biguint(p.clone());
    let x = &IntegerAU::random_below(&p_int).to_biguint();
    let y = &IntegerAU::random_below(&p_int).to_biguint();
    bencher.bench_local(move || {
        let _reduce = x * y % &p;
    });
}

#[divan::bench(args = PRIMES)]
fn bench_barrett_poseidon_approx(bencher: divan::Bencher, prime_str: &str) {
    let p = IntegerAU::from_biguint(BigUint::from_str(prime_str).unwrap());
    let barrett_reducer = Barrett::new(p.clone());
    let num_rounds = 70;

    let state_len = 3;
    // approximating PoseidonT3, 3 state elements
    let mut state = vec![IntegerAU::from(0); state_len];

    // we'll sample some pretend round constants
    let round_constants = (0..(num_rounds * state.len()))
        .map(|_| IntegerAU::random_below(&p))
        .collect::<Vec<_>>();
    // let m_box = (0..state.len())
    //     .map(|_| IntegerAU::random_below(&p))
    //     .collect::<Vec<_>>();
    bencher.bench_local(move || {
        let pow5 = |x: &IntegerAU| {
            let x2 = barrett_reducer.reduce(&(x * x));
            let x4 = barrett_reducer.reduce(&(&x2 * &x2));
            barrett_reducer.reduce(&(&x4 * x))
        };
        let simple_reduce = |x: &IntegerAU| {
            if x >= &p {
                x - &p
            } else {
                x.clone()
            }
        };
        for i in 0..num_rounds {
            // add the round constants
            state[0] += &round_constants[i * state_len + 0];
            // state[0] = simple_reduce(&state[0]);
            state[1] += &round_constants[i * state_len + 1];
            // state[1] = simple_reduce(&state[1]);
            state[2] += &round_constants[i * state_len + 2];
            // state[2] = simple_reduce(&state[2]);
            // pow5, pretend every round is a full round
            state[0] = pow5(&state[0]);
            state[1] = pow5(&state[1]);
            state[2] = pow5(&state[2]);
            //
            state[0] = barrett_reducer.reduce(&(&state[0] * &state[0]));
            state[0] = barrett_reducer.reduce(&(&state[0] * &state[0]));
            state[0] = barrett_reducer.reduce(&(&state[0] * &state[0]));
            state[1] = barrett_reducer.reduce(&(&state[1] * &state[1]));
            state[1] = barrett_reducer.reduce(&(&state[1] * &state[1]));
            state[1] = barrett_reducer.reduce(&(&state[1] * &state[1]));
            state[2] = barrett_reducer.reduce(&(&state[2] * &state[2]));
            state[2] = barrett_reducer.reduce(&(&state[2] * &state[2]));
            state[2] = barrett_reducer.reduce(&(&state[2] * &state[2]));
        }
    });
}

#[divan::bench(args = PRIMES)]
fn bench_montgomery_poseidon_approx(bencher: divan::Bencher, prime_str: &str) {
    let p = IntegerAU::from_biguint(BigUint::from_str(prime_str).unwrap());
    let montgomery = Montgomery::new(&p);
    let x = &montgomery.to_mont(&IntegerAU::random_below(&p));
    let y = &montgomery.to_mont(&IntegerAU::random_below(&p));
    let num_rounds = 70;

    let state_len = 3;
    // approximating PoseidonT3, 3 state elements
    let mut state = vec![IntegerAU::from(0); state_len];

    // we'll sample some pretend round constants
    let round_constants = (0..(num_rounds * state.len()))
        .map(|_| montgomery.to_mont(&IntegerAU::random_below(&p)))
        .collect::<Vec<_>>();
    // let m_box = (0..state.len())
    //     .map(|_| IntegerAU::random_below(&p))
    //     .collect::<Vec<_>>();
    bencher.bench_local(move || {
        let pow5 = |x: &IntegerAU| {
            let x2 = montgomery.redc(&(x * x));
            let x4 = montgomery.redc(&(&x2 * &x2));
            montgomery.redc(&(&x4 * x))
        };
        let simple_reduce = |x: &IntegerAU| {
            if x >= &p {
                x - &p
            } else {
                x.clone()
            }
        };
        for i in 0..num_rounds {
            // add the round constants
            state[0] += &round_constants[i * state_len + 0];
            state[0] = simple_reduce(&state[0]);
            state[1] += &round_constants[i * state_len + 1];
            state[1] = simple_reduce(&state[1]);
            state[2] += &round_constants[i * state_len + 2];
            state[2] = simple_reduce(&state[2]);
            // pow5, pretend every round is a full round
            state[0] = pow5(&state[0]);
            state[1] = pow5(&state[1]);
            state[2] = pow5(&state[2]);
            state[0] = montgomery.redc(&(&state[0] * &state[0]));
            state[0] = montgomery.redc(&(&state[0] * &state[0]));
            state[0] = montgomery.redc(&(&state[0] * &state[0]));
            state[1] = montgomery.redc(&(&state[1] * &state[1]));
            state[1] = montgomery.redc(&(&state[1] * &state[1]));
            state[1] = montgomery.redc(&(&state[1] * &state[1]));
            state[2] = montgomery.redc(&(&state[2] * &state[2]));
            state[2] = montgomery.redc(&(&state[2] * &state[2]));
            state[2] = montgomery.redc(&(&state[2] * &state[2]));
        }
        montgomery.from_mont(&state[0]);
    });
}

#[divan::bench(args = PRIMES)]
fn bench_montgomery(bencher: divan::Bencher, prime_str: &str) {
    let p = IntegerAU::from_biguint(BigUint::from_str(prime_str).unwrap());
    let montgomery = Montgomery::new(&p);
    let x = &montgomery.to_mont(&IntegerAU::random_below(&p));
    let y = &montgomery.to_mont(&IntegerAU::random_below(&p));
    bencher.bench_local(move || {
        let _z = montgomery.from_mont(&montgomery.redc(&(x * y)));
    });
}

#[divan::bench(args = PRIMES)]
fn bench_naive(bencher: divan::Bencher, prime_str: &str) {
    let p = IntegerAU::from_biguint(BigUint::from_str(prime_str).unwrap());
    let x = &IntegerAU::random_below(&p);
    let y = &IntegerAU::random_below(&p);
    bencher.bench_local(move || {
        let _z = (x * y) % p.clone();
    });
}

// Benchmark a sequence of multiplications between random values
// For montgomery we assume the values are already in montgomery form
// and extract the final value into base field representation

// Barrett generally performs better here
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
        let mut expected = Vec::with_capacity(iterations);
        let start = Instant::now();
        for (x, y) in &values {
            expected.push((x * y) % p.clone());
        }
        println!(
            "Naïve time for {iterations} multiplications: {:?}",
            start.elapsed()
        );

        let mut barrett_result = Vec::with_capacity(iterations);
        let start = Instant::now();
        for (x, y) in &values {
            // the value before reduction
            let z = x * y;
            let reduced = barrett_reducer.reduce(&z);
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
            .map(|(x, y)| (montgomery.to_mont(x), montgomery.to_mont(y)))
            .collect::<Vec<_>>();
        let start = Instant::now();
        for (x, y) in &mont_vals {
            // the value before reduction
            let z = montgomery.from_mont(&montgomery.redc(&(x * y)));
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

fn benchmark_muls_sum(primes: &Vec<IntegerAU>) -> anyhow::Result<()> {
    let iterations = 10000;
    println!("\nBenchmarking multiplications and then summation");
    println!("between random values.");
    println!("For Montgomery we assume the inputs are already in");
    println!("Montgomery form. The final result is converted to");
    println!("field representation.");
    for p in primes {
        println!("\n===== modulus {p} ({} bits) =====", p.bit_len());
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
        let mut expected = Vec::with_capacity(iterations);
        let start = Instant::now();
        for (x, y) in &values {
            // the value before reduction
            let z = (x * y) % p.clone();
            expected.push(z);
        }
        let expected_out = expected
            .iter()
            .fold(IntegerAU::from(0), |acc, x| (&acc + x) % p.clone());
        println!(
            "Naïve time for {iterations} multiplications and summation: {:?}",
            start.elapsed()
        );
        let mut barrett_result = Vec::with_capacity(iterations);
        let barrett_reducer = Barrett::new(p.clone());
        let start = Instant::now();
        for (x, y) in &values {
            // the value before reduction
            let z = x * y;
            let reduced = barrett_reducer.reduce(&z);
            barrett_result.push(reduced);
        }
        let barrett_out = barrett_result.iter().fold(IntegerAU::from(0), |acc, x| {
            barrett_reducer.reduce(&(&acc + x))
        });
        println!(
            "Barrett time for {iterations} multiplications and summation: {:?}",
            start.elapsed()
        );
        let mut mont_result = Vec::with_capacity(iterations);
        let montgomery = Montgomery::new(&p);
        let mont_vals = values
            .iter()
            .map(|(x, y)| (montgomery.to_mont(x), montgomery.to_mont(y)))
            .collect::<Vec<_>>();
        let start = Instant::now();
        for (x, y) in &mont_vals {
            // the value before reduction
            let z = montgomery.redc(&(x * y));
            mont_result.push(z);
        }
        let mont_out = mont_result.iter().fold(IntegerAU::from(0), |acc, x| {
            montgomery.reduce_naive(&(&acc + x))
        });
        let mont_out = montgomery.from_mont(&mont_out);
        println!(
            "Montgomery time for {iterations} multiplications and summation: {:?}",
            start.elapsed()
        );
        let biguint_values = values
            .iter()
            .map(|(x, y)| (x.to_biguint(), y.to_biguint()))
            .collect::<Vec<_>>();
        let p_biguint = p.to_biguint();
        let start = Instant::now();
        let mut biguint_result = Vec::with_capacity(iterations);
        for (x, y) in biguint_values {
            // the value before reduction
            let z = (x * y) % &p_biguint;
            biguint_result.push(z);
        }
        let biguint_out = biguint_result
            .iter()
            .fold(BigUint::from(0u64), |acc, x| (&acc + x) % &p_biguint);
        println!(
            "BigUint time for {iterations} multiplications and summation: {:?}",
            start.elapsed()
        );
        for i in 0..iterations {
            assert_eq!(
                expected_out, barrett_out,
                "barrett reduction mismatches naive reduction"
            );
            assert_eq!(
                expected_out, mont_out,
                "montgomery reduction mismatches naive reduction"
            );
            assert_eq!(
                expected_out,
                IntegerAU::from_biguint(biguint_out.clone()),
                "BigUint reduction mismatches naive reduction"
            );
        }
    }
    Ok(())
}
