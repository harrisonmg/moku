use core::panic;

use basic::{
    state_machine::{BasicMachineBuilder, BasicState, BASIC_STATE_CHART},
    Bar, Foo, Iron, Top, Wet,
};
use moku::*;

#[state_machine]
mod basic {
    use std::{cell::Cell, rc::Rc};

    use moku::*;

    #[machine_module]
    pub mod state_machine {}

    use state_machine::BasicState;

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
    pub struct Foo {
        pub access: u8,
        pub enter: u8,
        pub init: u8,
        pub update: u8,
        pub top_down_update: u8,
        pub update_order: u8,
        pub exit: Rc<Cell<u8>>,
    }

    impl Foo {
        pub fn exit_counter(&self) -> Rc<Cell<u8>> {
            self.exit.clone()
        }
    }

    #[superstate(Top)]
    impl State<BasicState> for Foo {
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
    pub struct Bar {
        pub access: u8,
        pub enter: u8,
        pub init: u8,
        pub update: u8,
        pub top_down_update: u8,
        pub update_order: u8,
        pub exit: Rc<Cell<u8>>,
    }

    impl Bar {
        pub fn exit_counter(&self) -> Rc<Cell<u8>> {
            self.exit.clone()
        }
    }

    #[superstate(Top)]
    impl State<BasicState> for Bar {
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
    pub struct Iron {
        pub access: u8,
        pub enter: u8,
        pub init: u8,
        pub update: u8,
        pub top_down_update: u8,
        pub update_order: u8,
        pub exit: Rc<Cell<u8>>,
    }

    impl Iron {
        pub fn exit_counter(&self) -> Rc<Cell<u8>> {
            self.exit.clone()
        }
    }

    #[superstate(Bar)]
    impl State<BasicState> for Iron {
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
    pub struct Wet {
        pub access: u8,
        pub enter: u8,
        pub init: u8,
        pub update: u8,
        pub top_down_update: u8,
        pub update_order: u8,
        pub exit: Rc<Cell<u8>>,
    }

    impl Wet {
        pub fn exit_counter(&self) -> Rc<Cell<u8>> {
            self.exit.clone()
        }
    }

