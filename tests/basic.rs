#![allow(clippy::upper_case_acronyms)]

use moku::*;
use test_log::test;
use tester::{machine::*, *};

#[state_machine]
mod tester {
    use std::{cell::Cell, rc::Rc};

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
        pub exit: Rc<Cell<u8>>,
    }

    impl A {
        pub fn exit_counter(&self) -> Rc<Cell<u8>> {
            self.exit.clone()
        }
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

        fn exit(self, _superstates: &mut Self::Superstates<'_>) -> impl Into<Next<TesterState>> {
            self.exit.set(self.exit.get() + 1);
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
        pub exit: Rc<Cell<u8>>,
    }

    impl B {
        pub fn exit_counter(&self) -> Rc<Cell<u8>> {
            self.exit.clone()
        }
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

        fn exit(self, _superstates: &mut Self::Superstates<'_>) -> impl Into<Next<TesterState>> {
            self.exit.set(self.exit.get() + 1);
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
        pub exit: Rc<Cell<u8>>,
    }

    impl BA {
        pub fn exit_counter(&self) -> Rc<Cell<u8>> {
            self.exit.clone()
        }
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

        fn exit(self, _superstates: &mut Self::Superstates<'_>) -> impl Into<Next<TesterState>> {
            self.exit.set(self.exit.get() + 1);
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
        pub exit: Rc<Cell<u8>>,
    }

    impl BB {
        pub fn exit_counter(&self) -> Rc<Cell<u8>> {
            self.exit.clone()
        }
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

        fn exit(self, _superstates: &mut Self::Superstates<'_>) -> impl Into<Next<TesterState>> {
            self.exit.set(self.exit.get() + 1);
            None
        }
    }
}

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

    let mut machine = TesterMachineBuilder::new(Top::default())
        .name("Kantan".to_owned())
        .build();
    assert_eq!(machine.name(), "Kantan");

    machine.set_name("Kikai".to_owned());
    assert_eq!(machine.name(), "Kikai");
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

    let ba_exit = state.exit_counter();
    assert_eq!(ba_exit.get(), 0);

    machine.transition(TesterState::BB);
    assert!(matches!(machine.state(), TesterState::BB));

    assert_eq!(ba_exit.get(), 1);

    assert_eq!(machine.top_ref().init, 1);

    let state: &B = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 0);
    assert_eq!(state.exit_counter().get(), 0);

    let state: &BB = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 1);

    machine.transition(TesterState::B);
    assert!(matches!(machine.state(), TesterState::BB));

    let state: &B = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 0);
    assert_eq!(state.exit_counter().get(), 0);

    let state: &BB = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 1);
}

#[test]
fn self_transition() {
    let mut machine = TesterMachineBuilder::new(Top::default()).build();
    assert!(matches!(machine.state(), TesterState::Top));

    assert_eq!(machine.top_ref().init, 1);

    machine.transition(TesterState::Top);
    assert_eq!(machine.top_ref().init, 1);

    machine.exact_transition(TesterState::Top);
    assert_eq!(machine.top_ref().init, 2);

    machine.transition(TesterState::A);
    assert!(matches!(machine.state(), TesterState::A));
    let state: &A = machine.state_ref().unwrap();
    assert_eq!(state.init, 1);
    assert_eq!(state.enter, 1);
    assert_eq!(state.exit_counter().get(), 0);

    machine.transition(TesterState::A);
    assert!(matches!(machine.state(), TesterState::A));
    let state: &A = machine.state_ref().unwrap();
    assert_eq!(state.init, 1);
    assert_eq!(state.enter, 1);
    assert_eq!(state.exit_counter().get(), 0);

    let a_exit = state.exit_counter();

    machine.exact_transition(TesterState::A);
    assert!(matches!(machine.state(), TesterState::A));
    let state: &A = machine.state_ref().unwrap();
    assert_eq!(state.init, 1);
    assert_eq!(state.enter, 1);
    assert_eq!(a_exit.get(), 1);
}

#[test]
fn state_list() {
    let mut machine = TesterMachineBuilder::new(Top::default()).build();

    assert!(matches!(machine.state(), TesterState::Top));
    assert_eq!(machine.state_list(), vec![TesterState::Top]);

    machine.transition(TesterState::A);
    assert!(matches!(machine.state(), TesterState::A));
    assert_eq!(machine.state_list(), vec![TesterState::Top, TesterState::A]);

    machine.transition(TesterState::AA);
    assert!(matches!(machine.state(), TesterState::AA));
    assert_eq!(
        machine.state_list(),
        vec![TesterState::Top, TesterState::A, TesterState::AA]
    );

    machine.transition(TesterState::BB);
    assert!(matches!(machine.state(), TesterState::BB));
    assert_eq!(
        machine.state_list(),
        vec![TesterState::Top, TesterState::B, TesterState::BB]
    );
}
