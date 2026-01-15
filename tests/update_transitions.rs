#![allow(clippy::upper_case_acronyms)]

use moku::*;
use test_log::test;
use tester::{machine::*, *};

#[state_machine]
mod tester {
    use moku::*;

    #[machine_module]
    pub mod machine {}

    use machine::TesterState;

    pub struct Top {
        pub update: u8,
        pub top_down_update: u8,
    }

    impl TopState<TesterState> for Top {
        fn update(&mut self) -> impl Into<Next<TesterState>> {
            self.update += 1;
            None
        }

        fn top_down_update(&mut self) -> impl Into<Next<TesterState>> {
            self.top_down_update += 1;
            None
        }
    }

    pub struct A {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(Top)]
    impl State<TesterState> for A {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<TesterState, Self> {
            StateEntry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.update += 1;
            Some(TesterState::BA)
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.top_down_update += 1;
            Some(TesterState::BA)
        }
    }

    pub struct AA {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(A)]
    impl State<TesterState> for AA {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<TesterState, Self> {
            StateEntry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.update += 1;
            None
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.top_down_update += 1;
            None
        }
    }

    pub struct B {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(Top)]
    impl State<TesterState> for B {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<TesterState, Self> {
            StateEntry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.update += 1;
            Some(TesterState::BA)
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.top_down_update += 1;
            Some(TesterState::BA)
        }
    }

    pub struct BA {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(B)]
    impl State<TesterState> for BA {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<TesterState, Self> {
            StateEntry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.update += 1;
            None
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.top_down_update += 1;
            None
        }
    }

    pub struct BB {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(B)]
    impl State<TesterState> for BB {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<TesterState, Self> {
            StateEntry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.update += 1;
            None
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.top_down_update += 1;
            None
        }
    }

    pub struct BBA {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(BB)]
    impl State<TesterState> for BBA {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<TesterState, Self> {
            StateEntry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.update += 1;
            Some(TesterState::BA)
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            self.top_down_update += 1;
            Some(TesterState::BA)
        }
    }
}

#[test]
fn state_chart() {
    assert_eq!(
        TESTER_STATE_CHART,
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
    let mut machine = TesterMachineBuilder::new(Top {
        update: 0,
        top_down_update: 0,
    })
    .build();

    machine.transition(TesterState::AA);
    assert!(matches!(machine.state(), TesterState::AA));

    assert_eq!(machine.top_ref().update, 0);

    let a: &A = machine.state_ref().unwrap();
    assert_eq!(a.update, 0);

    let aa: &AA = machine.state_ref().unwrap();
    assert_eq!(aa.update, 0);

    machine.update();
    assert!(matches!(machine.state(), TesterState::BA));

    assert_eq!(machine.top_ref().update, 1);

    let b: &B = machine.state_ref().unwrap();
    assert_eq!(b.update, 0);

    let ba: &BA = machine.state_ref().unwrap();
    assert_eq!(ba.update, 0);

    machine.update();
    assert!(matches!(machine.state(), TesterState::BA));

    assert_eq!(machine.top_ref().update, 2);

    let b: &B = machine.state_ref().unwrap();
    assert_eq!(b.update, 1);

    let ba: &BA = machine.state_ref().unwrap();
    assert_eq!(ba.update, 1);

    machine.transition(TesterState::BBA);
    assert!(matches!(machine.state(), TesterState::BBA));

    machine.update();
    assert!(matches!(machine.state(), TesterState::BA));

    assert_eq!(machine.top_ref().update, 3);

    let b: &B = machine.state_ref().unwrap();
    assert_eq!(b.update, 2);

    let ba: &BA = machine.state_ref().unwrap();
    assert_eq!(ba.update, 0);
}

#[test]
fn top_down_update() {
    let mut machine = TesterMachineBuilder::new(Top {
        update: 0,
        top_down_update: 0,
    })
    .build();

    machine.transition(TesterState::AA);
    assert!(matches!(machine.state(), TesterState::AA));

    assert_eq!(machine.top_ref().top_down_update, 0);

    let a: &A = machine.state_ref().unwrap();
    assert_eq!(a.top_down_update, 0);

    let aa: &AA = machine.state_ref().unwrap();
    assert_eq!(aa.top_down_update, 0);

    machine.top_down_update();
    assert!(matches!(machine.state(), TesterState::BA));

    assert_eq!(machine.top_ref().top_down_update, 1);

    let b: &B = machine.state_ref().unwrap();
    assert_eq!(b.top_down_update, 1);

    let ba: &BA = machine.state_ref().unwrap();
    assert_eq!(ba.top_down_update, 1);

    machine.top_down_update();
    assert!(matches!(machine.state(), TesterState::BA));

    assert_eq!(machine.top_ref().top_down_update, 2);

    let b: &B = machine.state_ref().unwrap();
    assert_eq!(b.top_down_update, 2);

    let ba: &BA = machine.state_ref().unwrap();
    assert_eq!(ba.top_down_update, 2);
}
