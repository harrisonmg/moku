use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    struct Top {}
    impl TopState for &Top {}
}

fn main() {}
