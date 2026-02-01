use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    struct Top;
    impl TopState for Top {}

    struct Foo;

    impl Substate<Bar> for Foo {}

    struct Bar;

    impl Substate<Foo> for Bar {}
}

fn main() {}
