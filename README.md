# moduli-comparison

Compare performance of montgomery and barrett reductions against naïve reduction. All base operations are naïvely implemented on top of unsigned 64 bit integers.

Usage: `cargo run --release`

```sh
Benchmarking multiplications between random values.
For Montgomery we assume the inputs are already in
Montgomery form. The final result is converted to
field representation.

===== modulus 2013265921 (31 bits) =====
Naïve time for 1000 multiplications: 7.281834ms
Barrett time for 1000 multiplications: 411.5µs
Montgomery time for 1000 multiplications: 383.458µs

===== modulus 18446744069414584321 (64 bits) =====
Naïve time for 1000 multiplications: 5.620084ms
Barrett time for 1000 multiplications: 198.917µs
Montgomery time for 1000 multiplications: 414.25µs

===== modulus 170141183460469231731687303715884105727 (127 bits) =====
Naïve time for 1000 multiplications: 14.191209ms
Barrett time for 1000 multiplications: 250.375µs
Montgomery time for 1000 multiplications: 526.917µs

===== modulus 340282366920938463463374607431768211507 (129 bits) =====
Naïve time for 1000 multiplications: 14.868542ms
Barrett time for 1000 multiplications: 302µs
Montgomery time for 1000 multiplications: 616.708µs

===== modulus 57896044618658097711785492504343953926634992332820282019728792003956564819949 (255 bits) =====
Naïve time for 1000 multiplications: 31.487208ms
Barrett time for 1000 multiplications: 372.333µs
Montgomery time for 1000 multiplications: 680.25µs
```
