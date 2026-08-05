// TODO: 本 crate 每个模块里都有任务要完成！
mod description;
mod status;
mod title;

// 常见模式：拆成多个（私有）模块，再在 crate 根 re-export 公开部分。
// 对用户隐藏内部结构，同时保持内部任意组织。
pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone)]
// 字段不必再私有！
// 每个字段类型自己封装校验，用户改字段也不会轻易破坏不变量。
//
// 但若存在跨字段不变量，仍需私有字段并自行维护。
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
