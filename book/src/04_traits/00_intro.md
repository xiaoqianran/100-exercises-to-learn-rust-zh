# 特征 Traits

Trait 描述**共享行为**：定义一组方法签名，类型通过 `impl Trait for Type` 提供实现。

本章将学习：

- 定义与实现 trait
- 孤儿规则
- 运算符重载（如 `PartialEq`）
- `derive` 宏
- trait bound
- `&str` / `Deref` / `Sized`
- `From` / `Clone` / `Copy` / `Drop`

```bash
cargo test -p intro_03
```
