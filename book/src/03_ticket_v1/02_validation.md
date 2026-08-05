# 校验

构造函数（如 `Ticket::new`）是放置**不变量**检查的好地方：标题非空、长度限制、状态枚举值等。  
不满足时可用 `panic!`（本章后续会学更好的错误处理）。

常用 `String` 方法：

- `is_empty`
- `len`（**字节**长度）

允许的状态：`To-Do`、`In Progress`、`Done`。

```bash
cargo test -p validation
```
