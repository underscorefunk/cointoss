// @todo - Remove mutable
// @todo - Event queue
// @todo - Internal private events, external public events
// @todo - Add initialization events
// @todo - Event data (use a closure to wrap and apply it)
// @todo - Consider adding _uninitialized state

// Each state variant should have it's config.
// - On Enter
// - On Event

#[derive(Debug, Clone, PartialEq)]
enum State {
    Idle,
    TossingCoin,
    Win,
}

#[derive(Debug, Clone)]
enum Event {
    Toss,
}

#[derive(Debug, Clone)]
struct Context {
    wins: i8,
}

#[derive(Debug, Clone)]
struct Machine {
    context: Context,
    state: State,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            state: State::Idle,
            context: Context { wins: 0 }
        }
    }

    pub fn state(&self) -> State {
        self.state.clone()
    }

    pub fn context(&self) -> Context {
        self.context.clone()
    }

    pub fn send(&mut self, event: Event) {
        match self.state {

            State::Idle => match event {
                Event::Toss => {

                    // Transition state
                    self.state = State::TossingCoin;

                    // coinToss Action
                    // Do on enter actions (imagine we invoke here)
                    self.context.wins += 1;

                    //Guarded transition
                    if self.context.wins >= 3 {
                        self.state = State::Win;
                        return;
                    }

                    // Forced transition
                    self.state = State::Idle;

                },
                _ => ()
            },
            _ => ()
        }
    }

}


fn main() {
    println!("Coin Toss Game");
    let mut game = Machine::new();
    println!("{:?}", game);

    game.send( Event::Toss );
    game.send( Event::Toss );
    game.send( Event::Toss );
    game.send( Event::Toss );

    println!("{:?}", game);
}
