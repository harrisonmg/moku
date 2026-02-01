use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    struct Top;
    impl TopState for Top {}

    struct Bottom<T> {
        t: T,
    }

    impl<T> Substate<Top> for Bottom<T> {}
}

fn main() {}
