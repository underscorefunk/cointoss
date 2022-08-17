/// Invocation behaviours
///
/// Required communication busses
///     -> Invocation to parent
///     -> Parent to invocation
///         -> Invocation to Actor
///         -> Actor to Invocation
///
/// When an invocation is started
///
/// >   Actor => data + strategy + invocation connection -> into a thread
/// >   State => Started
///
/// The invocation runs in a thread until it is resolved.
/// When resolved, the thread joins back on the
///
/// When an invocation is finished (ok or error)
///
///
/// When an invocation is terminated
///     State => Terminated
///

enum InvocationResult {
    Done,
    Error
}

struct Invocation {

}

impl Invocation {
    pub fn new(

    )

    /// The invocation can receive messages
    /// from the outside word
    pub fn receive() {

    }

    pub fn start() {

    }

}