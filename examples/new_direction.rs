use moku::StateMachineBuilder;

#[moku::state_machine]
mod new {
    #[moku::machine_module]
    mod machine {}
    pub use machine::*;

    pub struct Top {
        pub val: u8,
    }

    impl moku::TopState for Top {
        fn init(&mut self) -> impl Into<moku::Next<Self::State>> {
            Some(State::A)
        }
    }

    struct A;
    impl moku::Substate<Top> for A {
        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<moku::Next<Self::State>> {
            println!("{}", ctx.top.val);
            None
        }
    }
}

fn main() {
    let _machine = new::Builder::new(new::Top { val: 42 }).build();
}
