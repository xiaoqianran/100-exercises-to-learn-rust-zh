# 带数据的变体

```rust
enum Status {
  ToDo,
  InProgress { assigned_to: String },
  Done,
}
```

用模式匹配取出字段。

```bash
cargo test -p variants_with_data
```
