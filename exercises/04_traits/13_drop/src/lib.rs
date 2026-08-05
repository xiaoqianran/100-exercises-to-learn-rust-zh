// TODO: 实现所谓 “Drop bomb”：被 drop 时会 panic 的类型，
//  除非事先对它执行了某个操作。
//  期望 API 见下方测试。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // bomb 在 drop 时应 panic
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // 已拆除，drop 时不应 panic
    }
}
