# Sized

大多数类型实现了 `Sized`（编译期已知大小）。  
`str`、`[T]` 等是 **DST**（动态大小类型），不能写 `size_of::<str>()`。

应使用 `&str` 等胖/指针形式。

```bash
cargo test -p sized
```
