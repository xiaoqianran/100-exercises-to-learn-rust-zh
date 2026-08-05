# 异步原语

在 async 代码中使用 tokio 的 channel/Mutex 等，避免 std 阻塞原语。

```bash
cargo test -p async_locks
```
