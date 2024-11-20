# moduli-comparison

compare performance of montgomery and barrett reductions

```sh
Running Montgomery benchmark...
N (Prime) = 2013265921
Montgomery 1000000 muls: 17.58ms
Naive Time: 7.48ms

Running Montgomery benchmark...
N (Prime) = 18446744069414584321
Montgomery 1000000 muls: 18.48ms
Naive Time: 22.30ms

Running Montgomery benchmark...
N (Prime) = 170141183460469231731687303715884105727
Montgomery 1000000 muls: 16.27ms
Naive Time: 119.96ms

Running Montgomery benchmark...
N (Prime) = 340282366920938463463374607431768211507
Montgomery 1000000 muls: 18.21ms
Naive Time: 91.70ms

Running Barrett benchmark...
N (Prime):  2013265921
Barrett 1000000 muls: 27.91ms
Naive Time: 10.86ms

Running Barrett benchmark...
N (Prime):  18446744069414584321
Barrett 1000000 muls: 68.13ms
Naive Time: 27.60ms

Running Barrett benchmark...
N (Prime):  170141183460469231731687303715884105727
Barrett 1000000 muls: 80.04ms
Naive Time: 126.00ms

Running Barrett benchmark...
N (Prime):  340282366920938463463374607431768211507
Barrett 1000000 muls: 94.86ms
Naive Time: 95.59ms
```
