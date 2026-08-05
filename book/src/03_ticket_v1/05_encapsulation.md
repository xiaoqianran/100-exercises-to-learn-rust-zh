# 封装

封装隐藏内部表示，通过公开 API 维护不变量。

- 字段私有
- 公开构造器 `Ticket::new` 做校验
- 公开 getter 读取状态

若至少有一个字段私有，外部不能用结构体字面量绕过 `new`。

```bash
cargo test -p encapsulation
```
