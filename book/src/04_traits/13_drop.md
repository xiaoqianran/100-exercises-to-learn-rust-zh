# Drop

`Drop::drop` 在值离开作用域时调用，用于释放资源或执行清理。

「Drop bomb」模式：未调用 `defuse` 则在 drop 时 `panic!`，用于检测是否忘记关键逻辑。

```bash
cargo test -p drop
```
