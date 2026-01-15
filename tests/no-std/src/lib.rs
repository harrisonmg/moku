// This is a copy of the /tests/tester.rs tests but with the std feature disabled and enforcing no_std.
//
// These tests will fail if run with `cargo test --all` at the workspace root, due to a limitation of
// cargo that is well described here: https://github.com/rust-lang/cargo/issues/10636
//
// Run these tests with `cargo test` in the directory of this particular crate.

#![no_std]

extern crate moku;
use moku::*;

#[state_machine]
mod tester {
    use moku::*;

    #[machine_module]
    pub mod machine {}

    use machine::TesterState;

    #[derive(Default)]
    pub struct Top {
        pub access: u8,
        pub init: u8,
        pub update: u8,
        pub top_down_update: u8,
        pub update_order: u8,
        pub update_order_acc: u8,
    }

    impl TopState<TesterState> for Top {
        fn init(&mut self) -> impl Into<Next<TesterState>> {
            self.init += 1;
            None
        }

        fn update(&mut self) -> impl Into<Next<TesterState>> {
            self.update += 1;
            self.update_order = self.update_order_acc;
            self.update_order_acc += 1;
            None
        }

        fn top_down_update(&mut self) -> impl Into<Next<TesterState>> {
            self.top_down_update += 1;
            self.update_order = self.update_order_acc;
            self.update_order_acc += 1;
            None
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

    #[superstate(Top)]
    impl State<TesterState> for A {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<TesterState, Self> {
            StateEntry::State(Self {
                enter: 1,
                ..Default::default()
            })
        }

        fn init(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.init += 1;
            None
        }

        fn update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.update += 1;
            self.update_order = superstates.top.update_order_acc;
            superstates.top.update_order_acc += 1;
            None
        }

        fn top_down_update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.top_down_update += 1;
            self.update_order = superstates.top.update_order_acc;
            superstates.top.update_order_acc += 1;
            None
        }
    }

    struct AA;

    #[superstate(A)]
    impl State<TesterState> for AA {}

    struct AAA;

    #[superstate(AA)]
    impl State<TesterState> for AAA {}

    struct AB;

    #[superstate(A)]
    impl State<TesterState> for AB {}

    struct ABA;

    #[superstate(AB)]
    impl State<TesterState> for ABA {}

    #[derive(Default)]
    pub struct B {
        pub access: u8,
        pub enter: u8,
        pub init: u8,
        pub update: u8,
        pub top_down_update: u8,
        pub update_order: u8,
    }

    #[superstate(Top)]
    impl State<TesterState> for B {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<TesterState, Self> {
            StateEntry::State(Self {
                enter: 1,
                ..Default::default()
            })
        }

        fn init(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.init += 1;
            None
        }

        fn update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.update += 1;
            self.update_order = superstates.top.update_order_acc;
            superstates.top.update_order_acc += 1;
            None
        }

        fn top_down_update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.top_down_update += 1;
            self.update_order = superstates.top.update_order_acc;
            superstates.top.update_order_acc += 1;
            None
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

    #[superstate(B)]
    impl State<TesterState> for BA {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<TesterState, Self> {
            StateEntry::State(Self {
                enter: 1,
                ..Default::default()
            })
        }

        fn init(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.init += 1;
            None
        }

        fn update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.update += 1;
            self.update_order = superstates.top.update_order_acc;
            superstates.top.update_order_acc += 1;
            None
        }

        fn top_down_update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.top_down_update += 1;
            self.update_order = superstates.top.update_order_acc;
            superstates.top.update_order_acc += 1;
            None
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

    #[superstate(B)]
    impl State<TesterState> for BB {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<TesterState, Self> {
            StateEntry::State(Self {
                enter: 1,
                ..Default::default()
            })
        }

        fn init(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.init += 1;
            None
        }

        fn update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.update += 1;
            self.update_order = superstates.top.update_order_acc;
            superstates.top.update_order_acc += 1;
            None
        }

