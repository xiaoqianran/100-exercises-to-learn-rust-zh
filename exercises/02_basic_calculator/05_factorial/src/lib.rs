// 定义函数 `factorial`：给定非负整数 `n`，返回 `n!`（n 的阶乘）。
//
// `n!` 定义为从 1 到 n 的所有正整数之积。
// 例如 `5!`（读作「五的阶乘」）= `5 * 4 * 3 * 2 * 1` = `120`。
// 规定 `0! = 1`。
//
// 期望：`factorial(0) == 1`，`factorial(1) == 1`，`factorial(2) == 2`，以此类推。
//
// 只用目前学过的内容！还没有循环，请用**递归**实现。

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
