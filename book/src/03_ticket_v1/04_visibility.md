# 可见性

Rust **默认私有**。私有条目仅：

1. 定义它的模块，或
2. 其子模块

可访问。

## 常用修饰符

- `pub`：对外公开
- `pub(crate)`：crate 内公开
- `pub(super)`：对父模块公开

字段也可以单独控制可见性：结构体公开但字段私有 → 外界无法直接读写字段。

```bash
cargo test -p visibility
```
