# Clone

`Clone` 显式复制：`value.clone()`。  
当函数要消费值又要保留一份时，先 `clone` 再 move。

可 `#[derive(Clone)]`（字段都实现 Clone 时）。

```bash
cargo test -p clone
```
