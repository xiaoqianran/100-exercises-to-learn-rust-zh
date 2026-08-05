// TODO: 使用 `Rc` 与 `RefCell` 实现 `DropTracker<T>`：
//  包装 `T`，每次被 drop 时递增共享的 `usize` 计数器。

use std::cell::RefCell;
use std::rc::Rc;

pub struct DropTracker<T> {
    value: T,
    counter: todo!(),
}

impl<T> DropTracker<T> {
    pub fn new(value: T, counter: todo!()) -> Self {
        Self { value, counter }
    }
}

impl<T> Drop for DropTracker<T> {
    fn drop(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let counter = Rc::new(RefCell::new(0));
        let _ = DropTracker::new((), Rc::clone(&counter));
        assert_eq!(*counter.borrow(), 1);
    }

    #[test]
    fn multiple() {
        let counter = Rc::new(RefCell::new(0));

        {
            let a = DropTracker::new(5, Rc::clone(&counter));
            let b = DropTracker::new(6, Rc::clone(&counter));
        }

        assert_eq!(*counter.borrow(), 2);
    }
}
