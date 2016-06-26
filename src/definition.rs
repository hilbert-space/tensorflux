use std::fs::File;
use std::io::Read;
use std::ops::Deref;
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

impl Deref for Definition {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        &self.data
    }
}

impl From<Definition> for Vec<u8> {
    #[inline]
    fn from(definition: Definition) -> Vec<u8> {
        definition.data
    }
}

impl From<Vec<u8>> for Definition {
    #[inline]
    fn from(data: Vec<u8>) -> Definition {
        Definition { data: data }
    }
}
