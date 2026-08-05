// TODO: 修复下面的函数签名，让测试通过。
//  请认真阅读编译器报错——在本课程中，编译器就是你的结对编程伙伴！
//
// 参数类型应与返回类型相同。
fn compute(a, b) -> u32 {
    // 不要改函数体。
    a + b * 2
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 5);
    }
}
