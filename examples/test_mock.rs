use example::*;
use moku::*;

/// Some real hardware interface.
pub struct Gpio;

impl Gpio {
    pub fn set_high(&mut self) {}
    pub fn set_low(&mut self) {}
}

/// A test mock for hardware.
pub struct TestGpio {
    level: bool,
}

impl TestGpio {
    pub fn set_high(&mut self) {
        self.level = true;
    }

    pub fn set_low(&mut self) {
        self.level = false;
    }
}

#[state_machine]
mod example {
    use moku::*;

    #[allow(unused_imports)]
    use super::{Gpio, TestGpio};

    #[machine_module]
    mod machine {}
    pub use machine::*;

    pub struct Top {
        #[cfg(not(test))]
        pub gpio: Gpio,

        #[cfg(test)]
        pub gpio: TestGpio,
    }

    impl Top {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            #[cfg(not(test))]
            let gpio = Gpio;

            #[cfg(test)]
            let gpio = TestGpio { level: false };

            Self { gpio }
        }
    }

    impl TopState for Top {}

    struct Foo;

    impl Substate<Top> for Foo {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            ctx.top.gpio.set_high();
            Self
        }
    }

    struct Bar;

    impl Substate<Top> for Bar {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            ctx.top.gpio.set_low();
            Self
        }
    }
}

fn main() {
    // Non-test code can use the real hardware interface [Gpio].
    let _machine = Builder::new(Top::new()).build();
}

#[cfg(test)]
mod tests {
    use crate::{
        example::{Builder, State, Top},
        TestGpio,
    };
    use moku::*;

    #[test]
    fn test_level() {
        // Test code can use the test interface [TestGpio].
        let mut machine = Builder::new(Top::new()).build();

        machine.transition(State::Foo);
        assert!(machine.top_ref().gpio.level);

        machine.transition(State::Bar);
        assert!(!machine.top_ref().gpio.level);
    }
}
