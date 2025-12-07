use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

// Flavors of Channels
// Synchronous: Channel where send can block. Limited Capacity.
//  Mutex + Condvar + VecDeque
//  Atmoic VecDeque (atomic queue) + thread::park + thread::Thread::notify
// Asynchronous: Channel where send cannot block. Unbound.
//  Mutex + Condvar + VecDeque
//  Mutex + Condvar + LinkedList
//  Atomic LinkedList, LinkedList of T
//  Atomic block LinkedList, LinkedList of Atomic VecDeque<T>
// Rendevouz: Synchronous channel where capacity is 0. Sync sender and recevier threads(They meet at the rendevouz)
// Oneshot: Any capacity. In practice, only one call to send.

// async

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);

        Sender {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;

        drop(inner);

        if was_last {
            self.shared.available.notify_one();
        }
    }
}

impl<T> Sender<T> {
    fn send(&self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap();

        inner.queue.push_back(t);
        drop(inner);
        self.shared.available.notify_one();
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> Receiver<T> {
    fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }

        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    if !inner.queue.is_empty() {
                        std::mem::swap(&mut self.buffer, &mut inner.queue);
                    }

                    return Some(t);
                }
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::default(),
        senders: 1,
    };
    let shared = Shared {
        inner: Mutex::new(inner),
        available: Condvar::new(),
    };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
            buffer: VecDeque::default(),
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ping_pong() {
        let (tx, mut rx) = channel();
        tx.send(42);

        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn barrage() {
        let result = [42, 60, 70];
        let (tx, rx) = channel();
        tx.send(result[0]);
        tx.send(result[1]);
        tx.send(result[2]);
        drop(tx);

        rx.enumerate().for_each(|(i, x)| assert_eq!(x, result[i]));
    }

    #[test]
    fn closed_tx() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn closed_rx() {
        let (tx, rx) = channel();
        drop(rx);
        tx.send(42);
    }
}
