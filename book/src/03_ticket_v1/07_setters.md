# 可变引用与 Setter

`&mut self` 允许在不转移所有权的情况下修改字段：

```rust
pub fn set_title(&mut self, new_title: String) {
    // 校验...
    self.title = new_title;
}
```

Setter 必须与构造器执行**相同校验**，可抽取私有函数复用。

也有吃掉 `self` 再返回 `Self` 的链式风格；本题要求 `&mut self` 风格。

```bash
cargo test -p setters
```
