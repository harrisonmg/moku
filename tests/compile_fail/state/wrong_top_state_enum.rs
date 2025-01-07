use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod state_machine {}

    #[derive(Debug, Clone, Copy)]
    enum BlonkyState {}
    impl StateEnum for BlonkyState {}

    struct Top {}
    impl TopState<BlonkyState> for Top {}
}

fn main() {}
