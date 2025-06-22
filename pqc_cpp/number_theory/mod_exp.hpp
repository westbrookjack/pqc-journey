#pragma once

#include <tuple>
#include <bit>
#include <cstdint>


// Computes (gcd, x, y) such that gcd = x * a + y * b
uint64_t modular_exponentiation(const uint64_t alpha, const uint64_t beta, const uint64_t mu);