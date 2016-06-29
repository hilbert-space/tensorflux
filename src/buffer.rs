use std::convert::AsRef;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use Result;

/// A buffer.
pub struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    /// Load a buffer.
    pub fn load<T: AsRef<Path>>(path: T) -> Result<Self> {
        let mut data = vec![];
        let mut file = ok!(File::open(path));
        ok!(file.read_to_end(&mut data));
        Ok(Buffer { data: data })
    }
}

impl AsRef<[u8]> for Buffer {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}
