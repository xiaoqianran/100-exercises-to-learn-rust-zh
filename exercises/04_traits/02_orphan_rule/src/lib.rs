// TODO: 这是孤儿规则违规示例。
//  我们在为外部类型（`u32`，来自 std）实现外部 trait（`PartialEq`，来自 std）。
//  先看编译器报错长什么样。
//  然后删除下面代码，进入下一题。

impl PartialEq for u32 {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
