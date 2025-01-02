extern crate moku_macros as moku;

use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    struct Module {}
}

fn main() {}
