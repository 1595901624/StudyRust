# const fn 为什么不一定在编译期执行？

每个 `src/bin` 文件对应文章中的一个可运行示例。

```powershell
cargo run --bin const-context
cargo run --bin runtime-call
cargo run --bin const-panic
cargo run --bin runtime-panic
cargo run --bin const-block
```

`compile_fail/const_panic.rs` 对应文章中把 `percentage(0, 3)` 用在常量上下文时的编译失败示例。它不放在 Cargo 默认构建目录中，避免影响其他示例运行。
