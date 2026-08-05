use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    Insert(todo!()),
}

// 启动系统：spawn 服务端线程。
// 返回 `Sender`，供一个或多个客户端与服务器交互。
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: 服务端任务应**永不**停止。
//  循环：等待通道命令 → 执行 → 再等待下一条。
pub fn server(receiver: Receiver<Command>) {}
