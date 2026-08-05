// TODO: 通过访问器返回 `title` / `description` 时，应先规范化——
//   去掉首尾空白。
//   标准库里有方法能帮忙，但你在 `String` 的文档里未必找得到。
//   它定义在哪里？如何调用？

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn title(&self) -> &str {
        todo!()
    }

    pub fn description(&self) -> &str {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let ticket = Ticket {
            title: "   A title ".to_string(),
            description: " A description   ".to_string(),
            status: "To-Do".to_string(),
        };

        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
