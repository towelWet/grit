use super::Hash;
use std::fmt;

impl fmt::Debug for Hash {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        write!(output, "Hash({})", self.short())
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        write!(output, "{}", self.hex())
    }
}
