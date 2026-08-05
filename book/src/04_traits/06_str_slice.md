# 字符串切片 `&str`

`&String` 只能指向 `String`；`&str` 是字符串切片，更通用（也可指向字面量）。

API 设计上，getter 常返回 `&str`：

```rust
pub fn title(&self) -> &str {
    &self.title
}
```

```bash
cargo test -p str_slice
```
