//! This crate provides a LazyCell struct which acts as a lazily filled Cell,
//! but with frozen contents.
//!
//! With a `RefCell`, the inner contents cannot be borrowed for the lifetime of
//! the entire object, but only of the borrows returned. A `LazyCell` is a
//! variation on `RefCell` which allows borrows tied to the lifetime of the
//! outer object.
//!
//! The limitation of a `LazyCell` is that after initialized, it can never be
//! modified.

#![cfg_attr(feature = "nightly", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use std::cell::RefCell;
use std::mem;

pub struct LazyCell<T> {
    inner: RefCell<Option<T>>,
}

impl<T> LazyCell<T> {
    /// Creates a new empty lazy cell.
    pub fn new() -> LazyCell<T> {
        LazyCell { inner: RefCell::new(None) }
    }

    /// Put a value into this cell.
    ///
    /// This function will fail if the cell has already been filled.
    pub fn fill(&self, t: T) {
        let mut slot = self.inner.borrow_mut();
        if slot.is_some() {
            panic!("lazy cell is already filled")
        }
        *slot = Some(t);
    }

    /// Test whether this cell has been previously filled.
    pub fn filled(&self) -> bool {
        self.inner.borrow().is_some()
    }

    /// Borrows the contents of this lazy cell for the duration of the cell
    /// itself.
    ///
    /// This function will return `Some` if the cell has been previously
    /// initialized, and `None` if it has not yet been initialized.
    pub fn borrow(&self) -> Option<&T> {
        match *self.inner.borrow() {
            Some(ref inner) => unsafe { Some(mem::transmute(inner)) },
            None => None,
        }
    }

    /// Consumes this `LazyCell`, returning the underlying value.
    pub fn into_inner(self) -> Option<T> {
        self.inner.into_inner()
    }
}
