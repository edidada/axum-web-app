use criterion::{criterion_group, criterion_main, Criterion};

fn bench_simple_operation(c: &mut Criterion) {
    c.bench_function("simple_operation", |b| {
        b.iter(|| {
            // 简单的操作，用于测试基准测试框架是否正常工作
            let mut sum = 0;
            for i in 0..1000 {
                sum += i;
            }
            sum
        });
    });
}

criterion_group!(benches, bench_simple_operation);
criterion_main!(benches);
