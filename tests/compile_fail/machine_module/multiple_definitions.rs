use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    #[machine_module]
    mod other_machine {}
}

fn main() {}
