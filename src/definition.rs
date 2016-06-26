use std::convert::AsRef;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use Result;

/// A definition.
pub struct Definition {
    data: Vec<u8>,
}

impl Definition {
    /// Load a definition.
    pub fn load<T: AsRef<Path>>(path: T) -> Result<Self> {
        let mut data = vec![];
        let mut file = ok!(File::open(path));
        ok!(file.read_to_end(&mut data));
        Ok(Definition { data: data })
    }
}

impl AsRef<[u8]> for Definition {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}
