use minreq;
use std::{convert, error, fmt, result};

#[derive(Debug)]
pub enum Error {
    ClientError(minreq::Error),
    InputError(String),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            ClientError(err) => write!(f, "{}", err),
            InputError(err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            ClientError(err) => Some(err),
            _ => None,
        }
    }
}

impl convert::From<minreq::Error> for Error {
    fn from(err: minreq::Error) -> Self {
        Error::ClientError(err)
    }
}
