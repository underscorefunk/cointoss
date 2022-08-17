#[derive(Debug, PartialEq)]
enum State {
    Idle,
}

#[derive(Debug)]
struct Machine {}
impl Machine {
    pub fn state(&self) -> State {
        State::Idle
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initial_machine() {
        let coin_toss = Machine {};
        assert_eq!(coin_toss.state(), State::Idle);
    }
}
