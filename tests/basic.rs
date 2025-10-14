#![allow(clippy::upper_case_acronyms)]

use basic::{
    machine::{BasicMachineBuilder, BasicState, BASIC_STATE_CHART},
    Top, A, B, BA, BB,
};
use moku::*;
use test_log::test;

#[state_machine]
mod basic {
    use std::{cell::Cell, rc::Rc};

    use moku::*;

    #[machine_module]
    pub mod machine {}

    use machine::BasicState;

    #[derive(Default)]
    pub struct Top {
        pub access: u8,
        pub init: u8,
        pub update: u8,
        pub top_down_update: u8,
        pub update_order: u8,
        pub update_order_acc: u8,
    }

    impl TopState<BasicState> for Top {
        fn init(&mut self) -> Option<BasicState> {
            self.init += 1;
            None
        }

        fn update(&mut self) -> Option<BasicState> {
            self.update += 1;
            self.update_order = self.update_order_acc;
            self.update_order_acc += 1;
            None
        }

        fn top_down_update(&mut self) -> Option<BasicState> {
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
    impl State<BasicState> for A {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, BasicState> {
            StateEntry::State(Self {
                enter: 1,
                ..Default::default()
            })
        }

        fn init(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<BasicState> {
            self.init += 1;
            None
        }

        fn update(&mut self, superstates: &mut Self::Superstates<'_>) -> Option<BasicState> {
            self.update += 1;
            self.update_order = superstates.top.update_order_acc;
            superstates.top.update_order_acc += 1;
            None
        }

        fn top_down_update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> Option<BasicState> {
            self.top_down_update += 1;
            self.update_order = superstates.top.update_order_acc;
            superstates.top.update_order_acc += 1;
            None
        }

        fn exit(self, _superstates: &mut Self::Superstates<'_>) -> Option<BasicState> {
            self.exit.set(self.exit.get() + 1);
            None
        }
    }

    struct AA;

    #[superstate(A)]
    impl State<BasicState> for AA {}

    struct AAA;

    #[superstate(AA)]
    impl State<BasicState> for AAA {}

    struct AB;

    #[superstate(A)]
    impl State<BasicState> for AB {}

    struct ABA;

    #[superstate(AB)]
    impl State<BasicState> for ABA {}

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
    impl State<BasicState> for B {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, BasicState> {
            StateEntry::State(Self {
                enter: 1,
                ..Default::default()
            })
        }

        fn init(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<BasicState> {
            self.init += 1;
            None
        }

        fn update(&mut self, superstates: &mut Self::Superstates<'_>) -> Option<BasicState> {
            self.update += 1;
            self.update_order = superstates.top.update_order_acc;
            superstates.top.update_order_acc += 1;
            None
        }

        fn top_down_update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> Option<BasicState> {
            self.top_down_update += 1;
            self.update_order = superstates.top.update_order_acc;
            superstates.top.update_order_acc += 1;
            None
        }

        fn exit(self, _superstates: &mut Self::Superstates<'_>) -> Option<BasicState> {
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
    impl State<BasicState> for BA {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, BasicState> {
            StateEntry::State(Self {
                enter: 1,
                ..Default::default()
            })
        }

        fn init(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<BasicState> {
            self.init += 1;
            None
        }

        fn update(&mut self, superstates: &mut Self::Superstates<'_>) -> Option<BasicState> {
            self.update += 1;
            self.update_order = superstates.top.update_order_acc;
            superstates.top.update_order_acc += 1;
            None
        }

        fn top_down_update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> Option<BasicState> {
            self.top_down_update += 1;
            self.update_order = superstates.top.update_order_acc;
            superstates.top.update_order_acc += 1;
            None
        }

        fn exit(self, _superstates: &mut Self::Superstates<'_>) -> Option<BasicState> {
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
    impl State<BasicState> for BB {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, BasicState> {
            StateEntry::State(Self {
                enter: 1,
                ..Default::default()
            })
        }

        fn init(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<BasicState> {
            self.init += 1;
            None
        }

        fn update(&mut self, superstates: &mut Self::Superstates<'_>) -> Option<BasicState> {
            self.update += 1;
            self.update_order = superstates.top.update_order_acc;
            superstates.top.update_order_acc += 1;
            None
        }

        fn top_down_update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> Option<BasicState> {
            self.top_down_update += 1;
            self.update_order = superstates.top.update_order_acc;
            superstates.top.update_order_acc += 1;
            None
        }

        fn exit(self, _superstates: &mut Self::Superstates<'_>) -> Option<BasicState> {
            self.exit.set(self.exit.get() + 1);
            None
        }
    }
}

#[test]
fn state_chart() {
    assert_eq!(
        BASIC_STATE_CHART,
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
    let machine = BasicMachineBuilder::new(Top::default()).build();
    assert_eq!(machine.name(), "Basic");

    let mut machine = BasicMachineBuilder::new(Top::default())
        .name("Kantan".to_owned())
        .build();
    assert_eq!(machine.name(), "Kantan");

    machine.set_name("Kikai".to_owned());
    assert_eq!(machine.name(), "Kikai");
}

#[test]
fn state_match() {
    let mut machine = BasicMachineBuilder::new(Top::default()).build();

    assert!(matches!(machine.state(), BasicState::Top));

    assert!(machine.state_matches(BasicState::Top));
    assert!(!machine.state_matches(BasicState::A));
    assert!(!machine.state_matches(BasicState::B));
    assert!(!machine.state_matches(BasicState::BA));
    assert!(!machine.state_matches(BasicState::BB));

    machine.transition(BasicState::A);
    assert!(matches!(machine.state(), BasicState::A));

    assert!(machine.state_matches(BasicState::Top));
    assert!(machine.state_matches(BasicState::A));
    assert!(!machine.state_matches(BasicState::B));
    assert!(!machine.state_matches(BasicState::BA));
    assert!(!machine.state_matches(BasicState::BB));

    machine.transition(BasicState::B);
    assert!(matches!(machine.state(), BasicState::B));

    assert!(machine.state_matches(BasicState::Top));
    assert!(!machine.state_matches(BasicState::A));
    assert!(machine.state_matches(BasicState::B));
    assert!(!machine.state_matches(BasicState::BA));
    assert!(!machine.state_matches(BasicState::BB));

    machine.transition(BasicState::BA);
    assert!(matches!(machine.state(), BasicState::BA));

    assert!(machine.state_matches(BasicState::Top));
    assert!(!machine.state_matches(BasicState::A));
    assert!(machine.state_matches(BasicState::B));
    assert!(machine.state_matches(BasicState::BA));
    assert!(!machine.state_matches(BasicState::BB));

    machine.transition(BasicState::BB);
    assert!(matches!(machine.state(), BasicState::BB));

    assert!(machine.state_matches(BasicState::Top));
    assert!(!machine.state_matches(BasicState::A));
    assert!(machine.state_matches(BasicState::B));
    assert!(!machine.state_matches(BasicState::BA));
    assert!(machine.state_matches(BasicState::BB));
}

#[test]
fn state_refs() {
    let mut machine = BasicMachineBuilder::new(Top::default()).build();

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

    machine.transition(BasicState::A);

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

    machine.transition(BasicState::A);

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

    machine.transition(BasicState::BA);

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

    machine.transition(BasicState::BB);

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
    let mut machine = BasicMachineBuilder::new(Top::default()).build();
    machine.transition(BasicState::BA);

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
    let mut machine = BasicMachineBuilder::new(Top::default()).build();
    assert!(matches!(machine.state(), BasicState::Top));

    assert_eq!(machine.top_ref().init, 1);

    machine.transition(BasicState::BA);
    assert!(matches!(machine.state(), BasicState::BA));

    assert_eq!(machine.top_ref().init, 1);

    let state: &B = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 0);

    let state: &BA = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 1);

    let ba_exit = state.exit_counter();
    assert_eq!(ba_exit.get(), 0);

    machine.transition(BasicState::BB);
    assert!(matches!(machine.state(), BasicState::BB));

    assert_eq!(ba_exit.get(), 1);

    assert_eq!(machine.top_ref().init, 1);

    let state: &B = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 0);
    assert_eq!(state.exit_counter().get(), 0);

    let state: &BB = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 1);

    machine.transition(BasicState::B);
    assert!(matches!(machine.state(), BasicState::BB));

    let state: &B = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 0);
    assert_eq!(state.exit_counter().get(), 0);

    let state: &BB = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 1);
}
