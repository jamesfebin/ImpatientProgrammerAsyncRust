use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

struct Inner<T> {
    value: Option<T>,
    waker: Option<Waker>,
}

pub struct Sender<T> {
    inner: Arc<Mutex<Inner<T>>>,
}

pub struct Receiver<T> {
    inner: Arc<Mutex<Inner<T>>>,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Arc::new(Mutex::new(Inner {
        value: None,
        waker: None,
    }));
    (
        Sender {
            inner: Arc::clone(&inner),
        },
        Receiver { inner },
    )
}

impl<T> Sender<T> {
    pub fn send(self, value: T) {
        let mut inner = self.inner.lock().unwrap();
        inner.value = Some(value);
        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }
    }
}

impl<T> Future for Receiver<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let mut inner = self.inner.lock().unwrap();
        match inner.value.take() {
            Some(value) => Poll::Ready(value),
            None => {
                inner.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}
