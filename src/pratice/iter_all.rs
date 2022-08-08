// 自定义结构体实现迭代器特性
// 使用标准库中定义好的Iterator特性所实现的一些关于迭代器的关联方法
#![allow(dead_code)]

struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    // 实现迭代器特性
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // 实现迭代器接口
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1)) // zip函数将两个迭代器合成一个迭代器对
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}
