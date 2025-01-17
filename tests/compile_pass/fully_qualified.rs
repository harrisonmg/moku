#[::moku::state_machine]
mod blinky {
    #[::moku::machine_module]
    mod state_machine {}

    use state_machine::BlinkyState;

    struct Top;

    impl ::moku::TopState<BlinkyState> for Top {}

    struct Disabled;

    #[::moku::superstate(Top)]
    impl ::moku::State<BlinkyState> for Disabled {}

    struct Enabled;

    #[::moku::superstate(Top)]
    impl moku::State<BlinkyState> for Enabled {}
}

fn main() {}
