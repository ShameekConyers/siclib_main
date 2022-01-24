use ::std;
use std::alloc;
use std::iter;
use std::marker;
use std::mem;
use std::ops;
use std::option::Option;
use std::ptr;

const INITIAL_ALLOC_CAPACITY: usize = 8;
pub struct RawVector<T> {
    ptr: ptr::NonNull<T>,
    capacity: usize,
    _marker: marker::PhantomData<T>,
}

unsafe impl<T: marker::Send> marker::Send for RawVector<T> {}
unsafe impl<T: marker::Sync> marker::Sync for RawVector<T> {}

impl<T> RawVector<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "TODO: implement ZST support");

        return RawVector {
            ptr: ptr::NonNull::dangling(),
            capacity: 0,
            _marker: marker::PhantomData,
        };
    }

    fn grow(&mut self) -> () {
        // get preliminary data to input into alloc
        let (new_capacity, new_layout) = if self.capacity == 0 {
            (
                INITIAL_ALLOC_CAPACITY,
                alloc::Layout::array::<T>(INITIAL_ALLOC_CAPACITY).unwrap(),
            )
        } else {
            (
                self.capacity * 2,
                alloc::Layout::array::<T>(self.capacity * 2).unwrap(),
            )
        };

        // check bounds
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        // allocate memory in the layout
        let new_ptr = if self.capacity == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = alloc::Layout::array::<T>(self.capacity).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // check error if allocation fails
        let new_ptr_optional = ptr::NonNull::new(new_ptr as *mut T);
        self.ptr = if new_ptr_optional.is_some() {
            new_ptr_optional.unwrap()
        } else {
            alloc::handle_alloc_error(new_layout)
        };
        self.capacity = new_capacity;
    }
}

impl<T> ops::Drop for RawVector<T> {
    fn drop(&mut self) -> () {
        if self.capacity != 0 {
            let layout = alloc::Layout::array::<T>(self.capacity).unwrap();
            unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout) };
            self.capacity = 0;
        }
    }
}
pub struct Vector<T> {
    buffer: RawVector<T>,
    length: usize,
}

//
impl<T> Vector<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "No zero sized types");

        return Vector {
            buffer: RawVector::<T>::new(),
            length: 0,
        };
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        // Note: equality is allowed since its valid to insert at end
        assert!(index <= self.len(), "index out of bounds.");
        if self.cap() == self.len() {
            self.grow();
        }

        unsafe {
            ptr::copy(
                self.offset(index),
                self.offset(index + 1),
                self.len() - index,
            );
            ptr::write(self.offset(index), elem);
            self.length += 1;
        };
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len(), "index out of bounds.");
        unsafe {
            self.length -= 1;
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
        if self.length == self.cap() {
            self.grow();
        }

        unsafe {
            //
            ptr::write(self.end(), elem)
        };

        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            return Option::None;
        } else {
            self.length -= 1;
            return unsafe { Option::Some(ptr::read(self.end())) };
        }
    }

    pub fn len(&self) -> usize {
        return self.length;
    }

    pub fn cap(&self) -> usize {
        return self.buffer.capacity;
    }

    unsafe fn begin(&self) -> *mut T {
        return self.buffer.ptr.as_ptr();
    }

    unsafe fn end(&self) -> *mut T {
        match self.cap() {
            0 => return self.begin(),
            _ => return self.offset(self.len()),
        };
    }

    unsafe fn offset(&self, index: usize) -> *mut T {
        match self.cap() {
            0 => return self.begin(),
            _ => return self.begin().add(index),
        };
    }

    fn grow(&mut self) {
        self.buffer.grow();
    }
}

//
impl<T> ops::Drop for Vector<T> {
    fn drop(&mut self) {
        while self.len() != 0 {
            self.pop();
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

pub struct IntoIter<T> {
    _buffer: RawVector<T>,
    begin: *const T,
    end: *const T,
}

impl<T> Vector<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        unsafe {
            // Note: We need ptr:read to unsafely move the buffer out to avoid a drop
            let _buffer = ptr::read(&self.buffer);
            let begin = self.begin();
            let end = self.end();
            let _length = self.len();

            // make sure not to drop Vector to make sure we keep buffer
            mem::forget(self);

            return IntoIter {
                _buffer,
                begin,
                end,
            };
        }
    }
}

impl<T> IntoIter<T> {
    fn _begin(&self) -> *const T {
        return self.begin;
    }

    fn _end(&self) -> *const T {
        return self.end;
    }
}

impl<T> iter::Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.begin == self.end {
            return Option::None;
        } else {
            unsafe {
                let result = ptr::read(self.begin);
                self.begin = self.begin.offset(1);
                return Option::Some(result);
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = (self.end as usize - self.begin as usize) / mem::size_of::<T>();
        return (length, Option::Some(length));
    }
}

impl<T> iter::DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.begin == self.end {
            return Option::None;
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                return Option::Some(ptr::read(self.end));
            }
        }
    }
}

impl<T> ops::Drop for IntoIter<T> {
    fn drop(&mut self) {
        // drop any remaining elements by reading them
        for _ in &mut *self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_vector_init_test() {
        let raw_vector = RawVector::<i32>::new();
        assert!(raw_vector.capacity == 0);
    }

    #[test]
    fn vector_init_test() {
        let vector = Vector::<i32>::new();
        assert!(vector.len() == 0);
        assert!(vector.cap() == 0);
    }

    #[test]
    fn vector_push_test() {
        let mut vector = Vector::<i32>::new();
        for i in 0..10 {
            vector.push(i);
        }
    }

    #[test]
    fn into_iter_test() {
        let mut vector = Vector::<i32>::new();
        for i in 0..10 {
            vector.push(i);
        }

        vector.into_iter();
    }
}
