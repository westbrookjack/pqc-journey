// extended_gcd.hpp

#pragma once

#include <tuple>
#include <cstdint>

// Computes (gcd, x, y) such that gcd = x * a + y * b
std::tuple<uint64_t, __int128_t, __int128_t> gcd_triple(const uint64_t alpha, const uint64_t beta);
