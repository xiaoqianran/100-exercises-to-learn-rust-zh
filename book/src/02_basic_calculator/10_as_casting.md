# 转换：`as`

`as` 可在部分原始类型之间转换：

```rust
let a = 47u16 as u32; // 拓宽，数值不变
let b = true as u8;   // true → 1, false → 0
```

缩窄或有符号/无符号转换可能截断或按补码解释，**需非常小心**。

## 练习

完成三个 `todo!()`。

```bash
cargo test -p as_cast
```
