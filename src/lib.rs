//!Wrapper for time functions of C standard library
//!
//!This provides minimal and uniform API to use in Rust without need for cumbersome crates like time
#![no_std]
#![warn(missing_docs)]
#![allow(clippy::style)]

pub mod format;
pub mod sys;
