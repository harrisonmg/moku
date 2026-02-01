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
        pub exit: Rc<Cell<u8>>,
    }

    impl A {
        pub fn exit_counter(&self) -> Rc<Cell<u8>> {
            self.exit.clone()
        }
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

        fn exit(self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.exit.set(self.exit.get() + 1);
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
        pub exit: Rc<Cell<u8>>,
    }

    impl B {
        pub fn exit_counter(&self) -> Rc<Cell<u8>> {
            self.exit.clone()
        }
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

        fn exit(self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.exit.set(self.exit.get() + 1);
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

        fn exit(self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.exit.set(self.exit.get() + 1);
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

        fn exit(self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.exit.set(self.exit.get() + 1);
        }
    }
}

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

    let mut machine = Builder::new(Top::default())
        .name("Kantan".to_owned())
        .build();
    assert_eq!(machine.name(), "Kantan");

    machine.set_name("Kikai".to_owned());
    assert_eq!(machine.name(), "Kikai");
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

    let ba_exit = state.exit_counter();
    assert_eq!(ba_exit.get(), 0);

    machine.transition(State::BB);
    assert!(matches!(machine.state(), State::BB));

    assert_eq!(ba_exit.get(), 1);

    assert_eq!(machine.top_ref().init, 1);

    let state: &B = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 0);
    assert_eq!(state.exit_counter().get(), 0);

    let state: &BB = machine.state_ref().unwrap();
    assert_eq!(state.enter, 1);
    assert_eq!(state.init, 1);

    machine.transition(State::B);
    assert!(matches!(machine.state(), State::BB));

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
    let mut machine = Builder::new(Top::default()).build();
    assert!(matches!(machine.state(), State::Top));

    assert_eq!(machine.top_ref().init, 1);

    machine.transition(State::Top);
    assert_eq!(machine.top_ref().init, 1);

    machine.exact_transition(State::Top);
    assert_eq!(machine.top_ref().init, 2);

    machine.transition(State::A);
    assert!(matches!(machine.state(), State::A));
    let state: &A = machine.state_ref().unwrap();
    assert_eq!(state.init, 1);
    assert_eq!(state.enter, 1);
    assert_eq!(state.exit_counter().get(), 0);

    machine.transition(State::A);
    assert!(matches!(machine.state(), State::A));
    let state: &A = machine.state_ref().unwrap();
    assert_eq!(state.init, 1);
    assert_eq!(state.enter, 1);
    assert_eq!(state.exit_counter().get(), 0);

    let a_exit = state.exit_counter();

    machine.exact_transition(State::A);
    assert!(matches!(machine.state(), State::A));
    let state: &A = machine.state_ref().unwrap();
    assert_eq!(state.init, 1);
    assert_eq!(state.enter, 1);
    assert_eq!(a_exit.get(), 1);
}

#[test]
fn state_list() {
    let mut machine = Builder::new(Top::default()).build();

    assert!(matches!(machine.state(), State::Top));
    assert_eq!(machine.state_list(), vec![State::Top]);

    machine.transition(State::A);
    assert!(matches!(machine.state(), State::A));
    assert_eq!(machine.state_list(), vec![State::Top, State::A]);

    machine.transition(State::AA);
    assert!(matches!(machine.state(), State::AA));
    assert_eq!(machine.state_list(), vec![State::Top, State::A, State::AA]);

    machine.transition(State::BB);
    assert!(matches!(machine.state(), State::BB));
    assert_eq!(machine.state_list(), vec![State::Top, State::B, State::BB]);
}

#[state_machine(Kikai)]
mod named {
    use moku::*;

    #[machine_module]
    pub mod machine {}

    pub struct Top;
    impl TopState for Top {}
}

#[test]
fn custom_name() {
    let machine = named::machine::Builder::new(named::Top).build();
    assert_eq!(machine.name(), "Kikai");
}
