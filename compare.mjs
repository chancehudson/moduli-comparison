import { benchmarkBarrett } from "./barrett.mjs";
import { benchmarkMontgomery } from "./montgomery.mjs";

const primes = [
  BigInt("2013265921"), // baby bear 32 bit
  BigInt("18446744069414584321"), // oxfoi 64 bit
  BigInt("170141183460469231731687303715884105727"), // prime just below 2^128
  BigInt("340282366920938463463374607431768211507"), // prime just above 2^128
];
const iterations = 1000000;
for (const p of primes) {
  benchmarkMontgomery(p, iterations);
}
for (const p of primes) {
  benchmarkBarrett(p, iterations);
}
