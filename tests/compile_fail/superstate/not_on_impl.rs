mod blinky {
    use moku::*;

    struct Top {}

    #[superstate(Top)]
    struct Bottom {}
}

fn main() {}
