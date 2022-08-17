/// TDD Goodness is unloacked via
/// cargo watch -x test

#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
enum State {
    Idle,
    TossingCoin,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
enum Event {
    Toss,
}

#[derive(Debug)]
struct Machine {
    state: State,
}

impl Default for Machine {
    fn default() -> Self {
        Self { state: State::Idle }
    }
}

impl Machine {
    pub fn state(&self) -> State {
        self.state
    }

    pub fn receive(&mut self, e: Event) {
        match self.state {
            State::Idle => match e {
                Event::Toss => self.state = State::TossingCoin,
                _ => {}
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initial_machine() {
        let m = Machine::default();
        assert_eq!(m.state(), State::Idle);
    }

    #[test]
    fn first_toss() {
        let mut m = Machine::default();
        m.receive(Event::Toss);
        assert_eq!(m.state(), State::TossingCoin);
    }
}
