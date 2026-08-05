# 饱和运算

饱和运算在溢出时夹到类型的最小/最大值，而不是 panic 或 wrap。

例如：`u32::saturating_mul`。

## 练习

阶乘改用饱和乘法，使 `factorial(20) == u32::MAX`。

```bash
cargo test -p saturating
```
