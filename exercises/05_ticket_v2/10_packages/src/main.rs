// 这是 `main.rs`，因此 cargo 会把它当作 binary 目标的根。

// TODO: 修复这个无法解析的导入。在 `src` 目录创建新的 library 目标。
//   库应公开函数 `hello_world`：无参数、无返回值。
use packages::hello_world;

// binary 入口
fn main() {
    hello_world();
}
