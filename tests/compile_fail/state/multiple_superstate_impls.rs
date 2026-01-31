use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    struct Top;
    impl TopState for Top {}

    struct Middle;
    impl Substate<Top> for Middle {}

    struct Bottom;
    impl Substate<Top> for Bottom {}
    impl Substate<Middle> for Bottom {}
}

fn main() {}
