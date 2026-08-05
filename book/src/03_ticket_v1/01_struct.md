# 结构体

结构体把相关数据打成一组：

```rust
struct Order {
    price: u32,
    quantity: u32,
}
```

## 实例化

```rust
let order = Order {
    price: 100,
    quantity: 10,
};
```

## 方法

方法定义在 `impl` 块中，第一个参数通常是 `self` / `&self` / `&mut self`：

```rust
impl Order {
    fn is_available(&self) -> bool {
        self.quantity > 0
    }
}
```

## 练习

定义 `Order` 与 `is_available`。

```bash
cargo test -p struct_
```
