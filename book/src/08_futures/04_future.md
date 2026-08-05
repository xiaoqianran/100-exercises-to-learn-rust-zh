# Future 与 Send

跨 `.await` 持有的状态必须满足 spawn 对 `Send` 的要求。

```bash
cargo test -p future
```
