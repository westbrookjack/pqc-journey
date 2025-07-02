use aes_toy::{encrypt_cbc, decrypt_cbc, encrypt_ecb, decrypt_ecb};
use criterion::{criterion_group, criterion_main, Criterion, black_box};

const TEN_MB: usize = 10 * 1024 * 1024;


fn bench_encrypt_cbc(c: &mut Criterion) {
    let key = [0u8; 16];
    let iv = [1u8; 16];
    let plaintext = vec![0u8; TEN_MB];

    c.bench_function("encrypt_cbc 10MB", |b| {
        b.iter(|| {
            let _ = encrypt_cbc(black_box(&plaintext), black_box(&key), black_box(iv));
        });
    });
}


fn bench_decrypt_cbc(c: &mut Criterion) {
    let key = [0u8; 16];
    let iv = [1u8; 16];
    let ciphertext = encrypt_cbc(&vec![0u8; TEN_MB], &key, iv);

    c.bench_function("decrypt_cbc 10MB", |b| {
        b.iter(|| {
            let _ = decrypt_cbc(black_box(&ciphertext), black_box(&key), black_box(iv));
        });
    });
}


fn bench_encrypt_ecb(c: &mut Criterion) {
    let key = [0u8; 16];
    let plaintext = vec![0u8; TEN_MB];

    c.bench_function("encrypt_ecb 10MB", |b| {
        b.iter(|| {
            let _ = encrypt_ecb(black_box(&plaintext), black_box(&key));
        });
    });
}


fn bench_decrypt_ecb(c: &mut Criterion) {
    let key = [0u8; 16];
    let ciphertext = encrypt_ecb(&vec![0u8; TEN_MB], &key);

    c.bench_function("decrypt_ecb 10MB", |b| {
        b.iter(|| {
            let _ = decrypt_ecb(black_box(&ciphertext), black_box(&key));
        });
    });
}



criterion_group!(
    benches,
    bench_encrypt_cbc,
    bench_decrypt_cbc,
    bench_encrypt_ecb,
    bench_decrypt_ecb,
);
criterion_main!(benches);


