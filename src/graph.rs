use std::fs::File;
use std::io::Read;
use std::path::Path;

use Result;

/// A graph.
pub struct Graph {
    data: Vec<u8>,
}

impl Graph {
    /// Load a graph.
    pub fn load<T: AsRef<Path>>(path: T) -> Result<Self> {
        let mut data = vec![];
        let mut file = ok!(File::open(path));
        ok!(file.read_to_end(&mut data));
        Ok(Graph { data: data })
    }
}

impl From<Graph> for Vec<u8> {
    #[inline]
    fn from(graph: Graph) -> Vec<u8> {
        graph.data
    }
}
