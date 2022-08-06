// SCHEMA
// ------
// Enumerate the game states which contain their state capabilities

#[derive(Debug)]
enum GameState {
    Idle(GameIdle),
    TossingCoin(GameTossingCoin),
    Win(GameWin),
}

// CONTEXT
// -------
// Define the shared data that is unique to an instance of a machine

#[derive(Debug)]
struct GameContext {
    wins: i8,
}

// STATE
// -----
// A game state struct holds the capabilities of a given state in context

#[derive(Debug)]
struct GameIdle {
    context: GameContext,
}

// STATE BEHAVIOURS
// ----------------
// Lifecycle hooks/actions, behaviours, and possible transition methods.
// How you would interact with the state.

impl GameIdle {
    // STATE INITIALIZATION
    // --------------------
    // Always takes context for its "state". The struct is the state.
    pub fn new(context: GameContext) -> GameState {

        // LIFECYCLE: On Enter
        // -------------------
        // On enter, we can do things here to change which state is returned
        // or if things happen to the context as part of state initialization
        GameState::Idle(
            Self { context }
        )
    }

    // TRANSITION
    // ----------
    // Returns a new state. Actions (changes to context) also return game states of the same state.
    // They are like self transitions.

    // Note public access
    pub fn toss(self) -> GameState {

        // To toss a coin we enter the tossing state which does some things on enter
        // This means that the 'toss' method may actually return a different GameState
        // because of it's on enter processing
        GameTossingCoin::new(self.context)
    }
}

#[derive(Debug)]
struct GameTossingCoin {
    context: GameContext,
}


impl GameTossingCoin {
    pub fn new(context: GameContext) -> GameState {

        // ON ENTRY
        // -- ACTION
        let context = Self::toss(context);

        // -- TRANSITION
        Self::done(context)

        // THERE IS NO DEFAULT/STANDARD RETURN BECAUSE OF THE done action
        // GameState::TossingCoin(
        //     Self { context }
        // )
    }

    // Note private access
    fn toss(context: GameContext) -> GameContext{
        GameContext{
            wins: context.wins + 1
        }
    }

    // Note private access
    fn done(context: GameContext) -> GameState {
        match context.wins {
            3 => GameWin::new( context ),
            _ => GameIdle::new( context )
        }
    }
}

#[derive(Debug)]
struct GameWin {
    context: GameContext,
}

impl GameWin {
    pub fn new(context: GameContext) -> GameState {
        GameState::Win(
            Self { context }
        )
    }
}

fn main() {
    println!("Coin Toss Game");

    // Create a new machine
    let mut game_machine = GameIdle::new(GameContext { wins: 0 });

    while let GameState::Idle(game) = game_machine {
        game_machine = game.toss();
    }

    println!("{:?}", game_machine);
}
