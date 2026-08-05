# 枚举

```rust
enum Status { ToDo, InProgress, Done }
```

比字符串状态更类型安全。常配合 `#[derive(Debug, PartialEq)]`。

```bash
cargo test -p enum_
```
