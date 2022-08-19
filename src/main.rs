use cointoss;
use std::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
};

fn main() {
    // We setup a FnOnce tasks
    let task = || Ok(24);

    // We setup message channels so that we can get the response back
    let (send, recv): (
        Sender<Result<i32, String>>,
        std::sync::mpsc::Receiver<Result<i32, String>>,
    ) = mpsc::channel();

    // Spawn the invocation
    // ISSUE Currently the invocation is waiting for
    // its task to complete to join the thread back, which
    // blocks the main task. We need to fix this so that the
    // listening/parent channel is async as well.
    cointoss::invocation::Invocation::spawn(task, send);

    // This is a bit fake because the above waited until the task completed
    // to continue. We know that the receiver channel has a message waiting
    // in its queue. This is just proof of concept.
    if let Ok(msg) = recv.recv() {
        match msg {
            Ok(result) => println!("Invocation yielded {}", result),
            Err(_) => println!("Invocation error"),
        }
    }
}
