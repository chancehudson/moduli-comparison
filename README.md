# moduli-comparison

Compare performance of montgomery and barrett reductions against naïve reduction. All base operations are naïvely implemented on top of unsigned 64 bit integers.

Usage: `cargo run --release`

```sh
Benchmarking multiplications between random values.
For Montgomery we assume the inputs are already in
Montgomery form. The final result is converted to
field representation.

===== modulus 2013265921 (31 bits) =====
Naïve time for 1000 multiplications: 5.440709ms
Barrett time for 1000 multiplications: 309.708µs
Montgomery time for 1000 multiplications: 162.417µs

===== modulus 18446744069414584321 (64 bits) =====
Naïve time for 1000 multiplications: 4.62325ms
Barrett time for 1000 multiplications: 116.083µs
Montgomery time for 1000 multiplications: 182.709µs

===== modulus 170141183460469231731687303715884105727 (127 bits) =====
Naïve time for 1000 multiplications: 10.771375ms
Barrett time for 1000 multiplications: 141.458µs
Montgomery time for 1000 multiplications: 244.458µs

===== modulus 340282366920938463463374607431768211507 (129 bits) =====
Naïve time for 1000 multiplications: 11.514125ms
Barrett time for 1000 multiplications: 197.625µs
Montgomery time for 1000 multiplications: 285.459µs

===== modulus 57896044618658097711785492504343953926634992332820282019728792003956564819949 (255 bits) =====
Naïve time for 1000 multiplications: 24.535417ms
Barrett time for 1000 multiplications: 192.833µs
Montgomery time for 1000 multiplications: 330.917µs
```
