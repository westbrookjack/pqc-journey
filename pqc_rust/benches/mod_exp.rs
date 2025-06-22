// benches/mod_exp.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use pqc_rust::number_theory::mod_exp::modular_exponentiation;

fn bench_mod_exp(c: &mut Criterion) {
    c.bench_function("mod_exp", |b| {
        b.iter(|| {
            let a = 7u64;
            let b_exp = 1_000_000_000u64;
            let m = 1_000_000_007u64;
            black_box(modular_exponentiation(a, b_exp, m));
        });
    });
}

criterion_group!(benches, bench_mod_exp);
criterion_main!(benches);
