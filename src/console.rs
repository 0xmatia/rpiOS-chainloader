/*
 * File: console.rs
 * Project: RpiOS
 * File Created: Saturday, 6th November 2021 5:17:59 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

 pub mod interface {
    pub use core::fmt;
    
   /// Console write functions
   pub trait Write {
      /// write format string trait
      fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
   }

   /// console statisitcs
   pub trait Statisitcs {
      /// returns the number of characters written
      fn chars_written(&self) -> usize;
   }

   /// trait alias: All for output interface that needs to implement
   /// both trait
   pub trait All = Write + Statisitcs;
 }