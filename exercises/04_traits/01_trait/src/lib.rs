// 定义 trait `IsEven`：方法 `is_even` 在 `self` 为偶数时返回 true，否则 false。
//
// 然后为 `u32` 与 `i32` 实现该 trait。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_is_even() {
        assert!(42u32.is_even());
        assert!(!43u32.is_even());
    }

    #[test]
    fn test_i32_is_even() {
        assert!(42i32.is_even());
        assert!(!43i32.is_even());
        assert!(0i32.is_even());
        assert!(!(-1i32).is_even());
    }
}
