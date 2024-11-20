export function benchmarkMontgomery(PRIME) {
  const R = 2n ** (log2BigInt(PRIME) + 1n); // R = 2^5 (smallest power of 2 > PRIME)
  const R_SQUARED = (R * R) % PRIME; // R^2 mod P = 4

  // Find N' = -N^(-1) mod R
  const N_PRIME = (-modinv(PRIME, R) + R) % R;
  // Print parameters
  console.log("\nRunning Montgomery benchmark...");
  console.log(`N (Prime) = ${PRIME}`);

  verify(BigInt(Math.floor(Math.random() * Number(PRIME))));

  function log2BigInt(n) {
    // Input validation
    if (typeof n !== "bigint" || n <= 0n) {
      throw new Error("Input must be a positive BigInt");
    }

    // Handle the case of n = 1
    if (n === 1n) {
      return 0n;
    }

    // Initialize the result
    let result = 0n;

    // Keep dividing by 2 until we reach 1
    // Count how many times we can divide by 2
    while (n > 1n) {
      n = n >> 1n; // Bit shift right is equivalent to division by 2
      result += 1n;
    }

    return result;
  }

  // Extended Euclidean Algorithm
  function egcd(a, b) {
    let [old_r, r] = [a, b];
    let [old_s, s] = [1n, 0n];
    let [old_t, t] = [0n, 1n];

    while (r !== 0n) {
      const quotient = old_r / r;
      [old_r, r] = [r, old_r - quotient * r];
      [old_s, s] = [s, old_s - quotient * s];
      [old_t, t] = [t, old_t - quotient * t];
    }

    return { gcd: old_r, x: old_s, y: old_t };
  }

  // Calculate modular multiplicative inverse
  function modinv(a, m) {
    const { gcd, x } = egcd(a, m);
    if (gcd !== 1n) throw new Error("Modular inverse does not exist");
    return ((x % m) + m) % m;
  }

  // Montgomery reduction (REDC)
  function redc(T) {
    const m = ((T % R) * N_PRIME) % R;
    let t = (T + m * PRIME) / R;

    while (t >= PRIME) {
      t -= PRIME;
    }
    return t;
  }

  function toMontgomery(x) {
    return (x * R) % PRIME;
  }

  function fromMontgomery(x) {
    return redc(x);
  }

  function montgomeryMultiply(x, y) {
    // Assuming x and y are in Montgomery form
    return redc(x * y);
  }

  // multiply
  function verify(x, count = 1000000) {
    // console.log(`\nMultiplying ${x} by ${count} random values`);

    // Convert to Montgomery form
    const values = Array(count)
      .fill()
      .map(() => BigInt(Math.floor(Math.random() * Number(PRIME))));
    const valuesMont = values.map(toMontgomery);
    // console.log(`${x} in Montgomery form: ${toMontgomery(x)}`);

    const timeStart = performance.now();
    const x_mont = toMontgomery(x);
    // Multiply in Montgomery form
    let v = x_mont;
    for (let c of valuesMont) {
      v = montgomeryMultiply(v, c);
    }

    // Convert back to normal form
    const result = fromMontgomery(v);
    const time = performance.now() - timeStart;
    console.log(`Montgomery ${count} muls: ${time.toFixed(2)}ms`);

    // Verify against native modular multiplication
    const timeStartN = performance.now();
    const expected = values.reduce((acc, v) => {
      return (acc * v) % PRIME;
    }, x);
    const timeN = performance.now() - timeStartN;
    console.log(`Naive Time: ${timeN.toFixed(2)}ms`);
    // console.log(`Result: ${result}`);
    // console.log(`Expected: ${expected}`);
    // console.log(`Correct: ${result === expected ? "✓" : "✗"}`);

    return result === expected;
  }
}
