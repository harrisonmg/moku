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

    pub struct Foo {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(Top)]
    impl State<UpdateTransState> for Foo {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, UpdateTransState> {
            StateEntry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<UpdateTransState> {
            self.update += 1;
            Some(UpdateTransState::Buzz)
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> Option<UpdateTransState> {
            self.top_down_update += 1;
            Some(UpdateTransState::Buzz)
        }
    }

    pub struct Bar {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(Foo)]
    impl State<UpdateTransState> for Bar {
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

    pub struct Fizz {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(Top)]
    impl State<UpdateTransState> for Fizz {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, UpdateTransState> {
            StateEntry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<UpdateTransState> {
            self.update += 1;
            Some(UpdateTransState::Buzz)
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> Option<UpdateTransState> {
            self.top_down_update += 1;
            Some(UpdateTransState::Buzz)
        }
    }

    pub struct Buzz {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(Fizz)]
    impl State<UpdateTransState> for Buzz {
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

    pub struct Qux {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(Fizz)]
    impl State<UpdateTransState> for Qux {
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

    pub struct Quz {
        pub update: u8,
        pub top_down_update: u8,
    }

    #[superstate(Qux)]
    impl State<UpdateTransState> for Quz {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, UpdateTransState> {
            StateEntry::State(Self {
                update: 0,
                top_down_update: 0,
            })
        }

        fn update(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<UpdateTransState> {
            self.update += 1;
            Some(UpdateTransState::Buzz)
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> Option<UpdateTransState> {
            self.top_down_update += 1;
            Some(UpdateTransState::Buzz)
        }
    }
}

#[test]
fn state_chart() {
    assert_eq!(
        UPDATE_TRANS_STATE_CHART,
        "Top
├─ Foo
│  └─ Bar
└─ Fizz
   ├─ Buzz
   └─ Qux
      └─ Quz"
    );
}

#[test]
fn update() {
    let mut machine = UpdateTransMachineBuilder::new(Top {
        update: 0,
        top_down_update: 0,
    })
    .build();

    machine.transition(UpdateTransState::Bar);
    assert!(matches!(machine.state(), UpdateTransState::Bar));

    assert_eq!(machine.top_ref().update, 0);

    let foo: &Foo = machine.state_ref().unwrap();
    assert_eq!(foo.update, 0);

    let bar: &Bar = machine.state_ref().unwrap();
    assert_eq!(bar.update, 0);

    machine.update();
    assert!(matches!(machine.state(), UpdateTransState::Buzz));

    assert_eq!(machine.top_ref().update, 1);

    let fizz: &Fizz = machine.state_ref().unwrap();
    assert_eq!(fizz.update, 0);

    let buzz: &Buzz = machine.state_ref().unwrap();
    assert_eq!(buzz.update, 0);

    machine.update();
    assert!(matches!(machine.state(), UpdateTransState::Buzz));

    assert_eq!(machine.top_ref().update, 2);

    let fizz: &Fizz = machine.state_ref().unwrap();
    assert_eq!(fizz.update, 1);

    let buzz: &Buzz = machine.state_ref().unwrap();
    assert_eq!(buzz.update, 1);

    machine.transition(UpdateTransState::Fizz);
    assert!(matches!(machine.state(), UpdateTransState::Fizz));

    machine.update();
    assert!(matches!(machine.state(), UpdateTransState::Buzz));

    assert_eq!(machine.top_ref().update, 3);

    let fizz: &Fizz = machine.state_ref().unwrap();
    assert_eq!(fizz.update, 2);

    let buzz: &Buzz = machine.state_ref().unwrap();
    assert_eq!(buzz.update, 0);

    machine.transition(UpdateTransState::Quz);
    assert!(matches!(machine.state(), UpdateTransState::Quz));

    machine.update();
    assert!(matches!(machine.state(), UpdateTransState::Buzz));

    assert_eq!(machine.top_ref().update, 4);

    let fizz: &Fizz = machine.state_ref().unwrap();
    assert_eq!(fizz.update, 3);

    let buzz: &Buzz = machine.state_ref().unwrap();
    assert_eq!(buzz.update, 0);
}

#[test]
fn top_down_update() {
    let mut machine = UpdateTransMachineBuilder::new(Top {
        update: 0,
        top_down_update: 0,
    })
    .build();

    machine.transition(UpdateTransState::Bar);
    assert!(matches!(machine.state(), UpdateTransState::Bar));

    assert_eq!(machine.top_ref().top_down_update, 0);

    let foo: &Foo = machine.state_ref().unwrap();
    assert_eq!(foo.top_down_update, 0);

    let bar: &Bar = machine.state_ref().unwrap();
    assert_eq!(bar.top_down_update, 0);

    machine.top_down_update();
    assert!(matches!(machine.state(), UpdateTransState::Buzz));

    assert_eq!(machine.top_ref().top_down_update, 1);

    let fizz: &Fizz = machine.state_ref().unwrap();
    assert_eq!(fizz.top_down_update, 1);

    let buzz: &Buzz = machine.state_ref().unwrap();
    assert_eq!(buzz.top_down_update, 1);

    machine.top_down_update();
    assert!(matches!(machine.state(), UpdateTransState::Buzz));

    assert_eq!(machine.top_ref().top_down_update, 2);

    let fizz: &Fizz = machine.state_ref().unwrap();
    assert_eq!(fizz.top_down_update, 2);

    let buzz: &Buzz = machine.state_ref().unwrap();
    assert_eq!(buzz.top_down_update, 2);

    machine.transition(UpdateTransState::Fizz);
    assert!(matches!(machine.state(), UpdateTransState::Fizz));

    machine.top_down_update();
    assert!(matches!(machine.state(), UpdateTransState::Buzz));

    assert_eq!(machine.top_ref().top_down_update, 3);

    let fizz: &Fizz = machine.state_ref().unwrap();
    assert_eq!(fizz.top_down_update, 3);

    let buzz: &Buzz = machine.state_ref().unwrap();
    assert_eq!(buzz.top_down_update, 1);
}
