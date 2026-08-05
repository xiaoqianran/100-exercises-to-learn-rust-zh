// TODO: 为 `min` 添加必要的 trait bound，使其通过编译。
//   更多比较相关 trait 见 `std::cmp` 文档。
//
// 注意：多种 bound 都能让编译器满意，但**语义**不同。
// 后面讲有序集合（如 BTreeMap）时会再区分。

/// 返回两个值中的较小者。
pub fn min<T>(left: T, right: T) -> T {
    if left <= right {
        left
    } else {
        right
    }
}
