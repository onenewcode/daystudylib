use criterion::{black_box, criterion_group, criterion_main, Criterion};
use half::f16;
use rand::{thread_rng, Rng};
use zcriterion::{gen_rand_block_q8_0_vec, vec_dot_q8, vec_dot_q8_0_q8_0_avx2, vec_dot_q8_naive, vec_dot_q8_stdsimd, BlockQ8_0};

pub fn tensor_benchmarks(c: &mut Criterion) {
    const TEST_BLOCKS: usize = 1000;
    const TEST_ELEMS: usize = TEST_BLOCKS * 32;

    let v1 = gen_rand_block_q8_0_vec(TEST_BLOCKS);
    let v2 = gen_rand_block_q8_0_vec(TEST_BLOCKS);

    c.bench_function("native q8", |b| {
        b.iter(|| {
            vec_dot_q8_naive(TEST_ELEMS, black_box(&v1), black_box(&v2));
        })
    });
    c.bench_function("stdsimd q8", |b| {
        b.iter(|| {
            vec_dot_q8_stdsimd(TEST_ELEMS, black_box(&v1), black_box(&v2));
        })
    });
    c.bench_function("avx2 q8", |b| {
        b.iter(|| {
            vec_dot_q8(black_box(&v1), black_box(&v2));
        })
    });
    c.bench_function("crabml avx2", |b| {
        b.iter(|| {
            vec_dot_q8_0_q8_0_avx2(black_box(&v1), black_box(&v2));
        })
    });
}

criterion_group!(benches, tensor_benchmarks);
criterion_main!(benches);
