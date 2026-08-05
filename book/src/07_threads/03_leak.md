# 泄漏

`Vec::leak` 放弃释放，得到 `'static` 切片（教学用途，生产慎用）。

```bash
cargo test -p leaking
```
