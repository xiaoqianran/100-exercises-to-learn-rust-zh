# Copy

`Copy` 是隐式按位复制（仅适用于简单类型），需要先 `Clone`。  
实现运算符 trait（如 `Add`）可写 `a + b`。

包装 `u32` 时可 `#[derive(Copy, Clone, PartialEq, Debug)]` 再 `impl Add`。

```bash
cargo test -p copy
```
