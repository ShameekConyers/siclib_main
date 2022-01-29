#![no_implicit_prelude]
#![feature(allocator_api, ptr_internals)]
#![feature(inherent_associated_types)]
#![cfg(target_arch = "x86_64")]
#![macro_use]
extern crate std;

pub mod algorithms;
pub mod collections;
pub mod linalg;
