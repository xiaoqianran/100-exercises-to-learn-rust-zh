# 关联类型 vs 泛型参数

Trait 可用泛型参数（如 `Power<RHS>`）或关联类型（`type Output`）表达输出/输入关系。

- **泛型参数**：同一类型可对多种 RHS 分别实现
- **关联类型**：每个实现只固定一种关联类型

本题为 `u32` 实现以 `u16` / `u32` / `&u32` 为指数的 `power`。

```bash
cargo test -p assoc_vs_generic
```
