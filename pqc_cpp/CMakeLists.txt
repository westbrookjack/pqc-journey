cmake_minimum_required(VERSION 3.14)
project(pqc_cpp)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include_directories(
    data_structures
    number_theory
    katas
)

add_executable(securebuffer_demo
    data_structures/securebuffer-demo/main.cpp
    data_structures/SecureBuffer.cpp
)

add_executable(benchmark
    number_theory/benchmark.cpp
    number_theory/mod_exp.cpp
    number_theory/extended_gcd.cpp
)

add_executable(kata_runner
    katas/counter.cpp
    katas/fizzbuzz.cpp
    katas/max3.cpp
    katas/sum_n.cpp
    katas/print_triangle.cpp
)