        fn top_down_update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.top_down_update += 1;
            self.update_order = superstates.top.update_order_acc;
            superstates.top.update_order_acc += 1;
            None
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
            TESTER_STATE_CHART,
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
        let machine = TesterMachineBuilder::new(Top::default()).build();
        assert_eq!(machine.name(), "Tester");
    }

    #[test]
    fn state_match() {
        let mut machine = TesterMachineBuilder::new(Top::default()).build();

        assert!(matches!(machine.state(), TesterState::Top));

        assert!(machine.state_matches(TesterState::Top));
        assert!(!machine.state_matches(TesterState::A));
        assert!(!machine.state_matches(TesterState::B));
        assert!(!machine.state_matches(TesterState::BA));
        assert!(!machine.state_matches(TesterState::BB));

        machine.transition(TesterState::A);
        assert!(matches!(machine.state(), TesterState::A));

        assert!(machine.state_matches(TesterState::Top));
        assert!(machine.state_matches(TesterState::A));
        assert!(!machine.state_matches(TesterState::B));
        assert!(!machine.state_matches(TesterState::BA));
        assert!(!machine.state_matches(TesterState::BB));

        machine.transition(TesterState::B);
        assert!(matches!(machine.state(), TesterState::B));

        assert!(machine.state_matches(TesterState::Top));
        assert!(!machine.state_matches(TesterState::A));
        assert!(machine.state_matches(TesterState::B));
        assert!(!machine.state_matches(TesterState::BA));
        assert!(!machine.state_matches(TesterState::BB));

        machine.transition(TesterState::BA);
        assert!(matches!(machine.state(), TesterState::BA));

        assert!(machine.state_matches(TesterState::Top));
        assert!(!machine.state_matches(TesterState::A));
        assert!(machine.state_matches(TesterState::B));
        assert!(machine.state_matches(TesterState::BA));
        assert!(!machine.state_matches(TesterState::BB));

        machine.transition(TesterState::BB);
        assert!(matches!(machine.state(), TesterState::BB));

        assert!(machine.state_matches(TesterState::Top));
        assert!(!machine.state_matches(TesterState::A));
        assert!(machine.state_matches(TesterState::B));
        assert!(!machine.state_matches(TesterState::BA));
        assert!(machine.state_matches(TesterState::BB));
    }

    #[test]
    fn state_refs() {
        let mut machine = TesterMachineBuilder::new(Top::default()).build();

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

        machine.transition(TesterState::A);

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

        machine.transition(TesterState::A);

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

        machine.transition(TesterState::BA);

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

        machine.transition(TesterState::BB);

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
        let mut machine = TesterMachineBuilder::new(Top::default()).build();
        machine.transition(TesterState::BA);

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
        let mut machine = TesterMachineBuilder::new(Top::default()).build();
        assert!(matches!(machine.state(), TesterState::Top));

        assert_eq!(machine.top_ref().init, 1);

        machine.transition(TesterState::BA);
        assert!(matches!(machine.state(), TesterState::BA));

        assert_eq!(machine.top_ref().init, 1);

        let state: &B = machine.state_ref().unwrap();
        assert_eq!(state.enter, 1);
        assert_eq!(state.init, 0);

        let state: &BA = machine.state_ref().unwrap();
        assert_eq!(state.enter, 1);
        assert_eq!(state.init, 1);

        machine.transition(TesterState::BB);
        assert!(matches!(machine.state(), TesterState::BB));

        assert_eq!(machine.top_ref().init, 1);

        let state: &B = machine.state_ref().unwrap();
        assert_eq!(state.enter, 1);
        assert_eq!(state.init, 0);

        let state: &BB = machine.state_ref().unwrap();
        assert_eq!(state.enter, 1);
        assert_eq!(state.init, 1);

        machine.transition(TesterState::B);
        assert!(matches!(machine.state(), TesterState::BB));

        let state: &B = machine.state_ref().unwrap();
        assert_eq!(state.enter, 1);
        assert_eq!(state.init, 0);

        let state: &BB = machine.state_ref().unwrap();
        assert_eq!(state.enter, 1);
        assert_eq!(state.init, 1);
    }
}
