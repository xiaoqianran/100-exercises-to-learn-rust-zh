// 自定义 `dev` profile，使溢出时环绕（wrap around）。
// 查阅 Cargo 文档了解正确语法：
// https://doc.rust-lang.org/cargo/reference/profiles.html
//
// 由于稍后会解释的原因，该配置必须写在**仓库根目录**的 `Cargo.toml`，
// 而不是本练习自己的 `Cargo.toml`。

pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        // 20! = 2432902008176640000，超出 u32 范围
        // 默认 dev profile 下运行 `cargo test` 会因溢出而 panic
        // 我们希望它改为环绕（wrap）
        assert_eq!(factorial(20), 2_192_834_560);
        //                           ☝️
        // 大数字字面量可用下划线提高可读性！
    }

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
