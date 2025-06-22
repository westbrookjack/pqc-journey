// benches/mod_exp_bench.cpp

#include <benchmark/benchmark.h>
#include "../number_theory/mod_exp.hpp"  // Adjust if necessary

// Prevent compiler optimization
static volatile uint64_t sink;

static void BM_ModExp(benchmark::State& state) {
    const uint64_t a = 7;
    const uint64_t b = 1'000'000'000;
    const uint64_t m = 1'000'000'007;

    for (auto _ : state) {
        sink = modular_exponentiation(a, b, m);
    }
}

BENCHMARK(BM_ModExp);

BENCHMARK_MAIN();
