# 生命周期

`impl<'a> IntoIterator for &'a TicketStore` 让 `for t in &store` 工作。

```bash
cargo test -p lifetime
```
