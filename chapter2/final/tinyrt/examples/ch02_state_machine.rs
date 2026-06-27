use std::thread;
use std::time::Duration;
use tinyrt::block_on;
use tinyrt::oneshot::{self, Receiver};
use tinyrt::state_machine::CartTotal;

fn price_lookups() -> (Receiver<i64>, Receiver<i64>) {
    let (socks_tx, socks_rx) = oneshot::channel::<i64>();
    let (shoes_tx, shoes_rx) = oneshot::channel::<i64>();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        socks_tx.send(12);
    });
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(400));
        shoes_tx.send(89);
    });
    (socks_rx, shoes_rx)
}

fn main() {
    let (socks, shoes) = price_lookups();
    let total = block_on(CartTotal::new(socks, shoes));
    println!("cart total = ${total}");
}
