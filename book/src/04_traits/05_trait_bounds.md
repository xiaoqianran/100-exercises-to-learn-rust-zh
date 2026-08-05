# Trait 约束

泛型代码若要调用某行为，需声明 bound：

```rust
pub fn min<T: PartialOrd>(left: T, right: T) -> T {
    if left <= right { left } else { right }
}
```

`PartialOrd` 提供偏序比较（含 `<=`）。

```bash
cargo test -p trait_bounds
```
