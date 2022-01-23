extern crate std;

use std::alloc::{self, Allocator, Layout};
use std::marker::{PhantomData, Send, Sync};
use std::ptr::{self, NonNull};
use std::{isize, mem};
pub struct Vector<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T: Sync> Sync for Vector<T> {}
unsafe impl<T: Send> Send for Vector<T> {}
trait Baz: Send + Sync {}
impl<T> Vector<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "No zero sized types");

        return Vector {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
            _marker: PhantomData,
        };
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe { ptr::write(self.get_tail(), elem) }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        } else {
            self.len -= 1;
            unsafe {
                return Some(ptr::read(self.get_tail()));
            }
        }
    }
    pub fn len(&self) -> usize {
        return self.len.clone();
    }

    fn get_tail(&self) -> *mut T {
        return unsafe { self.ptr.as_ptr().add(self.len()) };
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let tmp_new_cap = 2 * self.cap;
            let tmp_new_layout = Layout::array::<T>(tmp_new_cap).unwrap();
            (tmp_new_cap, tmp_new_layout)
        };

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let tmp_old_layout = Layout::array::<T>(self.cap).unwrap();
            let tmp_old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(tmp_old_ptr, tmp_old_layout, new_layout.size()) }
        };

        let new_ptr_optional = NonNull::new(new_ptr as *mut T);
        self.ptr = if new_ptr_optional.is_some() {
            new_ptr_optional.unwrap()
        } else {
            alloc::handle_alloc_error(new_layout);
        };
        self.cap = new_cap;
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while self.len() != 0 {
                self.pop();
            }

            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}
