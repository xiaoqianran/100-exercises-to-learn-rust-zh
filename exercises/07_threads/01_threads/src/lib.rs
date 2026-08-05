// TODO: 使用 `spawn` 与 `join` 实现多线程版 `sum`。
//  将整数向量拆成两半，分别在独立线程中求和。

// 注意：测试无法验证「如何」实现，只能验证结果正确。
// 你也可以 `v.iter().sum()` 蒙混过关，但那就失去练习意义。
//
// 提示：普通 spawn 难以直接借用向量切片，需要为两半分配新的 Vec。
// 下一题会解释原因。
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
