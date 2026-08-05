// TODO: 重新实现 `Ticket` 的访问器：这次返回 `&str`，而不是 `&String`。

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
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

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{valid_description, valid_title};
    use std::any::{Any, TypeId};

    #[test]
    fn test_type() {
        let ticket = Ticket::new(valid_title(), valid_description(), "To-Do".to_string());
        // 一点「黑魔法」：校验你用了期望的返回类型
        assert_eq!(TypeId::of::<str>(), ticket.title().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.description().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.status().type_id());
    }
}
