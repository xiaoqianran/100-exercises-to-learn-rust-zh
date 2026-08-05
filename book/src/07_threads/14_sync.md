# Send 与 Sync

- `Send`：所有权可转到另一线程
- `Sync`：可通过共享引用 `&T` 安全跨线程

```bash
cargo test -p sync
```
