pub fn example() {
    // 试图对 str（或任何 DST）使用 `std::mem::size_of`
    // 会在编译期报错。
    //
    // TODO: 注释掉下面一行，然后进入下一题。
    std::mem::size_of::<str>();
}
