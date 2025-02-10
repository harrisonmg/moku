use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {
        const SOMETHING: &str = "something";
    }
}

fn main() {}
