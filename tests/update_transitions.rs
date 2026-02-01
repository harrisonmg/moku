#![allow(clippy::upper_case_acronyms)]

use moku::*;
use test_log::test;
use tester::{machine::*, *};

#[state_machine]
mod tester {
    use moku::*;

    #[machine_module]
    pub mod machine {}

    use machine::State;

    pub struct Top {
        pub update: u8,
        pub top_down_update: u8,
    }

    impl TopState for Top {
        fn update(&mut self) -> impl Into<Next<Self::State>> {
            self.update += 1;
        }

        fn top_down_update(&mut self) -> impl Into<Next<Self::State>> {
            self.top_down_update += 1;
        }
    }

    pub struct A {
        pub update: u8,
        pub top_down_update: u8,
    }

    impl Substate<Top> for A {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.update += 1;
            State::BA
        }

        fn top_down_update(
            &mut self,
            _ctx: &mut Self::Context<'_>,
        ) -> impl Into<Next<Self::State>> {
            self.top_down_update += 1;
            State::BA
        }
    }

    pub struct AA {
        pub update: u8,
        pub top_down_update: u8,
    }

    impl Substate<A> for AA {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.update += 1;
        }

        fn top_down_update(
            &mut self,
            _ctx: &mut Self::Context<'_>,
        ) -> impl Into<Next<Self::State>> {
            self.top_down_update += 1;
        }
    }

    pub struct B {
        pub update: u8,
        pub top_down_update: u8,
    }

    impl Substate<Top> for B {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.update += 1;
            State::BA
        }

        fn top_down_update(
            &mut self,
            _ctx: &mut Self::Context<'_>,
        ) -> impl Into<Next<Self::State>> {
            self.top_down_update += 1;
            State::BA
        }
    }

    pub struct BA {
        pub update: u8,
        pub top_down_update: u8,
    }

    impl Substate<B> for BA {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.update += 1;
        }

        fn top_down_update(
            &mut self,
            _ctx: &mut Self::Context<'_>,
        ) -> impl Into<Next<Self::State>> {
            self.top_down_update += 1;
        }
    }

    pub struct BB {
        pub update: u8,
        pub top_down_update: u8,
    }

    impl Substate<B> for BB {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.update += 1;
        }

        fn top_down_update(
            &mut self,
            _ctx: &mut Self::Context<'_>,
        ) -> impl Into<Next<Self::State>> {
            self.top_down_update += 1;
        }
    }

    pub struct BBA {
        pub update: u8,
        pub top_down_update: u8,
    }

    impl Substate<BB> for BBA {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.update += 1;
            State::BA
        }

        fn top_down_update(
            &mut self,
            _ctx: &mut Self::Context<'_>,
        ) -> impl Into<Next<Self::State>> {
            self.top_down_update += 1;
            State::BA
        }
    }
}

#[test]
fn state_chart() {
    assert_eq!(
        STATE_CHART,
        "Top
├─ A
│  └─ AA
└─ B
   ├─ BA
   └─ BB
      └─ BBA"
    );
}

#[test]
fn update() {
    let mut machine = Builder::new(Top {
        update: 0,
        top_down_update: 0,
    })
    .build();

    machine.transition(State::AA);
    assert!(matches!(machine.state(), State::AA));

    assert_eq!(machine.top_ref().update, 0);

    let a: &A = machine.state_ref().unwrap();
    assert_eq!(a.update, 0);

    let aa: &AA = machine.state_ref().unwrap();
    assert_eq!(aa.update, 0);

    machine.update();
    assert!(matches!(machine.state(), State::BA));

    assert_eq!(machine.top_ref().update, 1);

    let b: &B = machine.state_ref().unwrap();
    assert_eq!(b.update, 0);

    let ba: &BA = machine.state_ref().unwrap();
    assert_eq!(ba.update, 0);

    machine.update();
    assert!(matches!(machine.state(), State::BA));

    assert_eq!(machine.top_ref().update, 2);

    let b: &B = machine.state_ref().unwrap();
    assert_eq!(b.update, 1);

    let ba: &BA = machine.state_ref().unwrap();
    assert_eq!(ba.update, 1);

    machine.transition(State::BBA);
    assert!(matches!(machine.state(), State::BBA));

    machine.update();
    assert!(matches!(machine.state(), State::BA));

    assert_eq!(machine.top_ref().update, 3);

    let b: &B = machine.state_ref().unwrap();
    assert_eq!(b.update, 2);

    let ba: &BA = machine.state_ref().unwrap();
    assert_eq!(ba.update, 0);
}

#[test]
fn top_down_update() {
    let mut machine = Builder::new(Top {
        update: 0,
        top_down_update: 0,
    })
    .build();

    machine.transition(State::AA);
    assert!(matches!(machine.state(), State::AA));

    assert_eq!(machine.top_ref().top_down_update, 0);

    let a: &A = machine.state_ref().unwrap();
    assert_eq!(a.top_down_update, 0);

    let aa: &AA = machine.state_ref().unwrap();
    assert_eq!(aa.top_down_update, 0);

    machine.top_down_update();
    assert!(matches!(machine.state(), State::BA));

    assert_eq!(machine.top_ref().top_down_update, 1);

    let b: &B = machine.state_ref().unwrap();
    assert_eq!(b.top_down_update, 1);

    let ba: &BA = machine.state_ref().unwrap();
    assert_eq!(ba.top_down_update, 1);

    machine.top_down_update();
    assert!(matches!(machine.state(), State::BA));

    assert_eq!(machine.top_ref().top_down_update, 2);

    let b: &B = machine.state_ref().unwrap();
    assert_eq!(b.top_down_update, 2);

    let ba: &BA = machine.state_ref().unwrap();
    assert_eq!(ba.top_down_update, 2);
}
