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
        fn init(&mut self) -> Self::Next {
            State::A.into()
        }
    }

    struct A;
    impl moku::Substate<Top> for A {
        fn update(&mut self, ctx: &mut Self::Context<'_>) -> Self::Next {
            println!("{}", ctx.top.val);
            moku::Next::None
        }
    }
}

fn main() {
    let _machine = new::Builder::new(new::Top { val: 42 }).build();
}
