// 定义过程宏

extern crate proc_macro;

use proc_macro::TokenStream;

// 属性宏 （两个参数）
#[proc_macro_attribute]
pub fn my_attr_macro(attr:TokenStream, item:TokenStream)->TokenStream{
    println!("Attr:{}", attr.to_string());
    println!("Item:{}", item.to_string());
    item
}

// 函数式宏
#[proc_macro]
pub fn my_func_macro(item: TokenStream) -> TokenStream {
    let name = item.to_string();
    let function = "Hello ".to_string() + name.as_ref();
    let fn_name =
        "fn hello_".to_string() + name.as_ref() + "(){ println!(\"" + function.as_ref() + "\"); }";
    fn_name.parse().unwrap()
}

// 派生宏
#[proc_macro_derive(Hello)]
pub fn my_derived_macro(input: TokenStream)-> TokenStream {
    println!("{:?}", input);
    TokenStream::new()  
    // 如果直接返回input，编译会报重复定义，说明派生宏用于扩展定义
    // input
}