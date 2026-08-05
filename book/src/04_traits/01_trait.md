# Trait

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
```

实现 trait 后，可在该类型上调用 trait 方法（需在作用域内 `use` trait，本练习同文件通常已可见）。

```bash
cargo test -p trait_
```
