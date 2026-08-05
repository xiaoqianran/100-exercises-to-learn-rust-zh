# impl Trait vs 泛型参数

参数位置 `impl Into<T>` 等价于 `<T: Into<Ticket>>(t: T)` 的简写。

```bash
cargo test -p impl_trait_2
```
