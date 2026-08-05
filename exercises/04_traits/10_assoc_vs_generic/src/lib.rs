// TODO: 定义新 trait `Power`，方法 `power` 计算 `self` 的 `n` 次幂。
//  trait 定义及其实现应足以让测试编译并通过。
//
// 建议：你可能想写一个包打天下的泛型实现，但那相当复杂，
// 往往还要额外 crate（如 `num-traits`）。
// 即便如此，有时用简单宏比高度泛化更合适。
// 若感兴趣可读《Little book of Rust macros》：
// https://veykril.github.io/tlborm/
// 当然也可以老老实实写三个独立 impl——好奇再深入即可。

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
