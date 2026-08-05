# 阻塞

不要在 async 里跑长时间同步 IO；用 `spawn_blocking`。

```bash
cargo test -p blocking
```
