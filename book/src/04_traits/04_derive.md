# 派生宏 derive

对「有明显默认实现」的 trait，可用：

```rust
#[derive(PartialEq, Debug)]
struct Ticket { /* ... */ }
```

`assert_eq!` 失败时要打印两侧值，因此通常还需要 `Debug`。

```bash
cargo test -p derives
```
