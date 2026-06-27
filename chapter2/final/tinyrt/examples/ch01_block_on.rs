use std::thread;
use std::time::Duration;

use tinyrt::{block_on, oneshot};

fn main() {
    let (tx, rx) = oneshot::channel::<&'static str>();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        tx.send("a fresh database row");
    });

    let row = block_on(rx);
    println!("{row}");
}
