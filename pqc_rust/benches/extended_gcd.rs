use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pqc_rust::number_theory::extended_gcd::gcd_triple;

fn bench_gcd_triple(c: &mut Criterion) {
    c.bench_function("extended_gcd", |b| {
        b.iter(|| {
            black_box(gcd_triple(123, 456));
        });
    });
}

criterion_group!(benches, bench_gcd_triple);
criterion_main!(benches);
