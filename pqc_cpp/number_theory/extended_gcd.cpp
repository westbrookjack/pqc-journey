// extended_gcd.cpp

#include "extended_gcd.hpp"

std::tuple<uint64_t, __int128_t, __int128_t> gcd_triple(const uint64_t alpha, const uint64_t beta) {
    if (alpha == 0) {
        return std::tuple(beta, 0, 1);
    } else if (beta == 0) {
        return std::tuple(alpha, __int128_t(1), __int128_t(0));
    } else {
        __int128_t a;
        __int128_t b;

        if (alpha < beta) {
            a = __int128_t(beta);
            b = __int128_t(alpha);
        } else {
            a = __int128_t(alpha);
            b = __int128_t(beta);
        }

        __int128_t q = a / b;
        std::tuple<__int128_t, __int128_t, __int128_t> tup1(a, 1, 0);
        std::tuple<__int128_t, __int128_t, __int128_t> tup2(b, 0, 1);
        std::tuple<__int128_t, __int128_t, __int128_t> holder;

        while (q * std::get<0>(tup2) != std::get<0>(tup1)) {
            holder = tup1;
            tup1 = tup2;
            std::get<0>(tup2) = std::get<0>(holder) % std::get<0>(tup1);
            std::get<1>(tup2) = std::get<1>(holder) - q * std::get<1>(tup1);
            std::get<2>(tup2) = std::get<2>(holder) - q * std::get<2>(tup1);
            q = std::get<0>(tup1) / std::get<0>(tup2);
        }

        if (alpha < beta) {
            __int128_t c = std::get<1>(tup2);
            std::get<1>(tup2) = std::get<2>(tup2);
            std::get<2>(tup2) = c;
        }

        return std::tuple<uint64_t, __int128_t, __int128_t>(
            static_cast<uint64_t>(std::get<0>(tup2)),
            std::get<1>(tup2),
            std::get<2>(tup2)
        );
    }
}
