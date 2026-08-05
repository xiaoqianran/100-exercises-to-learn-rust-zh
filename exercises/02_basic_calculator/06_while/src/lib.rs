// 用 `while` 循环重写阶乘函数。
pub fn factorial(n: u32) -> u32 {
    // `todo!()` 是占位宏：编译器会当作「稍后实现」，从而暂时压制部分类型错误。
    // 运行时会 panic。
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
