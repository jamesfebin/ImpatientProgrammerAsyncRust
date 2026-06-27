use crate::oneshot::Receiver;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub enum CartTotal {
    Start {
        socks: Receiver<i64>,
        shoes: Receiver<i64>,
    },
    GotSocks {
        socks_price: i64,
        shoes: Receiver<i64>,
    },
    Done,
}

impl CartTotal {
    pub fn new(socks: Receiver<i64>, shoes: Receiver<i64>) -> Self {
        CartTotal::Start { socks, shoes }
    }
}

impl Future for CartTotal {
    type Output = i64;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<i64> {
        let this = self.get_mut();
        loop {
            match this {
                CartTotal::Start { socks, .. } => match Pin::new(socks).poll(cx) {
                    Poll::Pending => {
                        println!("[cart] Start: socks price not back yet, waiting");
                        return Poll::Pending;
                    }
                    Poll::Ready(socks_price) => {
                        println!("[cart] Start -> GotSocks: socks are ${socks_price}");
                        let CartTotal::Start { shoes, .. } =
                            std::mem::replace(this, CartTotal::Done)
                        else {
                            unreachable!()
                        };
                        *this = CartTotal::GotSocks { socks_price, shoes };
                    }
                },
                CartTotal::GotSocks { socks_price, shoes } => match Pin::new(shoes).poll(cx) {
                    Poll::Pending => {
                        println!(
                            "[cart] GotSocks: shoes price not back yet, ${socks_price} waits as a field"
                        );
                        return Poll::Pending;
                    }
                    Poll::Ready(shoes_price) => {
                        let total = *socks_price + shoes_price;
                        println!(
                            "[cart] GotSocks -> Done: shoes are ${shoes_price}, cart total = ${total}"
                        );
                        *this = CartTotal::Done;
                        return Poll::Ready(total);
                    }
                },
                CartTotal::Done => {
                    panic!("polled CartTotal after it already returned the total")
                }
            }
        }
    }
}
