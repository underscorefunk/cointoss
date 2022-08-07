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
    _Uninitialized,
    Idle,
    TossingCoin,
    Win,
}

#[derive(Debug, Clone)]
enum Event {
    _Init,
    Toss,
}

#[derive(Debug, Clone)]
struct Context {
    wins: i8,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            wins: 0
        }
    }
}

#[derive(Debug, Clone)]
struct Machine {
    context: Context,
    state: State,
}

impl Machine {
    /// Initialize a machine to it's first state
    pub fn new() -> Self {
        Self {
            state: State::_Uninitialized,
            context: Context::default()
        }
    }

    pub fn start(&mut self) {
        self.send(Event::_Init)
    }

    /// State accessor so that we can keep state borrow free
    pub fn state(&self) -> State {
        self.state.clone()
    }

    /// Context accessor so that we can keep context borrow free
    pub fn context(&self) -> Context {
        self.context.clone()
    }

    pub fn send(&mut self, event: Event) {
        match self.state {

            State::_Uninitialized => match event {
                Event::_Init => {

                    // On Enter Actions? Set score?

                    // Transiton State
                    self.state = State::Idle;

                },
                _ => {}
            },

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
                _ => {}
            },
            _ => {}
        }
    }

}


fn main() {
    println!("Coin Toss Game");
    let mut game = Machine::new();
    println!("{:?}", game);

    game.start();
    println!("{:?}", game);

    game.send( Event::Toss );
    game.send( Event::Toss );
    game.send( Event::Toss );
    game.send( Event::Toss );

    println!("{:?}", game);
}
