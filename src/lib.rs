#![allow(unused)]

mod input;
mod output;

pub trait State<T>: Default {
    fn enter(&mut self) -> Option<T> {
        None
    }

    fn exit(&mut self) -> Option<T> {
        None
    }

    fn update(&mut self) -> Option<T> {
        None
    }
}

trait SubState<T> {
    fn update(&mut self) -> Option<T>;
    fn transition(&mut self, new_state: T);
}

trait LeafState<T>: State<T> {}

trait BranchState<T>: State<T> {
    fn init(&mut self) -> T;
}
