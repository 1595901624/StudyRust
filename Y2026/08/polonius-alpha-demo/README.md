# Polonius Alpha 示例

这个工程对应《Polonius Alpha 来了，Rust 下一代借用检查器能解决什么问题？》中的代码

默认示例使用稳定版推荐的 Entry API

```powershell
cargo run
```

`examples/rejected_by_nll.rs` 保留了文章中稳定版会被拒绝的写法

```powershell
cargo +stable check --example rejected_by_nll
cargo +nightly check --example rejected_by_nll
```

前一条命令会复现稳定版的 `E0499` 借用冲突，后一条命令用于检查 nightly 的 Polonius Alpha

如果本机 nightly 版本尚未默认启用 Polonius Alpha，可以显式开启

```powershell
$env:RUSTFLAGS = "-Zpolonius"
cargo +nightly check --example rejected_by_nll
```

如果要在 nightly 中关闭 Polonius Alpha，可以在 PowerShell 中执行

```powershell
$env:RUSTFLAGS = "-Zpolonius=off"
cargo +nightly check --example rejected_by_nll
```

文章中的两段函数代码也分别保存在 `snippets` 目录中，便于对照阅读
