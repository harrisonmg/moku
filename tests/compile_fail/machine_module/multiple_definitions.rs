use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod state_machine_module {}

    #[machine_module]
    mod other_state_machine_module {}
}

fn main() {}
