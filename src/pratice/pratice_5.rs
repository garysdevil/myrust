// 使用自定义的过程宏 Procedural Macro

#![allow(dead_code)]

#[cfg(feature2)]
extern crate custom_macro;

#[cfg(feature = "feature2")]
pub fn main() {
    use custom_macro::my_attr_macro;
    use custom_macro::my_func_macro;
    use custom_macro::Hello;

    // 使用函数宏
    my_func_macro!(world);
    my_func_macro!(张三);

    // 使用属性宏
    #[my_attr_macro(struct, "world")]
    struct Hello {
        pub name: String,
    }

    // 使用属性宏
    #[my_attr_macro(func, "test")]
    fn invoked() {}

    // 使用派生宏
    #[derive(Hello)]
    struct World;

    // fn main() {
    // 使用my_func_macro生成的函数
    hello_world();
    hello_张三();
    // }
}
