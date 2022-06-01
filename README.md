# myrust

## 开发
```bash
# 运行
cargo run
# 运行满足特定条件的代码
cargo run --features feature1 

# 运行区块链模拟功能
cargo run --bin blockchain
```

```bash
# 测试
cargo test 
# 运行 模块名称或者测试函数名称 包含有特定字符串的测试
cargo test ${name_filter}
```

```bash
# 基准测试
cargo bench
# 运行特定的基准测试
cargo bench --bench benchmark1
```

## 生成文档
```bash
# 生成一个文件的文档
cd mydocs && rustdoc src/lib.rs
# 文档位置 doc/lib/index.html

# 生成整个项目的文档
cargo doc
# 文档位置 target/doc/lib/
```

## 提交代码
```bash
# 代码格式化
cargo fmt

# 提交代码
git
```

## 项目的创建过程
```bash
# 创建这个项目
cargo new myrust

# 创建lib项目 过程宏
cargo new custom-macro --lib

# 创建lib项目 工具包
cargo new myutils --lib

# 创建lib项目 区块链功能模拟
cargo new blockchain --lib
```