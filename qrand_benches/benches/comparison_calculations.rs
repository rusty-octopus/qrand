use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

static SOBOL_DIRECTION_NUMBERS_DIM1: [u32; 32] = [
    0x80000000, 0x40000000, 0x20000000, 0x10000000, 0x08000000, 0x04000000, 0x02000000, 0x01000000,
    0x00800000, 0x00400000, 0x00200000, 0x00100000, 0x00080000, 0x00040000, 0x00020000, 0x00010000,
    0x00008000, 0x00004000, 0x00002000, 0x00001000, 0x00000800, 0x00000400, 0x00000200, 0x00000100,
    0x00000080, 0x00000040, 0x00000020, 0x00000010, 0x00000008, 0x00000004, 0x00000002, 0x00000001,
];

fn sobol_par_u32(n: usize, d: usize) -> u32 {
    let mut n_inner = n;
    let mut value: u32 = 0;
    let mut index = 0;
    while n_inner > 0 {
        if n_inner & 1 == 1 {
            let direction_number = SOBOL_DIRECTION_NUMBERS_DIM1[32 * d + index];
            value ^= direction_number;
        }
        index += 1;
        n_inner >>= 1;
    }
    value
}

static GRAY_CODE_RE_ORDER: [usize; 1] = [0];

fn sobol_seq_u32(n: usize, d: usize) -> u32 {
    // this implementation is incorrect and just emulates a sequential calculations
    n as u32 ^ SOBOL_DIRECTION_NUMBERS_DIM1[GRAY_CODE_RE_ORDER[d]]
}

static RD_ALPHAS_F64: [f64; 1] = [0.7548776662466927];

fn rd_f64_par(n: usize, d: usize) -> f64 {
    n as f64 * RD_ALPHAS_F64[d]
}

static RD_ALPHAS_U64: [u64; 1] = [0xc13fa9a902a63000];

fn rd_u64_par(n: usize, d: usize) -> u64 {
    ((n as u128 * RD_ALPHAS_U64[d] as u128) >> 64) as u64
}

fn compare_calculations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Low discrepancy inner calculation");
    let mut value = 0xFF;
    let inc = 0xFF;

    for _ in 0..4 {
        group.bench_function(BenchmarkId::new("sobol par", value), |b| {
            b.iter(|| sobol_par_u32(black_box(value), black_box(0)))
        });
        group.bench_function(BenchmarkId::new("sobol seq", value), |b| {
            b.iter(|| sobol_seq_u32(black_box(value), black_box(0)))
        });
        group.bench_function(BenchmarkId::new("rd f64 par", value), |b| {
            b.iter(|| rd_f64_par(black_box(value), black_box(0)))
        });
        group.bench_function(BenchmarkId::new("rd u64 par", value), |b| {
            b.iter(|| rd_u64_par(black_box(value), black_box(0)))
        });
        value = value << 8 | inc;
    }
}

criterion_group!(
    name = benches; 
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = compare_calculations);
criterion_main!(benches);
