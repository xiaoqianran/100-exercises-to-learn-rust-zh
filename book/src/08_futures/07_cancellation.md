# 取消

`timeout` 取消 Future 时，已发生的副作用（如已读入 buffer）不会自动回滚。

```bash
cargo test -p cancellation
```
