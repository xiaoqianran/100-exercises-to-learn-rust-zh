#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(2);
        v.push(1);
        v.push(2); // max capacity reached
        assert_eq!(v.capacity(), 2);

        v.push(3); // beyond capacity, needs to resize

        // 能猜出新的 capacity 吗？
        // 注意：标准库不保证扩容算法，未来可能变化。
        assert_eq!(v.capacity(), todo!());
    }
}
