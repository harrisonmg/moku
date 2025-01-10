mod blinky {
    use moku::*;

    struct Top {}

    struct Bottom {}

    #[superstate(Top)]
    impl Default for Bottom {
        fn default() -> Self {
            Self {}
        }
    }
}

fn main() {}
