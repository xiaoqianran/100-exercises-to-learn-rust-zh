# 作用域线程

`thread::scope` 允许借用非 `'static` 局部数据，scope 结束前所有线程加入。

```bash
cargo test -p scoped_threads
```
