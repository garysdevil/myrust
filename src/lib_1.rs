// 官方基准测试工具
// rustup install nightly
// rustup override set nightly 
#![feature(test)] // 官方提供的测试工具，只能在非 stable 版本下使用。
extern crate test;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// cargo bench
// cargo bench
#[cfg(test)]
pub mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        // b.iter(|| add_two(2));
        // b.iter(|| test::black_box(add_two(2)));
        b.iter(|| 
            {
            for i in 100..200 {
                // add_two(i); // 基准测试没结果。 在LLVM中: LLVM认为fibonacci_u64函数调用的结果没有使用，同时也认为该函数没有任何副作用(造成其它的影响，例如修改外部变量、访问网络等), 因此它有理由把这个函数调用优化掉。
                test::black_box(add_two(test::black_box(i))); // 使用 Rust 标准库中的 black_box 函数，告诉编译器，让它尽量少做优化。
            }
            }
        );
    }
}