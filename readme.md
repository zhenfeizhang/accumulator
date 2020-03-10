Pairing based accumulator
-----------

This is a very raw, very prototype implementation of an accumulator using
pairing over BLS12-381 curve.
This code comes with minimum tests and benchmarks.
It is not audited. Use at your own risk.

# Algorithms
* public parameters:
  * `[g1^{alpha^i}]` for `i` in `[0, n]`, inclusive
  * `[g2^{alpha^i}]` for `i` in `[0, n]`, inclusive

* accumulate:
  * input `n` values; hash them into `n` Fr elements: `r_1 ... r_n`
  * generate the polynomial `f = \prod_{i=1}^n (x - r_i)`, denote `a_0,...a_n`
  the coefficients of `f`
  * output `acc =\prod_{i=0}^n g1^{alpha^i * a_i}`

* (batch) proof
  * input `n` values;
  * input `k<n` indices for which the proofs are to be generated
  * hash the rest `(n-k)` values into Fr elements `r_1 ... r_{n-k}`
  * generate the polynomial `f = \prod_{i=1}^{n-k} (x - r_i)`, denote `a_0,...a_{n-k}`
  the coefficients of `f`
  * output `pi = \prod_{i=0}^{n-k} g1^{alpha^i * a_i}`

* (batch) verify
  * input accumulator `acc` and proof `pi`
  * input `k` values to be verified
  * hash the rest `k` values into Fr elements `r_1 ... r_k`
  * generate the polynomial `f = \prod_{i=1}^k (x - r_i)`, denote `a_0,...a_k`
  the coefficients of `f`
  * compute `q = \prod_{i=0}^k g2^{alpha^i * a_i}`
  * check `e(pi, q) ?= e(acc, g2)`


# Characteristics
* Accumulator size: `48` bytes
* Accumulator cost: `n * G1_mul`
* Proof size: `48` bytes
* Proof cost: `(n-k) * G1_mul`
* Verification cost: `k * G2_mul + 2 pairing`
