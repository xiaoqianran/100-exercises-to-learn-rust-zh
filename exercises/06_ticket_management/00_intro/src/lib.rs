fn intro() -> &'static str {
    // TODO: 在此填写字段: 修复下面这一行 👇
    "I'm ready to __!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to build a ticket management system!");
    }
}
