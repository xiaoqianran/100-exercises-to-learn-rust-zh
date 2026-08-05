# 变量

用 `let` 绑定名字与值：

```rust
let x = 5;
let y: u32 = 10;
```

## 类型推断

很多情况下可省略类型标注，编译器会从用法推断。在函数签名等边界位置则通常需要写明。

## 不可变默认

默认绑定不可变。若要重新赋值，需 `let mut`：

```rust
let mut n = 1;
n = 2;
```

## 遮蔽（shadowing）

可以用同名 `let` 再次绑定（甚至改类型），这叫遮蔽，与 `mut` 不同。

## 练习：平均速度

实现 `distance`，使 `distance / time_elapsed` 得到平均速度（整数除法会截断）。

```bash
cargo test -p variables
```
