use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module = something]
    mod machine {}
}

fn main() {}
