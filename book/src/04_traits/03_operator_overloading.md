# 运算符重载

许多运算符由 trait 定义，例如 `==` 来自 `PartialEq`：

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.description == other.description
            && self.status == other.status
    }
}
```

```bash
cargo test -p overloading
```
