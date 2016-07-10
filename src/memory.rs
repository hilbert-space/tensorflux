use std::convert::AsRef;
use std::mem;

pub struct Memory<T> {
    data: Vec<T>,
    owned: bool,
}

impl<T> Memory<T> {
    #[inline]
    pub fn new(data: Vec<T>) -> Self {
        Memory { data: data, owned: true }
    }

    #[inline]
    pub fn from_raw_parts(pointer: *mut T, length: usize) -> Self {
        let data = unsafe { Vec::from_raw_parts(pointer, length, length) };
        Memory { data: data, owned: false }
    }

    pub fn empty(&mut self) -> Vec<T> where T: Clone {
        if self.owned {
            mem::replace(&mut self.data, vec![])
        } else {
            let data = self.data.clone();
            mem::forget(mem::replace(&mut self.data, vec![]));
            self.owned = true;
            data
        }
    }
}

deref!(Memory::data<T>);

impl<T> AsRef<[T]> for Memory<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

impl<T> Drop for Memory<T> {
    #[inline]
    fn drop(&mut self) {
        if !self.owned {
            mem::forget(mem::replace(&mut self.data, vec![]));
        }
    }
}

impl<T> Into<Vec<T>> for Memory<T> where T: Clone {
    #[inline]
    fn into(mut self) -> Vec<T> {
        self.empty()
    }
}
