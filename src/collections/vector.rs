extern crate std;

use std::alloc::Allocator;
use std::marker::{PhantomData, Sync};
use std::mem;
use std::ptr::NonNull;

pub struct Vector<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T: Sync> Sync for Vector<T> {}

impl<T> Vector<T> {
    pub fn new() -> Self {
        return Vector {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
            _marker: PhantomData,
        };
    }
}
