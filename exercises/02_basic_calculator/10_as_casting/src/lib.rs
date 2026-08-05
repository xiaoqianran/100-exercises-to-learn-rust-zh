// TODO: 根据本节所学，把 `todo!()` 换成转换后的正确值。

#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() {
        let v: u32 = todo!();
        assert_eq!(47u16 as u32, v);
    }

    #[test]
    fn u8_to_i8() {
        // 编译器足够聪明，知道 255 放不进 i8，因此会硬错误。
        // 我们故意关闭该防护，以便演示这种（糟糕的）转换。
        // 编译器能发现是因为值是字面量；若是变量则编译期抓不到。
        #[allow(overflowing_literals)]
        let x = { 255 as i8 };

        // 你可以用和上面完全一样的表达式「蒙混过关」，但这违背练习目的。
        // 请给出一个真正的 `i8` 值，使其按位/转换后与 `255 as i8` 相等。
        let y: i8 = todo!();

        assert_eq!(x, y);
    }

    #[test]
    fn bool_to_u8() {
        let v: u8 = todo!();
        assert_eq!(true as u8, v);
    }
}
