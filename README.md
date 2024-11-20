# moduli-comparison

compare performance of montgomery and barrett reductions

```sh
Running Montgomery benchmark...
N (Prime) = 2013265921
Montgomery 10000 muls: 1.33ms
Naive Time: 0.46ms

Running Barrett benchmark...
N (Prime):  2013265921
Barrett 10000 muls: 1.05ms
Naive Time: 0.22ms

Running Montgomery benchmark...
N (Prime) = 18446744069414584321
Montgomery 10000 muls: 1.47ms
Naive Time: 0.38ms

Running Barrett benchmark...
N (Prime):  18446744069414584321
Barrett 10000 muls: 1.04ms
Naive Time: 0.36ms

Running Montgomery benchmark...
N (Prime) = 170141183460469231731687303715884105727
Montgomery 10000 muls: 1.31ms
Naive Time: 1.43ms

Running Barrett benchmark...
N (Prime):  170141183460469231731687303715884105727
Barrett 10000 muls: 1.06ms
Naive Time: 1.55ms

Running Montgomery benchmark...
N (Prime) = 340282366920938463463374607431768211507
Montgomery 10000 muls: 1.34ms
Naive Time: 1.03ms

Running Barrett benchmark...
N (Prime):  340282366920938463463374607431768211507
Barrett 10000 muls: 1.12ms
Naive Time: 1.04ms

Running Montgomery benchmark...
N (Prime) = 57896044618658097711785492504343953926634992332820282019728792003956564819949
Montgomery 10000 muls: 1.33ms
Naive Time: 2.09ms

Running Barrett benchmark...
N (Prime):  57896044618658097711785492504343953926634992332820282019728792003956564819949
Barrett 10000 muls: 1.31ms
Naive Time: 2.46ms
```
