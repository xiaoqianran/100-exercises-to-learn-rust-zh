// 给定 `n`，返回斐波那契数列的第 `n+1` 项（从 0 起算下标 n）。
//
// 定义：
// - 第 1 个数为 0
// - 第 2 个数为 1
// - 之后每一项为前两项之和
//
// 序列：0, 1, 1, 2, 3, 5, 8, 13, 21, ...
//
// 期望：`fibonacci(0)==0`，`fibonacci(1)==1`，`fibonacci(2)==1`，……
pub fn fibonacci(n: u32) -> u32 {
    // TODO: 在此填写字段: implement the `fibonacci` function
    //
    // Hint: use a `Vec` to memoize the results you have already calculated
    // so that you don't have to recalculate them several times.
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
