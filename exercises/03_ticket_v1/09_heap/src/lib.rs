pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: 根据本节所学，将 `todo!()` 替换为对应类型的**栈占用字节数**。
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), todo!());
    }

    #[test]
    fn ticket_size() {
        // 这题有点绕！
        // 本题里「直觉答案」恰好正确；但一般情况下结构体内存布局更复杂。
        // 若好奇，可读 The Rust Reference 的 Type layout：
        // https://doc.rust-lang.org/reference/type-layout.html
        assert_eq!(size_of::<Ticket>(), todo!());
    }
}
