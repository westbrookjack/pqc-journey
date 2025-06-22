cmake_minimum_required(VERSION 3.14)
project(pqc_cpp)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_PREFIX_PATH "/opt/homebrew" ${CMAKE_PREFIX_PATH})

# Dependencies
find_package(benchmark REQUIRED)

include_directories(
    ${benchmark_INCLUDE_DIRS}
    data_structures
    number_theory
    katas
)

# Benchmarks
add_executable(extended_gcd_bench
    benches/extended_gcd_bench.cpp
    number_theory/extended_gcd.cpp
)

add_executable(mod_exp_bench
    benches/mod_exp_bench.cpp
    number_theory/mod_exp.cpp
)

target_link_libraries(extended_gcd_bench benchmark::benchmark)
target_link_libraries(mod_exp_bench benchmark::benchmark)

# Commented out unused executables
# add_executable(securebuffer_demo
#     data_structures/securebuffer-demo/main.cpp
#     data_structures/SecureBuffer.cpp
# )

# add_executable(benchmark
#     number_theory/benchmark.cpp
#     number_theory/mod_exp.cpp
#     number_theory/extended_gcd.cpp
# )

# add_executable(kata_runner
#     katas/counter.cpp
#     katas/fizzbuzz.cpp
#     katas/max3.cpp
#     katas/sum_n.cpp
#     katas/print_triangle.cpp
# )
