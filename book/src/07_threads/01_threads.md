# 线程

`thread::spawn` 启动，`join` 等待。闭包数据需满足 `'static`+`Send`（普通 spawn）。

```bash
cargo test -p threads
```
