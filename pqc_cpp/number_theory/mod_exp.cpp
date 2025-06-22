#include "mod_exp.hpp"

uint64_t modular_exponentiation(const uint64_t alpha, const uint64_t b, const uint64_t mu) {
    __uint128_t a = __uint128_t(alpha);
    __uint128_t m = __uint128_t(mu);

    a%=m;


    if (b==0) {
        return 1;
    }
    else {
        __uint128_t result = __uint128_t(1);
        for (uint64_t i = 0; i < 63 - std::countl_zero(b); ++i) {
            uint64_t bit = (b >> i) &1;

            if (bit == 1) {
                result = (result * a) % m;
            }
        a = (a*a)%m;
        }
    return uint64_t(result);
    }

}