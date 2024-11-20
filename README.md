# moduli-comparison

compare performance of montgomery and barrett reductions

```sh
Running Montgomery benchmark...
N (Prime) = 2013265921
Montgomery 1000000 muls: 25.53ms
Naive Time: 7.42ms

Running Montgomery benchmark...
N (Prime) = 18446744069414584321
Montgomery 1000000 muls: 260.99ms
Naive Time: 22.63ms

Running Montgomery benchmark...
N (Prime) = 170141183460469231731687303715884105727
Montgomery 1000000 muls: 208.14ms
Naive Time: 119.61ms

Running Montgomery benchmark...
N (Prime) = 340282366920938463463374607431768211507
Montgomery 1000000 muls: 313.31ms
Naive Time: 89.23ms

Running Barrett benchmark...
N (Prime):  2013265921
Barrett 1000000 muls: 27.86ms

Running Barrett benchmark...
N (Prime):  18446744069414584321
Barrett 1000000 muls: 67.93ms

Running Barrett benchmark...
N (Prime):  170141183460469231731687303715884105727
Barrett 1000000 muls: 80.56ms

Running Barrett benchmark...
N (Prime):  340282366920938463463374607431768211507
Barrett 1000000 muls: 92.42ms
```
