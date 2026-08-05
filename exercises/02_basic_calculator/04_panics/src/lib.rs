/// 给定行程起点、终点与耗时，计算平均速度。
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: 若 `time_elapsed` 为 0，用自定义消息 panic

    (end - start) / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    // 👇 `#[should_panic]` 表示我们期望被测代码发生 panic。
    //    还可用 `expected` 校验 panic 消息。这是 Rust 内置测试框架的能力！
    #[should_panic(expected = "The journey took no time at all. That's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
