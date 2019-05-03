//! This crate is experimental.
//!
//! It defines a [pointer::Ptr] datastructure,
//! which is ought to be a wrapper of a pointer-like type.
//! It wraps normal pointers, slice's, and trait objects.

#![no_std]

pub mod access;
pub mod pointee;
pub mod pointer;
pub mod trait_obj;

#[cfg(test)]
mod tests;
