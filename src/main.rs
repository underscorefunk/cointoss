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
    let (send, mut receive): (Sender<&str>, Receiver<&str>) = channel(10);

    populate_stream(send).await;

    if let Ok(message) = receive.recv().await {
        println!("{}", message);
    }

    Ok(())
}
