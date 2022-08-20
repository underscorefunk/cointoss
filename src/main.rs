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
