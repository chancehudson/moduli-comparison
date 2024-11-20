# moduli-comparison

compare performance of montgomery and barrett reductions

```sh
Running Montgomery benchmark...
N (Prime) = 2013265921
Montgomery 1000000 muls: 24.81ms
Naive Time: 7.68ms

Running Montgomery benchmark...
N (Prime) = 18446744069414584321
Montgomery 1000000 muls: 279.79ms
Naive Time: 22.43ms

Running Montgomery benchmark...
N (Prime) = 170141183460469231731687303715884105727
Montgomery 1000000 muls: 207.61ms
Naive Time: 121.00ms

Running Montgomery benchmark...
N (Prime) = 340282366920938463463374607431768211507
Montgomery 1000000 muls: 319.18ms
Naive Time: 89.77ms

Running Barrett benchmark...
N (Prime):  2013265921
Barrett 1000000 muls: 27.54ms
Naive Time: 14.11ms

Running Barrett benchmark...
N (Prime):  18446744069414584321
Barrett 1000000 muls: 67.70ms
Naive Time: 25.16ms

Running Barrett benchmark...
N (Prime):  170141183460469231731687303715884105727
Barrett 1000000 muls: 80.05ms
Naive Time: 122.21ms

Running Barrett benchmark...
N (Prime):  340282366920938463463374607431768211507
Barrett 1000000 muls: 97.13ms
Naive Time: 108.91ms
```
