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
    coin_toss: Invocation,
}

#[derive(Debug)]
struct Invocation {
    //@todo Create ability for an invocation to accept a
    //      once closure to put in a thread. It should have
    //      a maybe to see if the invocation is running or not.
    //
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            state: State::Idle,
            coin_toss: Invocation {},
        }
    }
}

impl Machine {
    pub fn state(&self) -> State {
        self.state
    }

    fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    pub fn receive(&mut self, e: Event) {
        match self.state {
            State::Idle => match e {
                Event::Toss => self.transition(State::TossingCoin, e),
                _ => {}
            },
            _ => {}
        }
    }

    fn transition(&mut self, to: State, e: Event) {
        let from = self.state;

        if Machine::is_transition_to(&from, &to, &State::TossingCoin) {
            // invoke the coin tosser
        }

        if Machine::is_transition_away(&State::TossingCoin, &to) {
            // clean up the coin tosser
        }

        self.set_state(to);
    }

    fn is_transition_to(from: &State, to: &State, target: &State) -> bool {
        Machine::is_transition_away(from, to) && to == target
    }

    fn is_transition_away(from: &State, to: &State) -> bool {
        from != to
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

    #[test]
    fn invoke_toss() {
        //@todo figure out how to test the invocation
        //      or what the best way is to kick it off.
        //      Possibly part of "on enter" then envoke?
        //      Allow coinToss to accept a waiting period
        //      so that we can test the error.
        //      Create a channel, spawn a thread,
    }

    #[test]
    fn last_event() {
        // @todo Confirm that the last event is stored, even if
        //       the event was not a supported event. In thst case
    }
}
