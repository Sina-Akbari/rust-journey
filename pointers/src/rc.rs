use super::cell::Cell;
use std::{ops::Deref, ptr::NonNull};

struct RcInner<T> {
    value: T,
    ref_count: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
}

impl<T> Rc<T> {
    fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            ref_count: Cell::new(1),
        });
        Self {
            // SAFETY: Box won't give us a null pointer
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last Rc goes out of scope.
        // we have an Rc, so the Box hasn't been deallocated, therefor deref is fine.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let ref_count = inner.ref_count.get();
        inner.ref_count.set(ref_count + 1);

        Rc { inner: self.inner }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.ref_count.get();

        if c == 1 {
            // SAFETY: It's the only reference, and after it, there won't be any other references to T.
            // so drop the Box
            drop(inner);
            unsafe {
                let _ = Box::from_raw(self.inner.as_ptr());
            }
        } else {
            // SAFETY: There are more references to T. So don't drop the Box.
            inner.ref_count.set(c - 1);
        }
    }
}
