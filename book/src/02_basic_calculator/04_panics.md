# 恐慌（panic）

遇到不可恢复的错误时，可以用 `panic!` 终止当前线程：

```rust
panic!("出错了：{}", 原因);
```

测试中可用 `#[should_panic(expected = "...")]` 断言应当 panic，并匹配消息子串。

## 练习

当 `time_elapsed == 0` 时 panic，消息必须为：

`The journey took no time at all. That's impossible!`

```bash
cargo test -p panics
```
