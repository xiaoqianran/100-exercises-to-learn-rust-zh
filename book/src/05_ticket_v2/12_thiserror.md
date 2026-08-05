# thiserror

用过程宏减少 Error/Display 样板代码：

```rust
#[derive(Debug, thiserror::Error)]
enum E {
  #[error("Title cannot be empty")]
  TitleCannotBeEmpty,
}
```

```bash
cargo test -p thiserror_
```
