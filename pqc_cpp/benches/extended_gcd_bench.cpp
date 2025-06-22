#include <tuple>
#include <benchmark/benchmark.h>
#include "../number_theory/extended_gcd.hpp"

static volatile uint64_t sink;

static void BM_ExtendedGCD(benchmark::State& state) {
    for (auto _ : state) {
        auto result = gcd_triple(123, 456);
        sink = std::get<0>(result);  // Store only the gcd to prevent optimization
    }
}

BENCHMARK(BM_ExtendedGCD);

BENCHMARK_MAIN();
