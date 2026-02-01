use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    struct Top;
    impl TopState for Top {}

    struct Foo;

    impl Substate<Beef> for Foo {}

    struct Bar;

    impl Substate<Foo> for Bar {}

    struct Fizz;

    impl Substate<Foo> for Fizz {}

    struct Buzz;

    impl Substate<Fizz> for Buzz {}

    struct Dead;

    impl Substate<Buzz> for Dead {}

    struct Beef;

    impl Substate<Dead> for Beef {}
}

fn main() {}
