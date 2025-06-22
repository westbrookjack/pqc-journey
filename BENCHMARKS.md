# 📊 Benchmarks: pqc_rust vs pqc_cpp

This document records benchmark results comparing implementations of number-theoretic algorithms in Rust and C++.

---

## 🧮 Function: `extended_gcd`

| Language | Time per iteration |
|----------|--------------------|
| Rust     | ~27.7 ns           |
| C++      | ~356 ns            |

---

## 🧮 Function: `modular_exponentiation`

| Language | Time per iteration |
|----------|--------------------|
| Rust     | ~196 ns            |
| C++      | ~403 ns            |

---

### ⚙️ Benchmark Setup

- Benchmarks are compiled with optimization flags enabled (e.g. `cargo bench`, CMake with `-O3` by default).
- C++ uses `__uint128_t` for 128-bit arithmetic.
- Boost dependency removed from the C++ build.
- Benchmarks are built and run via:
  - `cargo bench` (Rust)
  - `make && ./extended_gcd_bench` (C++)
  - `make && ./mod_exp_bench` (C++)
- **Tests run on a plugged-in desktop (Mac Studio).**
- Benchmark library: [Criterion](https://bheisler.github.io/criterion.rs/book/index.html) (Rust) and [Google Benchmark](https://github.com/google/benchmark) (C++).

