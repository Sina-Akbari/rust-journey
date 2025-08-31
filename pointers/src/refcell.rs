use crate::cell::Cell;
use std::cell::UnsafeCell;

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

// Implied by UnsafeCell
// impl<T> !Sync for RefCell<T> {}

impl<T> RefCell<T> {
    fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    fn borrow(&mut self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                // SAFETY: no exclusive references have been given out since state would be Exclusive.
                Some(Ref { refcell: self })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                // SAFETY: no exclusive references have been given out since state would be Exclusive.
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }
    fn borrow_mut(&mut self) -> Option<RefMut<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Exclusive);
                // SAFETY: no other references have been given out since state would be Shared/Exclusive.
                Some(RefMut { refcell: self })
            }
            _ => None,
        }
    }
}

struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: a ref is only created if no exclusive references have been given out.
        // once it's given out, the state is set to Shared, so no exclusive references are given out.
        // So derefencing to a shared ref is safe.
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => !unreachable!(),
            RefState::Shared(1) => self.refcell.state.set(RefState::Unshared),
            RefState::Shared(n) => self.refcell.state.set(RefState::Shared(n - 1)),
        }
    }
}

struct RefMut<'refcell, T> {
    refcell: &'refcell mut RefCell<T>,
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: a RefMut is only created if no other reference has been given out.
        // once it's given out, the state is set to Exclusive, so no future references are given out.
        // so we have an exclusive lease on the inner value and derefencing is fine.
        unsafe { &*self.refcell.value.get() }
    }
}
impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: a RefMut is only created if no other reference has been given out.
        // once it's given out, the state is set to Exclusive, so no future references are given out.
        // so we have an exclusive lease on the inner value and mutably derefencing is fine.
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => !unreachable!(),
            RefState::Exclusive => self.refcell.state.set(RefState::Unshared),
        }
    }
}
