# 孤儿规则

为避免多个 crate 为同一对（类型, trait）提供冲突实现，Rust 规定：

> **实现 trait 时，类型或 trait 至少有一个在当前 crate 定义。**

因此不能写 `impl PartialEq for u32`（二者都来自标准库）。

本题故意违规：阅读报错后删除代码即可。

```bash
cargo test -p orphan
```
