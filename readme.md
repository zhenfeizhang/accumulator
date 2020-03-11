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

# Benchmarks
## accumulation

| dim | time (ms) |
|---:|---:|
| 64 | 4.82 |
| 128 | 8.26 |
| 256 | 14.9 |
| 512 | 28.7 |
| 1024| 62.2 |

## prove and verify
| dim | #value | prove | verify |
|---:|---:|---:|---:|
| 64 | 1 | 4.77 | 4.68 |
| 64 | 2 | 4.70 | 5.31 |
| 64 | 4 | 4.63 | 6.06 |
| 64 | 8 | 4.42 | 7.42 |
| 64 | 16 | 3.94 | 9.58 |
| 64 | 32 | 2.93 | 13.2 |



| dim | #value | prove | verify |
|---:|---:|---:|---:|
| 128 | 1 | 8.25 | 4.80 |
| 128 | 2 | 8.20 | 5.25 |
| 128 | 4 | 8.08 | 6.08 |
| 128 | 8 | 7.90 | 7.48 |
| 128 | 16 | 7.51 | 9.52 |
| 128 | 32 | 6.64 | 13.2 |


| dim | #value | prove | verify |
|---:|---:|---:|---:|
| 256 | 1 | 14.87 | 4.69 |
| 256 | 2 | 14.85 | 5.33 |
| 256 | 4 | 14.74 | 6.23 |
| 256 | 8 | 14.50 | 7.35 |
| 256 | 16 | 14.15 | 9.51 |
| 256 | 32 | 13.29 | 13.1 |



| dim | #value | prove | verify |
|---:|---:|---:|---:|
| 512 | 1 | 28.68 | 4.74 |
| 512 | 2 | 28.63 | 5.30 |
| 512 | 4 | 28.55 | 6.15 |
| 512 | 8 | 28.30 | 7.32 |
| 512 | 16 | 27.80 | 9.44 |
| 512 | 32 | 26.87 | 13.1 |




| dim | #value | prove | verify |
|---:|---:|---:|---:|
| 1024 | 1 | 62.19 | 4.80 |
| 1024 | 2 | 62.33 | 5.31 |
| 1024 | 4 | 62.20 | 6.17 |
| 1024 | 8 | 61.90| 7.37 |
| 1024 | 16 | 61.30 | 9.63 |
| 1024 | 32 | 60.11 | 13.0 |
