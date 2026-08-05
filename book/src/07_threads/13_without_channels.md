# 不用通道

也可直接共享 `TicketStore`（加锁），不一定要 actor 模型。

```bash
cargo test -p without_channels
```
