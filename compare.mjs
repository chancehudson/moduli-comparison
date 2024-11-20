import { benchmarkBarrett } from "./barrett.mjs";
import { benchmarkMontgomery } from "./montgomery.mjs";

const primes = [
  BigInt("2013265921"), // baby bear 32 bit
  BigInt("18446744069414584321"), // oxfoi 64 bit
  BigInt("170141183460469231731687303715884105727"), // prime just below 2^128
  BigInt("340282366920938463463374607431768211507"), // prime just above 2^128
  BigInt(
    "57896044618658097711785492504343953926634992332820282019728792003956564819949",
  ), // closest prime to 2^255
];
const iterations = 10000;
for (const p of primes) {
  const x = BigInt(Math.floor(Math.random() * Number(p)));
  const constants = Array(iterations)
    .fill()
    .map(() => BigInt(Math.floor(Math.random() * Number(p))));
  benchmarkMontgomery(p, iterations, x, constants);
  benchmarkBarrett(p, iterations, x, constants);
}
