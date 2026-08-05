# Deref

`String` 实现了 `Deref<Target = str>`，因此 `String` 上可直接调用 `str` 的方法，例如 `trim`：

```rust
pub fn title(&self) -> &str {
    self.title.trim()
}
```

```bash
cargo test -p deref
```
