// cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion};
// black_box 可以阻止编译器进行优化，将整个函数折叠成常数并用常数替换。
// criterion_group 基准测试组
// criterion_main 执行基准测试

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci_plus(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    match n {
        0 => b,
        1 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

// 基准测试方式一
// fn criterion_benchmark(c: &mut Criterion) { 是criterion宝箱基准测试的标准写法
fn criterion_benchmark_1(c: &mut Criterion) {
    // bench_function 函数
    // 第一个参数，类型是字符串字面量；是基准测试的ID；需要是基准测试里全局唯一的。
    // 第二个参数，类型是带有Bencher参数的闭包函数；通过Bencher对函数进行基准测试。
    c.bench_function("fibonacci 30", |b| b.iter(|| fibonacci(black_box(30))));
}

// 基准测试方式二
// 如果函数只需要传递一个参数，则可以使用bench_with_input函数来进行基准测试，自动传入black_box。
use criterion::BenchmarkId;
fn criterion_benchmark_2(c: &mut Criterion) {
    let num: u64 = 33;

    c.bench_with_input(BenchmarkId::new("fibonacci_plus 30", num), &num, |b, &s| {
        b.iter(|| fibonacci_plus(s));
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark_1, criterion_benchmark_2,

}
criterion_main!(benches);
