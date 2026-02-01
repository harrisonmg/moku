use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    struct Event<T> {
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> StateMachineEvent for Event<T> {}

    struct Top;

    impl TopState for Top {}
}

fn main() {}
