# 所有权

每个值有唯一**所有者**。所有者离开作用域时值被丢弃。

## 移动（move）

把值赋给另一变量或传入吃掉 `self` 的函数，会**移动**所有权，原变量失效。

## 借用（borrow）

- `&T`：不可变借用，可同时存在多个
- `&mut T`：可变借用，同一时间只能有一个

访问器应优先：

```rust
pub fn title(&self) -> &String {
    &self.title
}
```

而不是吃掉 `self`，否则无法连续读取多个字段。

```bash
cargo test -p ownership
```
