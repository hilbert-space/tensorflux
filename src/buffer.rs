use ffi::TF_Buffer;
use libc::size_t;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::Path;

use Result;
use memory::Memory;

/// A buffer.
pub struct Buffer {
    memory: Memory<u8>,
    raw: *mut TF_Buffer,
}

impl Buffer {
    /// Create a buffer.
    pub fn new(data: Vec<u8>) -> Result<Self> {
        let raw = ffi!(TF_NewBuffer());
        unsafe {
            (*raw).data = data.as_ptr() as *mut _;
            (*raw).length = data.len() as size_t;
            (*raw).data_deallocator = None;
        }
        Ok(Buffer { memory: Memory::new(data), raw: raw })
    }

    /// Load a buffer.
    pub fn load<T>(path: T) -> Result<Self> where T: AsRef<Path> {
        let mut data = vec![];
        let mut file = ok!(File::open(path));
        ok!(file.read_to_end(&mut data));
        Buffer::new(data)
    }

    #[doc(hidden)]
    #[inline]
    pub fn as_raw(&self) -> *mut TF_Buffer {
        self.raw
    }

    #[doc(hidden)]
    pub fn reset(&mut self) {
        let (pointer, length) = unsafe { ((*self.raw).data, (*self.raw).length) };
        let mut memory = Memory::from_raw(pointer as *mut _, length as usize);
        mem::swap(&mut self.memory, &mut memory);
    }
}

memory!(Buffer<u8>);

impl Drop for Buffer {
    #[inline]
    fn drop(&mut self) {
        ffi!(TF_DeleteBuffer(self.raw));
    }
}
