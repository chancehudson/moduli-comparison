export function benchmarkBarrett(PRIME) {
  // Run benchmark
  console.log("\nRunning Barrett benchmark...");
  console.log("N (Prime): ", PRIME.toString());

  // Benchmark functions
  const PRIME_BIT_LENGTH = BigInt(PRIME.toString(2).length);
  // Precompute parameters
  const BARRET_MU = 2n ** (2n * PRIME_BIT_LENGTH) / PRIME;

  const iterations = 1000000;

  // Generate random test values
  const x = BigInt(Math.floor(Math.random() * Number(PRIME)));
  const testValues = Array(iterations)
    .fill()
    .map(() => BigInt(Math.floor(Math.random() * Number(PRIME))));
  const expected = testValues.reduce((acc, v) => {
    return (acc * v) % PRIME;
  }, x);

  // Benchmark Barrett
  const barrettStart = performance.now();
  const barrettOut = testValues.reduce((acc, v) => barrettReduce(acc * v), x);
  const barrettTime = performance.now() - barrettStart;
  // check that the barrett reduction is correct
  if (barrettOut !== expected) {
    console.log(barrettOut, expected);
    throw new Error("barrett reduction is incorrect");
  }

  console.log(`Barrett ${iterations} muls: ${barrettTime.toFixed(2)}ms`);

  function barrettReduce(x) {
    const q = ((x >> PRIME_BIT_LENGTH) * BARRET_MU) >> PRIME_BIT_LENGTH;
    let r = x - q * PRIME;

    // Final reduction step
    while (r >= PRIME) {
      r -= PRIME;
    }
    return r;
  }
}
