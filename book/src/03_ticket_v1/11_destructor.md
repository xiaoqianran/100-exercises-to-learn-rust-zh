# 析构

变量的**作用域**从声明开始，到块结束或所有权被移走为止。

所有者离开作用域时，Rust 调用其**析构函数**清理资源（例如释放 `String` 堆内存）。  
也可手动 `std::mem::drop(value)`。社区常说「值被 drop 了」。

编译器在合适的点插入 drop，你很少需要手动 `free`。

```bash
cargo test -p destructor
```
