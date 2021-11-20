/*
 * File: synchronization.rs
 * Project: RpiOS
 * File Created: Saturday, 20th November 2021 1:25:53 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

use core::cell::UnsafeCell;

pub mod interface {
    /// Any object wrapt with this mutex trait guarantees exclusive access,
    /// ensures not data races.
    pub trait Mutex {
        /// The data type wrapped with the mutex
        type Data;

        /// Locks the mutex, and grants the closure exclusive `mutable` access
        /// the the data within
        fn lock<R, F>(&self, func: F) -> R
        where
            F: FnOnce(&mut Self::Data) -> R;
    }
}

/// Dummy lock that doesn't really do anything (i.e doesn't support multithreaded operations for instance).
/// This is a proof of concept: if we wanted to have a global object but access it from multiple parts of the program
/// we may want wrap it with a lock and once we move to multithreaded switch to a real lock but have the same interface.
/// Another way of looking at it: If we wanted to have a global object we would probably use `static`. But this is useless
/// if we can't modify the object's state, so we would change the object to `static mut`. The problem is rust can't guarantee
/// the static object won't be used it multiple threads at the same time, which will cause data races. Because of this, any access
/// to *static mutable* item is unsafe.
/// https://doc.rust-lang.org/reference/items/static-items.html
/// Wrapping it in a lock may help (core:cell:Cell?). In the case of my RpiOS, the lock function doesn't really do anything besided
/// calling get on the object in an unsafe block. In the future, where multithreaded will be used, the lock function will probably use
/// atomic operations or whatever, to ensure exclusive access to the data.

pub struct NullLock<T>
where
    T: ?Sized,
{
    data: UnsafeCell<T>,
}

impl<T> NullLock<T> {
    /// Create null lock instance
    pub const fn new(value: T) -> Self {
        NullLock {
            data: UnsafeCell::new(value),
        }
    }
}

unsafe impl<T> Send for NullLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for NullLock<T> where T: ?Sized + Send {}

/// Implement the lock trait
impl<T> interface::Mutex for NullLock<T> {
    type Data = T;

    fn lock<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&mut Self::Data) -> R,
    {
        // when multithreaded is used, this function will have to verify exclusive access to data.
        // because this is a single thread, there is no way for concurrrent access.
        // right now I can be sure this exclusive reference is indeed exclusive.
        let data = unsafe { &mut *self.data.get() };
        func(data)
    }
}
