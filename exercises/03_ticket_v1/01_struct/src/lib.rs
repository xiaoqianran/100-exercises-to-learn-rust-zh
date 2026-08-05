// 定义结构体 `Order`，包含以下字段：
// - `price`：无符号整数
// - `quantity`：无符号整数
//
// 还需要方法 `is_available`：当 quantity > 0 时返回 `true`，否则 `false`。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}
