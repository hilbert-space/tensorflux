use ffi;
use libc::size_t;

use std::convert::AsRef;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::Path;

use Result;
use memory::Memory;

/// A buffer.
pub struct Buffer {
    memory: Memory<u8>,
    raw: *mut ffi::TF_Buffer,
}

impl Buffer {
    /// Create a buffer.
    #[inline]
    pub fn new() -> Result<Self> {
        Buffer::from(vec![])
    }

    /// Create a buffer from a vector.
    fn from(data: Vec<u8>) -> Result<Self> {
        let raw = ffi!(TF_NewBuffer());
        unsafe {
            (*raw).data = data.as_ptr() as *mut _;
            (*raw).length = data.len() as size_t;
            (*raw).data_deallocator = None;
        }
        Ok(Buffer { memory: Memory::new(data), raw: raw })
    }

    /// Load a buffer.
    pub fn load<T: AsRef<Path>>(path: T) -> Result<Self> {
        let mut data = vec![];
        let mut file = ok!(File::open(path));
        ok!(file.read_to_end(&mut data));
        Buffer::from(data)
    }
}

deref!(Buffer::memory<u8>);

impl AsRef<[u8]> for Buffer {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.memory
    }
}

impl Drop for Buffer {
    #[inline]
    fn drop(&mut self) {
        ffi!(TF_DeleteBuffer(self.raw));
    }
}

impl Into<Vec<u8>> for Buffer {
    #[inline]
    fn into(mut self) -> Vec<u8> {
        self.memory.empty()
    }
}

#[inline(always)]
pub fn as_raw(buffer: &Buffer) -> *mut ffi::TF_Buffer {
    buffer.raw
}

pub fn reset(buffer: &mut Buffer) {
    let (pointer, length) = unsafe { ((*buffer.raw).data, (*buffer.raw).length) };
    let mut memory = Memory::from_raw(pointer as *mut _, length as usize);
    mem::swap(&mut buffer.memory, &mut memory);
}
