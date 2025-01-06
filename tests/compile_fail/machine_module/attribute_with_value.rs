use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module = something]
    mod state_machine_module {}
}

fn main() {}
