use core::fmt;

/// An error that occurred during parsing or hashing.
#[derive(Clone, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// A data overflow error.
    HashDataOverflow,
}

// This prints better looking error messages if `unwrap` is called.
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match *self {
            HashDataOverflow => f.debug_tuple("HashDataOverflow").finish(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match *self {
            HashDataOverflow => write!(f, "Hash data length overflow."),
        }
    }
}
