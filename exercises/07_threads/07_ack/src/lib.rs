use std::sync::mpsc::{Receiver, Sender};
use crate::store::TicketStore;

pub mod data;
pub mod store;

// 参考测试以了解期望的命令结构。
pub enum Command {
    Insert { todo!() },
    Get { todo!() }
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: 按期望处理入站命令。
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {}) => {
                todo!()
            }
            Ok(Command::Get {
                todo!()
            }) => {
                todo!()
            }
            Err(_) => {
                // 没有更多 sender，可安全退出并关闭服务器。
                break
            },
        }
    }
}
