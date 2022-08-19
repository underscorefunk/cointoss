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

pub struct Invocation {}

use std::marker::Send;
use std::sync::mpsc::{Receiver, SendError, Sender};
use std::thread;
use std::thread::JoinHandle;

impl Invocation {
    pub fn spawn<Task, Response, Error>(task: Task, messages: Sender<Result<Response, Error>>) -> ()
    where
        Task: FnOnce() -> Result<Response, Error> + Send + 'static,
        Response: Send + 'static,
        Error: Send + 'static,
    {
        let t = thread::spawn(move || messages.send(task()));

        // Join will cause this thread to wait for processes to finish
        // this makes invocations blocking and we need to fix that!
        t.join();
    }
}
