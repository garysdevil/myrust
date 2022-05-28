// tests/attribute_macro.rs

use custom_macro::*;

#[test]
fn test_macro() {
    my_func_macro!(张三); // 使用函数宏生成一个函数，名为hello_张三()的函数
    hello_张三(); // 执行被生成的函数
}
