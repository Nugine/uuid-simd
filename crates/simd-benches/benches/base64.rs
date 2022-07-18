use simd_benches::rand_bytes;

use criterion::{black_box, criterion_group, criterion_main};
use criterion::{Bencher, BenchmarkId, Criterion, Throughput};

use base64_simd::OutBuf;

pub fn bench_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("base64-simd-encode");

    let inputs: Vec<Vec<u8>> = [16, 32, 64, 256, 1024, 4096, 65536]
        .iter()
        .copied()
        .map(rand_bytes)
        .collect();

    #[allow(clippy::type_complexity)]
    let functions: &[(&str, fn(&mut Bencher, &[u8], &mut [u8]))] = &[
        ("base64-simd/auto-indirect", |b, src, dst| {
            b.iter(|| {
                let (src, dst) = (black_box(src), black_box(OutBuf::new(dst)));
                let base64 = base64_simd::Base64::STANDARD;
                base64.encode(src, dst).unwrap();
            })
        }),
        ("radix64/auto", |b, src, dst| {
            b.iter(|| {
                radix64::STD.encode_slice(black_box(src), black_box(dst));
            })
        }),
        ("base64/fallback", |b, src, dst| {
            b.iter(|| {
                let config = base64::STANDARD;
                base64::encode_config_slice(black_box(src), config, black_box(dst))
            })
        }),
    ];

    for &(name, f) in functions {
        for src in &inputs {
            group.throughput(Throughput::Bytes(src.len() as u64));
            let id = BenchmarkId::new(name, src.len());
            let mut dst: Vec<u8> = vec![0; src.len() / 3 * 4 + 4];
            group.bench_with_input(id, src.as_slice(), |b, src| f(b, src, dst.as_mut_slice()));
        }
    }
}

pub fn bench_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("base64-simd-decode");

    let inputs: Vec<Vec<u8>> = [16, 32, 64, 256, 1024, 4096, 65536]
        .iter()
        .copied()
        .map(rand_bytes)
        .map(|b| base64::encode(b).into_bytes())
        .collect();

    #[allow(clippy::type_complexity)]
    let functions: &[(&str, fn(&mut Bencher, &[u8], &mut [u8]))] = &[
        ("base64-simd/auto-indirect", |b, src, dst| {
            b.iter(|| {
                let (src, dst) = (black_box(src), black_box(OutBuf::new(dst)));
                let base64 = base64_simd::Base64::STANDARD;
                base64.decode(src, dst).unwrap();
            })
        }),
        ("radix64/auto", |b, src, dst| {
            b.iter(|| {
                radix64::STD
                    .decode_slice(black_box(src), black_box(dst))
                    .unwrap();
            })
        }),
        ("base64/fallback", |b, src, dst| {
            b.iter(|| {
                let config = base64::STANDARD;
                base64::decode_config_slice(black_box(src), config, black_box(dst))
            })
        }),
    ];

    for &(name, f) in functions {
        for src in &inputs {
            group.throughput(Throughput::Bytes(src.len() as u64));
            let id = BenchmarkId::new(name, src.len());
            let mut dst: Vec<u8> = vec![0; src.len() / 4 * 3 + 3];
            group.bench_with_input(id, src.as_slice(), |b, src| f(b, src, dst.as_mut_slice()));
        }
    }
}

criterion_group!(benches, bench_encode, bench_decode);
criterion_main!(benches);
