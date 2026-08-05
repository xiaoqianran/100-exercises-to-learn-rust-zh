# 溢出与下溢

在 **debug** 默认配置下，整数溢出会 **panic**。  
在 **release** 下通常会 **环绕（wrap）**（依类型按模运算）。

你可以在 **仓库根** `Cargo.toml` 用 profile 覆盖某包行为，例如对本练习包关闭溢出检查 / 允许 wrap（以 Cargo 文档为准）。

## 练习

配置后使 `factorial(20)` 得到 `2_192_834_560`。

```bash
cargo test -p overflow
```

注意：改的是**根目录** `Cargo.toml`。
