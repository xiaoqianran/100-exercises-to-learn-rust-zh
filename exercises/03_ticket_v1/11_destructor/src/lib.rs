// 要给「析构」写更完整的练习还需要更多机制。
// 学完 trait 与内部可变性后，我们会再回到这个概念。
fn outro() -> &'static str {
    "I have a basic understanding of __!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a basic understanding of destructors!");
    }
}
