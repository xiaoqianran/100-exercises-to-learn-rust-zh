//! TODO: 仅通过**重排** `example` 中的语句使代码编译。
//!  不许改 `spawner`，也不许改变 `example` 每行在做什么。
//!  如需要可用 `{}` 包住语句形成作用域。
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
