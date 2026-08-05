# 模块

模块用于分组与命名空间：

```rust
mod helpers {
    // ...
}
```

## 模块树

crate 根（如 `src/lib.rs`）是树根，可嵌套子模块。

## 路径

- `self`：当前模块
- `super`：父模块
- `crate`：crate 根

子模块可以访问父模块中的私有条目；外部模块则需要可见性修饰。

```bash
cargo test -p modules
```
