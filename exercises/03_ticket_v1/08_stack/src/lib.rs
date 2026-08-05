// TODO: 根据本节所学，将 `todo!()` 替换为对应类型的**栈占用字节数**。
#[cfg(test)]
mod tests {
    use std::mem::size_of;

    #[test]
    fn u16_size() {
        assert_eq!(size_of::<u16>(), todo!());
    }

    #[test]
    fn i32_size() {
        assert_eq!(size_of::<i32>(), todo!());
    }

    #[test]
    fn bool_size() {
        assert_eq!(size_of::<bool>(), todo!());
    }
}
