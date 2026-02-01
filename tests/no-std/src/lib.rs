// This is a copy of the /tests/tester.rs tests but with the std feature disabled and enforcing no_std.
//
// These tests will fail if run with `cargo test --all` at the workspace root, due to a limitation of
// cargo that is well described here: https://github.com/rust-lang/cargo/issues/10636
//
// Run these tests with `cargo test` in the directory of this particular crate.

#![no_std]
#![allow(clippy::upper_case_acronyms)]

extern crate moku;
use moku::*;

#[state_machine]
mod tester {
    use moku::*;

    #[machine_module]
    pub mod machine {}

    #[derive(Default)]
    pub struct Top {
        pub access: u8,
        pub init: u8,
        pub update: u8,
        pub top_down_update: u8,
        pub update_order: u8,
        pub update_order_acc: u8,
    }

    impl TopState for Top {
        fn init(&mut self) -> impl Into<Next<Self::State>> {
            self.init += 1;
        }

        fn update(&mut self) -> impl Into<Next<Self::State>> {
            self.update += 1;
            self.update_order = self.update_order_acc;
            self.update_order_acc += 1;
        }

        fn top_down_update(&mut self) -> impl Into<Next<Self::State>> {
            self.top_down_update += 1;
            self.update_order = self.update_order_acc;
            self.update_order_acc += 1;
        }
    }

    #[derive(Default)]
    pub struct A {
        pub access: u8,
        pub enter: u8,
        pub init: u8,
        pub update: u8,
        pub top_down_update: u8,
        pub update_order: u8,
    }

    impl Substate<Top> for A {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::State(Self {
                enter: 1,
                ..Default::default()
            })
        }

