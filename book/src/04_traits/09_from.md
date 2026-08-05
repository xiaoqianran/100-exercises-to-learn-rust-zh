# From / Into

```rust
impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        Self { value }
    }
}
```

实现 `From` 后自动获得 `Into`（在对应方向上），可用 `.into()` 与 `T::from`。

```bash
cargo test -p from
```
