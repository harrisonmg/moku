#![allow(clippy::upper_case_acronyms)]

use moku::*;
use test_log::test;
use update_trans::machine::*;
use update_trans::*;

#[state_machine]
mod update_trans {
    use moku::*;

    #[machine_module]
    pub mod machine {}

    use machine::UpdateTransState;

    pub struct Top {
        pub update: u8,
        pub top_down_update: u8,
    }

    impl TopState<UpdateTransState> for Top {
        fn update(&mut self) -> Option<UpdateTransState> {
            self.update += 1;
            None
        }

        fn top_down_update(&mut self) -> Option<UpdateTransState> {
            self.top_down_update += 1;
            None
        }
    }

    pub struct A {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(Top)]
    impl State<UpdateTransState> for A {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, UpdateTransState> {
            StateEntry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<UpdateTransState> {
            self.update += 1;
            Some(UpdateTransState::BA)
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> Option<UpdateTransState> {
            self.top_down_update += 1;
            Some(UpdateTransState::BA)
        }
    }

    pub struct AA {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(A)]
    impl State<UpdateTransState> for AA {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, UpdateTransState> {
            StateEntry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<UpdateTransState> {
            self.update += 1;
            None
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> Option<UpdateTransState> {
            self.top_down_update += 1;
            None
        }
    }

    pub struct B {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(Top)]
    impl State<UpdateTransState> for B {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, UpdateTransState> {
            StateEntry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<UpdateTransState> {
            self.update += 1;
            Some(UpdateTransState::BA)
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> Option<UpdateTransState> {
            self.top_down_update += 1;
            Some(UpdateTransState::BA)
        }
    }

    pub struct BA {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(B)]
    impl State<UpdateTransState> for BA {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, UpdateTransState> {
            StateEntry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<UpdateTransState> {
            self.update += 1;
            None
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> Option<UpdateTransState> {
            self.top_down_update += 1;
            None
        }
    }

    pub struct BB {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(B)]
    impl State<UpdateTransState> for BB {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, UpdateTransState> {
            StateEntry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<UpdateTransState> {
            self.update += 1;
            None
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> Option<UpdateTransState> {
            self.top_down_update += 1;
            None
        }
    }

    pub struct BBA {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(BB)]
    impl State<UpdateTransState> for BBA {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, UpdateTransState> {
            StateEntry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<UpdateTransState> {
            self.update += 1;
            Some(UpdateTransState::BA)
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> Option<UpdateTransState> {
            self.top_down_update += 1;
            Some(UpdateTransState::BA)
        }
    }
}

#[test]
fn state_chart() {
    assert_eq!(
        UPDATE_TRANS_STATE_CHART,
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
    let mut machine = UpdateTransMachineBuilder::new(Top {
        update: 0,
        top_down_update: 0,
    })
    .build();

    machine.transition(UpdateTransState::AA);
    assert!(matches!(machine.state(), UpdateTransState::AA));

    assert_eq!(machine.top_ref().update, 0);

    let a: &A = machine.state_ref().unwrap();
    assert_eq!(a.update, 0);

    let aa: &AA = machine.state_ref().unwrap();
    assert_eq!(aa.update, 0);

    machine.update();
    assert!(matches!(machine.state(), UpdateTransState::BA));

    assert_eq!(machine.top_ref().update, 1);

    let b: &B = machine.state_ref().unwrap();
    assert_eq!(b.update, 0);

    let ba: &BA = machine.state_ref().unwrap();
    assert_eq!(ba.update, 0);

    machine.update();
    assert!(matches!(machine.state(), UpdateTransState::BA));

    assert_eq!(machine.top_ref().update, 2);

    let b: &B = machine.state_ref().unwrap();
    assert_eq!(b.update, 1);

    let ba: &BA = machine.state_ref().unwrap();
    assert_eq!(ba.update, 1);

    machine.transition(UpdateTransState::BBA);
    assert!(matches!(machine.state(), UpdateTransState::BBA));

    machine.update();
    assert!(matches!(machine.state(), UpdateTransState::BA));

    assert_eq!(machine.top_ref().update, 3);

    let b: &B = machine.state_ref().unwrap();
    assert_eq!(b.update, 2);

    let ba: &BA = machine.state_ref().unwrap();
    assert_eq!(ba.update, 0);
}

#[test]
fn top_down_update() {
    let mut machine = UpdateTransMachineBuilder::new(Top {
        update: 0,
        top_down_update: 0,
    })
    .build();

    machine.transition(UpdateTransState::AA);
    assert!(matches!(machine.state(), UpdateTransState::AA));

    assert_eq!(machine.top_ref().top_down_update, 0);

    let a: &A = machine.state_ref().unwrap();
    assert_eq!(a.top_down_update, 0);

    let aa: &AA = machine.state_ref().unwrap();
    assert_eq!(aa.top_down_update, 0);

    machine.top_down_update();
    assert!(matches!(machine.state(), UpdateTransState::BA));

    assert_eq!(machine.top_ref().top_down_update, 1);

    let b: &B = machine.state_ref().unwrap();
    assert_eq!(b.top_down_update, 1);

    let ba: &BA = machine.state_ref().unwrap();
    assert_eq!(ba.top_down_update, 1);

    machine.top_down_update();
    assert!(matches!(machine.state(), UpdateTransState::BA));

    assert_eq!(machine.top_ref().top_down_update, 2);

    let b: &B = machine.state_ref().unwrap();
    assert_eq!(b.top_down_update, 2);

    let ba: &BA = machine.state_ref().unwrap();
    assert_eq!(ba.top_down_update, 2);
}
