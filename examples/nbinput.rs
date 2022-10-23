use std::future;

use futures::StreamExt;
use termion::{
    event::Key,
    raw::IntoRawMode
};
use termion_input_tokio::TermReadAsync;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Disable line buffering, local echo, etc.
    let _raw_term = std::io::stdout().into_raw_mode()?;

    tokio::io::stdin()
        .keys_stream()
        // End the stream when 'q' is pressed.
        .take_while(|event| {
            future::ready(match event {
                Ok(Key::Char('q')) => false,
                _ => {
                    println!("its working");
                    true
                }
            })
        })
        // Print each key that was pressed.
        .for_each(|event| async move {
            println!("{:?}\r", event);
        })
        .await;

    println!("asd");
    Ok(())
}
