mod ticket {
    struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        fn new(title: String, description: String, status: String) -> Ticket {
            if title.is_empty() {
                panic!("Title cannot be empty");
            }
            if title.len() > 50 {
                panic!("Title cannot be longer than 50 bytes");
            }
            if description.is_empty() {
                panic!("Description cannot be empty");
            }
            if description.len() > 500 {
                panic!("Description cannot be longer than 500 bytes");
            }
            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
            }

            Ticket {
                title,
                description,
                status,
            }
        }
    }
}

// TODO: **本题例外**——需要同时修改 `ticket` 模块与 `tests` 模块。
#[cfg(test)]
mod tests {
    // TODO: 在父模块加上必要的 `pub`，消除下面 `use` 的可见性错误。
    use super::ticket::Ticket;

    // 注意：让 `use` 通过后，我们仍希望下面函数**无法**编译！
    // 验证确实不能编译后，再把它注释掉。
    fn should_not_be_possible() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());

        // 你应看到类似错误：
        // error[E0616]: field `description` of struct `Ticket` is private
        //
        // TODO: 确认无法编译后，注释掉下面这行再继续。
        assert_eq!(ticket.description, "A description");
    }

    fn encapsulation_cannot_be_violated() {
        // 同样应无法编译：证明只能通过 `Ticket::new` 创建实例，
        // 不能绕过校验用结构体字面量塞非法数据。
        //
        // TODO: 确认无法编译后，注释掉下面几行再继续。
        let ticket = Ticket {
            title: "A title".into(),
            description: "A description".into(),
            status: "To-Do".into(),
        };
    }
}
