// 👇 以 `///` 开头的是**文档注释**，会附着到紧随其后的条目（这里是 `speed` 函数）。
//    在本练习目录运行 `cargo doc --open`，Rust 会根据这些注释生成 HTML 文档。

/// 给定行程的起点、终点以及耗时，计算平均速度。
pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: 定义名为 `distance` 的变量，使测试通过
    //  需要标注 `distance` 的类型吗？为什么？

    // 不要改下面这一行
    distance / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(speed(10, 30, 10), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(speed(10, 31, 10), 2);
    }
}
