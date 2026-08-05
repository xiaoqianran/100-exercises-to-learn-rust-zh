// TODO: 为 `WrappingU32` 实现 `From`，让 `example` 通过编译。

pub struct WrappingU32 {
    value: u32,
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
