#[allow(unused)]
use std::convert::Infallible;

use tokio::{
    select,
    sync::broadcast::{channel, error::SendError, Receiver, Sender},
    time::{sleep, Duration},
};

async fn populate_stream(s: Sender<&str>) {
    let result = s.send("I'm data!");
    match result {
        Ok(_) => {
            println!("ok — sending to stream")
        }
        Err(_) => {
            println!("error — sending to stream")
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Infallible> {
    //
    // Next steps
    // ----------
    // @todo - Find a way to have multiple async tasks pushing to the queue
    //       - Find a way to continually consume the queue without blocking additions
    //
    //      The channel will become the event queue of a machine
    //      The machine's event loop will process events or pause if there are no events
    //      Other async tasks need to be able to ask the machine to do something, and then
    //      that machine will take the list of things to do and try to do them.
    //      Use a sandwich shop, front of house, and the kitchen as a domain example.

    // We create a sender and a single mutable receiver to process channel messages.
    // Only one single entity should be responsible for dealing with things on the queue.
    let (send, mut receive): (Sender<&str>, Receiver<&str>) = channel(10);

    // Add a message to the queue. Though this is async, we're awaiting here and there
    // are no other async tasks so it will be forced to run to completion.
    populate_stream(send).await;

    // We pluck the message off of the queue to confirm things worked as expected.
    if let Ok(message) = receive.recv().await {
        println!("{}", message);
    }

    Ok(())
}
