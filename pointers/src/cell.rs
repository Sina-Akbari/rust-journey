use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// Implied by UnsafeCell
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know no-one else concurrently mutating self.value (because !Sync).
        // SAFETY: we know we're not invalidating any references, because we never give any out.
        unsafe { *self.value.get() = value }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know no-one else is mutating this value, since only this thread can mutate
        // (because !Sync), and it's executing this function instead
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    // This test won't pass because Cell is not thread safe
    #[test]
    fn broken() {
        use std::sync::Arc;
        use std::thread;

        let x = Arc::new(Cell::new(0));
        let x1 = Arc::clone(&x);
        let t1 = thread::spawn(|| {
            for _ in 0..100000 {
                let x = x1.get();
                x1.set(x + 1);
            }
        });

        let x2 = Arc::clone(&x);
        let t2 = thread::spawn(|| {
            for _ in 0..100000 {
                let x = x2.get();
                x2.set(x + 1);
            }
        });
        t1.join();
        t2.join();

        assert_eq!(x.get(), 200000);
    }
}
