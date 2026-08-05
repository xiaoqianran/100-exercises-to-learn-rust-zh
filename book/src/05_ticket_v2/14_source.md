# Error::source

错误链：外层错误通过 `source()` 指向原因。thiserror 可用 `#[source]` / `#[from]`。

```bash
cargo test -p source
```
