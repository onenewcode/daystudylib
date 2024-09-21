use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

/// 计算斐波那契数
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// 基准测试函数
fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    
    // 测试不同的输入值
    for i in [10, 20, 30].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(i), i, |b, &i| b.iter(|| fibonacci(i)));
    }
    
    group.finish();
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);