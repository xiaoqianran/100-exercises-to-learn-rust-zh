fn compute(a: u32, b: u32) -> u32 {
    // TODO: 修改下面一行，消除编译错误并让测试通过。
    let multiplier: u8 = 4;
    a + b * multiplier
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
