# 分支：`if` / `else`

```rust
if 条件 {
    // ...
} else if 另一条件 {
    // ...
} else {
    // ...
}
```

条件必须是 `bool`。`if` 也可以是表达式：

```rust
let x = if flag { 1 } else { 2 };
```

各分支返回类型必须一致。

## 练习

按优先级实现 `magic_number`：**先偶数 → 12**，否则能被 3 整除 → 13，否则 → 17。

```bash
cargo test -p if_else
```