    #[superstate(Bar)]
    impl State<BasicState> for Wet {
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
├─ Foo
└─ Bar
   ├─ Iron
   └─ Wet"
    );
}

#[test]
fn machine_name() {
    let machine = BasicMachineBuilder::new(Top::default()).build();
    assert_eq!(machine.name(), "Basic");

    let mut machine = BasicMachineBuilder::new(Top::default())
        .name("Kantan")
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
    assert!(!machine.state_matches(BasicState::Foo));
    assert!(!machine.state_matches(BasicState::Bar));
    assert!(!machine.state_matches(BasicState::Iron));
    assert!(!machine.state_matches(BasicState::Wet));

    machine.transition(BasicState::Foo);
    assert!(matches!(machine.state(), BasicState::Foo));

    assert!(machine.state_matches(BasicState::Top));
    assert!(machine.state_matches(BasicState::Foo));
    assert!(!machine.state_matches(BasicState::Bar));
    assert!(!machine.state_matches(BasicState::Iron));
    assert!(!machine.state_matches(BasicState::Wet));

    machine.transition(BasicState::Bar);
    assert!(matches!(machine.state(), BasicState::Bar));

    assert!(machine.state_matches(BasicState::Top));
    assert!(!machine.state_matches(BasicState::Foo));
    assert!(machine.state_matches(BasicState::Bar));
    assert!(!machine.state_matches(BasicState::Iron));
    assert!(!machine.state_matches(BasicState::Wet));

    machine.transition(BasicState::Iron);
    assert!(matches!(machine.state(), BasicState::Iron));

    assert!(machine.state_matches(BasicState::Top));
    assert!(!machine.state_matches(BasicState::Foo));
    assert!(machine.state_matches(BasicState::Bar));
    assert!(machine.state_matches(BasicState::Iron));
    assert!(!machine.state_matches(BasicState::Wet));

    machine.transition(BasicState::Wet);
    assert!(matches!(machine.state(), BasicState::Wet));

    assert!(machine.state_matches(BasicState::Top));
    assert!(!machine.state_matches(BasicState::Foo));
    assert!(machine.state_matches(BasicState::Bar));
    assert!(!machine.state_matches(BasicState::Iron));
    assert!(machine.state_matches(BasicState::Wet));
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

    let state: Option<&Foo> = machine.state_ref();
    assert!(state.is_none());
    let state: Option<&Bar> = machine.state_ref();
    assert!(state.is_none());
    let state: Option<&Iron> = machine.state_ref();
    assert!(state.is_none());
    let state: Option<&Wet> = machine.state_ref();
    assert!(state.is_none());

    machine.transition(BasicState::Foo);

    let state: Option<&Top> = machine.state_ref();
    assert!(state.is_some());
    let state: Option<&Foo> = machine.state_ref();
    assert!(state.is_some());
    let state: Option<&Bar> = machine.state_ref();
    assert!(state.is_none());
    let state: Option<&Iron> = machine.state_ref();
    assert!(state.is_none());
    let state: Option<&Wet> = machine.state_ref();
    assert!(state.is_none());

    machine.transition(BasicState::Foo);

    let state: Option<&Top> = machine.state_ref();
    assert!(state.is_some());
    let state: Option<&Foo> = machine.state_ref();
    assert!(state.is_some());
    let state: Option<&Bar> = machine.state_ref();
    assert!(state.is_none());
    let state: Option<&Iron> = machine.state_ref();
    assert!(state.is_none());
    let state: Option<&Wet> = machine.state_ref();
    assert!(state.is_none());

    machine.transition(BasicState::Iron);

    let state: Option<&Top> = machine.state_ref();
    assert!(state.is_some());
    let state: Option<&Foo> = machine.state_ref();
    assert!(state.is_none());
    let state: Option<&Bar> = machine.state_ref();
    assert!(state.is_some());
    let state: Option<&Iron> = machine.state_ref();
    assert!(state.is_some());
    let state: Option<&Wet> = machine.state_ref();
    assert!(state.is_none());

    machine.transition(BasicState::Wet);

    let state: Option<&Top> = machine.state_ref();
    assert!(state.is_some());
    let state: Option<&Foo> = machine.state_ref();
    assert!(state.is_none());
    let state: Option<&Bar> = machine.state_ref();
    assert!(state.is_some());
    let state: Option<&Iron> = machine.state_ref();
    assert!(state.is_none());
    let state: Option<&Wet> = machine.state_ref();
    assert!(state.is_some());

    machine.transition(BasicState::Bar);

    let state: Option<&Top> = machine.state_ref();
    assert!(state.is_some());
    let state: Option<&Foo> = machine.state_ref();
    assert!(state.is_none());
    let state: Option<&Bar> = machine.state_ref();
    assert!(state.is_some());
    let state: Option<&Iron> = machine.state_ref();
    assert!(state.is_none());
    let state: Option<&Wet> = machine.state_ref();
    assert!(state.is_none());
}

#[test]
fn update_order() {
    let mut machine = BasicMachineBuilder::new(Top::default()).build();
    machine.transition(BasicState::Iron);

    let top: &Top = machine.state_ref().unwrap();
    let bar: &Bar = machine.state_ref().unwrap();
    let iron: &Iron = machine.state_ref().unwrap();

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
    let bar: &Bar = machine.state_ref().unwrap();
    let iron: &Iron = machine.state_ref().unwrap();

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
    let bar: &Bar = machine.state_ref().unwrap();
    let iron: &Iron = machine.state_ref().unwrap();

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

    machine.transition(BasicState::Iron);
    assert!(matches!(machine.state(), BasicState::Iron));

    assert_eq!(machine.top_ref().init, 1);

    let state: &Bar = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 0);

    let state: &Iron = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 1);

    machine.transition(BasicState::Wet);
    assert!(matches!(machine.state(), BasicState::Wet));

    assert_eq!(machine.top_ref().init, 1);

    let state: &Bar = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 0);

    let state: &Wet = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 1);

    let wet_exit = state.exit_counter();
    assert_eq!(wet_exit.get(), 0);

    machine.transition(BasicState::Bar);
    assert!(matches!(machine.state(), BasicState::Bar));

    assert_eq!(wet_exit.get(), 1);

    let state: &Bar = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 1);

    let bar_exit = state.exit_counter();
    assert_eq!(bar_exit.get(), 0);

    machine.transition(BasicState::Bar);
    assert!(matches!(machine.state(), BasicState::Bar));

    assert_eq!(bar_exit.get(), 0);

    let state: &Bar = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 2);

    machine.transition(BasicState::Top);
    assert!(matches!(machine.state(), BasicState::Top));

    assert_eq!(machine.top_ref().init, 2);

    machine.transition(BasicState::Iron);
    assert!(matches!(machine.state(), BasicState::Iron));

    assert_eq!(machine.top_ref().init, 2);

    let state: &Iron = machine.state_ref().unwrap();
    let iron_exit = state.exit_counter();
    assert_eq!(iron_exit.get(), 0);

    let state: &Bar = machine.state_ref().unwrap();
    let bar_exit = state.exit_counter();
    assert_eq!(bar_exit.get(), 0);

    machine.transition(BasicState::Foo);
    assert!(matches!(machine.state(), BasicState::Foo));

    assert_eq!(iron_exit.get(), 1);
    assert_eq!(bar_exit.get(), 1);
}
