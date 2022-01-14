/*
 * File: common.rs
 * Project: RpiOS
 * File Created: Thursday, 30th December 2021 11:36:34 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

/* TODO: Taken as is from the project, I can't fully understand everything thats going on at the moment
but I hope soon enough. 
this helps: https://github.com/rust-embedded/register-rs */


//! Common device driver code.

use core::{marker::PhantomData, ops};

//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------

pub struct MMIODerefWrapper<T> {
    start_addr: usize,
    phantom: PhantomData<fn() -> T>,
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

impl<T> MMIODerefWrapper<T> {
    /// Create an instance.
    pub const unsafe fn new(start_addr: usize) -> Self {
        Self {
            start_addr,
            phantom: PhantomData,
        }
    }
}

impl<T> ops::Deref for MMIODerefWrapper<T> {
    type Target = T;

    // Treat start_addr as the start of the registers struct.
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.start_addr as *const _) }
   }
} 