        fn init(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.init += 1;
        }

        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.update += 1;
            self.update_order = ctx.top.update_order_acc;
            ctx.top.update_order_acc += 1;
        }

        fn top_down_update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.top_down_update += 1;
            self.update_order = ctx.top.update_order_acc;
            ctx.top.update_order_acc += 1;
        }
    }

    struct AA;

    impl Substate<A> for AA {}

    struct AAA;

    impl Substate<AA> for AAA {}

    struct AB;

    impl Substate<A> for AB {}

    struct ABA;

    impl Substate<AB> for ABA {}

    #[derive(Default)]
    pub struct B {
        pub access: u8,
        pub enter: u8,
        pub init: u8,
        pub update: u8,
        pub top_down_update: u8,
        pub update_order: u8,
    }

    impl Substate<Top> for B {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::State(Self {
                enter: 1,
                ..Default::default()
            })
        }

        fn init(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.init += 1;
        }

        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.update += 1;
            self.update_order = ctx.top.update_order_acc;
            ctx.top.update_order_acc += 1;
        }

        fn top_down_update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.top_down_update += 1;
            self.update_order = ctx.top.update_order_acc;
            ctx.top.update_order_acc += 1;
        }
    }

    #[derive(Default)]
    pub struct BA {
        pub access: u8,
        pub enter: u8,
        pub init: u8,
        pub update: u8,
        pub top_down_update: u8,
        pub update_order: u8,
    }

    impl Substate<B> for BA {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::State(Self {
                enter: 1,
                ..Default::default()
            })
        }

        fn init(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.init += 1;
        }

        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.update += 1;
            self.update_order = ctx.top.update_order_acc;
            ctx.top.update_order_acc += 1;
        }

        fn top_down_update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.top_down_update += 1;
            self.update_order = ctx.top.update_order_acc;
            ctx.top.update_order_acc += 1;
        }
    }

    #[derive(Default)]
    pub struct BB {
        pub access: u8,
        pub enter: u8,
        pub init: u8,
        pub update: u8,
        pub top_down_update: u8,
        pub update_order: u8,
    }

    impl Substate<B> for BB {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::State(Self {
                enter: 1,
                ..Default::default()
            })
        }

        fn init(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.init += 1;
        }

        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.update += 1;
            self.update_order = ctx.top.update_order_acc;
            ctx.top.update_order_acc += 1;
        }

        fn top_down_update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.top_down_update += 1;
            self.update_order = ctx.top.update_order_acc;
            ctx.top.update_order_acc += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::moku::*;
    use super::tester::{machine::*, *};

    #[test]
    fn state_chart() {
        assert_eq!(
            STATE_CHART,
            "Top
├─ A
│  ├─ AA
│  │  └─ AAA
│  └─ AB
│     └─ ABA
└─ B
   ├─ BA
   └─ BB"
        );
    }

    #[test]
    fn machine_name() {
        let machine = Builder::new(Top::default()).build();
        assert_eq!(machine.name(), "Tester");
    }

    #[test]
    fn state_match() {
        let mut machine = Builder::new(Top::default()).build();

        assert!(matches!(machine.state(), State::Top));

        assert!(machine.state_matches(State::Top));
        assert!(!machine.state_matches(State::A));
        assert!(!machine.state_matches(State::B));
        assert!(!machine.state_matches(State::BA));
        assert!(!machine.state_matches(State::BB));

        machine.transition(State::A);
        assert!(matches!(machine.state(), State::A));

        assert!(machine.state_matches(State::Top));
        assert!(machine.state_matches(State::A));
        assert!(!machine.state_matches(State::B));
        assert!(!machine.state_matches(State::BA));
        assert!(!machine.state_matches(State::BB));

        machine.transition(State::B);
        assert!(matches!(machine.state(), State::B));

        assert!(machine.state_matches(State::Top));
        assert!(!machine.state_matches(State::A));
        assert!(machine.state_matches(State::B));
        assert!(!machine.state_matches(State::BA));
        assert!(!machine.state_matches(State::BB));

        machine.transition(State::BA);
        assert!(matches!(machine.state(), State::BA));

        assert!(machine.state_matches(State::Top));
        assert!(!machine.state_matches(State::A));
        assert!(machine.state_matches(State::B));
        assert!(machine.state_matches(State::BA));
        assert!(!machine.state_matches(State::BB));

        machine.transition(State::BB);
        assert!(matches!(machine.state(), State::BB));

        assert!(machine.state_matches(State::Top));
        assert!(!machine.state_matches(State::A));
        assert!(machine.state_matches(State::B));
        assert!(!machine.state_matches(State::BA));
        assert!(machine.state_matches(State::BB));
    }

    #[test]
    fn state_refs() {
        let mut machine = Builder::new(Top::default()).build();

        assert_eq!(machine.top_ref().init, 1);
        machine.top_mut().access += 1;
        assert_eq!(machine.top_ref().access, 1);

        let state: &mut Top = machine.state_mut().unwrap();
        state.access += 1;

        let state: &Top = machine.state_ref().unwrap();
        assert_eq!(state.access, 2);

        let state: Option<&A> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&B> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&BA> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&BB> = machine.state_ref();
        assert!(state.is_none());

        machine.transition(State::A);

        let state: Option<&Top> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&A> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&B> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&BA> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&BB> = machine.state_ref();
        assert!(state.is_none());

        machine.transition(State::A);

        let state: Option<&Top> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&A> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&B> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&BA> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&BB> = machine.state_ref();
        assert!(state.is_none());

        machine.transition(State::BA);

        let state: Option<&Top> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&A> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&B> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&BA> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&BB> = machine.state_ref();
        assert!(state.is_none());

        machine.transition(State::BB);

        let state: Option<&Top> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&A> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&B> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&BA> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&BB> = machine.state_ref();
        assert!(state.is_some());
    }

    #[test]
    fn update_order() {
        let mut machine = Builder::new(Top::default()).build();
        machine.transition(State::BA);

        let top: &Top = machine.state_ref().unwrap();
        let bar: &B = machine.state_ref().unwrap();
        let iron: &BA = machine.state_ref().unwrap();

        assert_eq!(top.update, 0);
        assert_eq!(bar.update, 0);
        assert_eq!(iron.update, 0);
        assert_eq!(top.top_down_update, 0);
        assert_eq!(bar.top_down_update, 0);
        assert_eq!(iron.top_down_update, 0);

        assert_eq!(top.update_order, 0);
        assert_eq!(bar.update_order, 0);
        assert_eq!(iron.update_order, 0);

        machine.update();

        let top: &Top = machine.state_ref().unwrap();
        let bar: &B = machine.state_ref().unwrap();
        let iron: &BA = machine.state_ref().unwrap();

        assert_eq!(top.update, 1);
        assert_eq!(bar.update, 1);
        assert_eq!(iron.update, 1);
        assert_eq!(top.top_down_update, 0);
        assert_eq!(bar.top_down_update, 0);
        assert_eq!(iron.top_down_update, 0);

        assert_eq!(top.update_order, 2);
        assert_eq!(bar.update_order, 1);
        assert_eq!(iron.update_order, 0);

        machine.top_mut().update_order_acc = 0;
        machine.top_down_update();

        let top: &Top = machine.state_ref().unwrap();
        let bar: &B = machine.state_ref().unwrap();
        let iron: &BA = machine.state_ref().unwrap();

        assert_eq!(top.update, 1);
        assert_eq!(bar.update, 1);
        assert_eq!(iron.update, 1);
        assert_eq!(top.top_down_update, 1);
        assert_eq!(bar.top_down_update, 1);
        assert_eq!(iron.top_down_update, 1);

        assert_eq!(top.update_order, 0);
        assert_eq!(bar.update_order, 1);
        assert_eq!(iron.update_order, 2);
    }

    #[test]
    fn enter_init_exit() {
        let mut machine = Builder::new(Top::default()).build();
        assert!(matches!(machine.state(), State::Top));

        assert_eq!(machine.top_ref().init, 1);

        machine.transition(State::BA);
        assert!(matches!(machine.state(), State::BA));

        assert_eq!(machine.top_ref().init, 1);

        let state: &B = machine.state_ref().unwrap();
        assert_eq!(state.enter, 1);
        assert_eq!(state.init, 0);

        let state: &BA = machine.state_ref().unwrap();
        assert_eq!(state.enter, 1);
        assert_eq!(state.init, 1);

        machine.transition(State::BB);
        assert!(matches!(machine.state(), State::BB));

        assert_eq!(machine.top_ref().init, 1);

        let state: &B = machine.state_ref().unwrap();
        assert_eq!(state.enter, 1);
        assert_eq!(state.init, 0);

        let state: &BB = machine.state_ref().unwrap();
        assert_eq!(state.enter, 1);
        assert_eq!(state.init, 1);

        machine.transition(State::B);
        assert!(matches!(machine.state(), State::BB));

        let state: &B = machine.state_ref().unwrap();
        assert_eq!(state.enter, 1);
        assert_eq!(state.init, 0);

        let state: &BB = machine.state_ref().unwrap();
        assert_eq!(state.enter, 1);
        assert_eq!(state.init, 1);
    }
}

#[state_machine(Kikai)]
mod named {
    use moku::*;

    #[machine_module]
    pub mod machine {}

    pub struct Top;
    impl TopState for Top {}
}

#[cfg(test)]
mod named_tests {
    use super::named::{machine::Builder, *};
    use moku::*;

    #[test]
    fn custom_name() {
        let machine = Builder::new(Top).build();
        assert_eq!(machine.name(), "Kikai");
    }
}
