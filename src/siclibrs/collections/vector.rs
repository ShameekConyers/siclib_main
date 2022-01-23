use ::std;
use std::alloc;
use std::clone::Clone;
use std::marker;
use std::ops;
use std::option::Option;
use std::ptr;

pub struct Vector<T> {
    ptr: ptr::NonNull<T>,
    cap: usize,
    len: usize,
    _marker: marker::PhantomData<T>,
}

unsafe impl<T: marker::Sync> marker::Sync for Vector<T> {}
unsafe impl<T: marker::Send> marker::Send for Vector<T> {}

//
impl<T> Vector<T> {
    pub fn new() -> Self {
        assert!(std::mem::size_of::<T>() != 0, "No zero sized types");

        return Vector {
            ptr: ptr::NonNull::dangling(),
            cap: 0,
            len: 0,
            _marker: marker::PhantomData,
        };
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        // Note: equality is allowed since its valid to insert at end
        assert!(index <= self.len(), "index out of bounds.");
        if self.cap == self.len {
            self.grow();
        }

        unsafe {
            ptr::copy(
                self.offset(index),
                self.offset(index + 1),
                self.len() - index,
            );
            ptr::write(self.offset(index), elem);
            self.len += 1;
        };
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len(), "index out of bounds.");
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.offset(index));
            ptr::copy(
                self.offset(index + 1),
                self.offset(index),
                self.len() - index,
            );
            return result;
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            //
            ptr::write(self.end(), elem)
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            return Option::None;
        } else {
            self.len -= 1;
            unsafe {
                return Option::Some(ptr::read(self.end()));
            }
        }
    }

    pub fn len(&self) -> usize {
        return self.len.clone();
    }

    unsafe fn begin(&self) -> *mut T {
        return self.ptr.as_ptr();
    }

    unsafe fn end(&self) -> *mut T {
        return self.offset(self.len());
    }

    unsafe fn offset(&self, index: usize) -> *mut T {
        return self.begin().add(index);
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, alloc::Layout::array::<T>(1).unwrap())
        } else {
            let tmp_new_cap = 2 * self.cap;
            let tmp_new_layout = alloc::Layout::array::<T>(tmp_new_cap).unwrap();
            (tmp_new_cap, tmp_new_layout)
        };

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let tmp_old_layout = alloc::Layout::array::<T>(self.cap).unwrap();
            let tmp_old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(tmp_old_ptr, tmp_old_layout, new_layout.size()) }
        };

        let new_ptr_optional = ptr::NonNull::new(new_ptr as *mut T);
        self.ptr = if new_ptr_optional.is_some() {
            new_ptr_optional.unwrap()
        } else {
            alloc::handle_alloc_error(new_layout);
        };
        self.cap = new_cap;
    }
}

//
impl<T> ops::Drop for Vector<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while self.len() != 0 {
                self.pop();
            }

            let layout = alloc::Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

//
impl<T> ops::Deref for Vector<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        return unsafe { std::slice::from_raw_parts(self.begin(), self.len()) };
    }
}

//
impl<T> ops::DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return unsafe { std::slice::from_raw_parts_mut(self.begin(), self.len()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vector_init_test() {
        let vector = Vector::<i32>::new();
        assert!(vector.len() == 0);
        assert!(vector.cap == 0);
    }
}
