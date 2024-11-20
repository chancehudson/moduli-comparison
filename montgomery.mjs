export function benchmarkMontgomery(PRIME, iterations, x, values) {
  const R = 2n ** log2BigInt(PRIME); // smallest power of 2 > PRIME
  const R_BITMASK = R - 1n;
  const R_BITS = log2BigInt(PRIME);

  // Find N' = -N^(-1) mod R
  const N_PRIME = (-modinv(PRIME, R) + R) % R;
  // Print parameters
  console.log("\nRunning Montgomery benchmark...");
  console.log(`N (Prime) = ${PRIME}`);

  verify(x);

  // log2_ceil
  function log2BigInt(n) {
    return BigInt(n.toString(2).length);
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
    const m = ((T & R_BITMASK) * N_PRIME) & R_BITMASK;
    let t = (T + m * PRIME) >> R_BITS;

    // if (t >= PRIME) {
    //   t -= PRIME;
    // }
    return t;
  }

  function toMontgomery(x) {
    return (x << R_BITS) % PRIME;
  }

  function fromMontgomery(x) {
    return redc(x);
  }

  function montgomeryMultiply(x, y) {
    // Assuming x and y are in Montgomery form
    return redc(x * y);
  }

  // multiply
  function verify(x) {
    // console.log(`\nMultiplying ${x} by ${count} random values`);

    // Convert to Montgomery form
    // console.log(`${x} in Montgomery form: ${toMontgomery(x)}`);
    const valuesMont = values.map(toMontgomery);

    const timeStart = performance.now();
    const x_mont = toMontgomery(x);
    // Multiply in Montgomery form
    const v = valuesMont.reduce(
      (acc, next) => montgomeryMultiply(acc, next),
      x_mont,
    );

    // Convert back to normal form
    const result = fromMontgomery(v);
    const time = performance.now() - timeStart;
    console.log(`Montgomery ${iterations} muls: ${time.toFixed(2)}ms`);

    // Verify against native modular multiplication
    const timeStartN = performance.now();
    const expected = values.reduce((acc, v) => {
      return (acc * v) % PRIME;
    }, x);
    const timeN = performance.now() - timeStartN;
    console.log(`Naive Time: ${timeN.toFixed(2)}ms`);
    if (expected !== result) {
      throw new Error(
        `Montgomery multiplication failed: ${result} !== ${expected}`,
      );
    }
    // console.log(`Result: ${result}`);
    // console.log(`Expected: ${expected}`);
    // console.log(`Correct: ${result === expected ? "✓" : "✗"}`);

    return result === expected;
  }
}
