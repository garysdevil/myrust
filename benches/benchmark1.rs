// cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
    // let mut a = 0;
    // let mut b = 1;
    // match n {
    //     0 => b,
    //     1 => b,
    //     _ => {
    //         for _ in 0..n {
    //              let c = a + b;
    //              a = b;
    //              b = c;
    //         }
    //         b
    //     }
    // }
}

fn criterion_benchmark1_1(c: &mut Criterion) {
    // bench_function 函数的第一个参数，是字符串字面量，需要是基准测试里全局唯一的
    c.bench_function("fibonacci 30", |b| b.iter(|| fibonacci(black_box(30))));
}

criterion_group!(benches, criterion_benchmark1_1);
criterion_main!(benches);
