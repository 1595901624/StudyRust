# Drop receiver demo

本文示例用于说明 `Drop::drop` 为什么接收 `&mut self`，以及 Rust 完整析构过程中的两个阶段：

1. 调用当前类型实现的 `Drop::drop`；
2. 执行编译器生成的 drop glue，递归析构字段。

## 运行正常示例

```shell
cargo run
```

## 运行测试

```shell
cargo test
cargo clippy --all-targets -- -D warnings
```

## 查看故意保留的编译错误

`compile_fail` 目录中的代码用于复现文章里的编译错误，不属于 Cargo 的正常构建目标。

```shell
rustc compile_fail/move_out_of_drop.rs
rustc compile_fail/call_drop_directly.rs
```

两个命令应该分别报告：

```text
E0509: cannot move out of type `Worker`, which implements the `Drop` trait
E0040: explicit use of destructor method
```
