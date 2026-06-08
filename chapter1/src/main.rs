use std::future::Future;
use std::pin::{Pin, pin};
use std::sync::{Arc, Mutex};
use std::task::Wake;
use std::task::{Context, Poll, Waker};
use std::thread::Thread;

struct Sender {
    inner: Arc<Mutex<Inner>>,
}
struct Receiver {
    inner: Arc<Mutex<Inner>>,
}

struct Inner {
    value: Option<String>,
    waker: Option<Waker>,
}

fn oneshot() -> (Sender, Receiver) {
    let inner = Arc::new(Mutex::new(Inner {
        value: None,
        waker: None,
    }));
    (
        Sender {
            inner: inner.clone(),
        },
        Receiver { inner },
    )
}

impl Sender {
    fn send(self, value: String) {
        let mut inner = self.inner.lock().unwrap();
        inner.value = Some(value);
        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }
    }
}

impl Future for Receiver {
    type Output = String;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        let mut inner = self.inner.lock().unwrap();
        if let Some(value) = inner.value.take() {
            Poll::Ready(value)
        } else {
            inner.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

struct ThreadWaker(Thread);
impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => return value,
            Poll::Pending => thread::park(),
        }
    }
}

use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx) = oneshot();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        tx.send("a fresh database row".to_string());
    });
    let row = block_on(rx);
    println!("{row}");
}
