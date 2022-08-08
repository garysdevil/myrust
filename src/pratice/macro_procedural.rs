// 使用自定义的过程宏 Procedural Macro
#![allow(dead_code)]
#[cfg(feature = "feature_custommacro")]
use custom_macro::my_attr_macro;
#[cfg(feature = "feature_custommacro")]
use custom_macro::my_func_macro;
#[cfg(feature = "feature_custommacro")]
use custom_macro::Hello;

#[cfg(feature = "feature_custommacro")]
pub fn main() {
    example_func_macro();
    example_attri_macro();
    example_derive_macro();
}

#[cfg(feature = "feature_custommacro")]
fn example_func_macro() {
    // 使用函数宏
    my_func_macro!(world);
    my_func_macro!(张三);

    // 使用函数宏my_func_macro生成的函数
    hello_world();
    hello_张三();
}

#[cfg(feature = "feature_custommacro")]
fn example_attri_macro() {
    #[my_attr_macro(struct, "world")] // 使用属性宏
    struct Hello {
        pub name: String,
    }

    #[my_attr_macro(func, "test")] // 使用属性宏
    fn invoked() {}
}

#[cfg(feature = "feature_custommacro")]
fn example_derive_macro() {
    // 使用派生宏
    #[derive(Hello)]
    struct World;
}
