use crate::kind::QueryKind;
use std::{convert, num::ParseIntError};

#[derive(Debug, PartialEq)]
pub enum Error {
    // is an error for dictionary key not exists
    KeyNotExist(String),
    // is an error for array index not exists or out of bound
    IndexNotExist(usize),
    EmptyPath(QueryKind),
    UnknownType(String),
    IndexError(IndexError),
    KeyError(KeyError),
    // path, expected, found
    TypeError(String, QueryKind, QueryKind),
}

#[derive(Debug, PartialEq)]
pub enum IndexError {
    ParseError(ParseIntError),
    EmptyIndex,
}

#[derive(Debug, PartialEq)]
pub enum KeyError {
    ParseError,
    EmptyKey,
}

impl convert::From<KeyError> for Error {
    #[inline]
    fn from(e: KeyError) -> Self {
        Error::KeyError(e)
    }
}

impl convert::From<IndexError> for Error {
    #[inline]
    fn from(e: IndexError) -> Self {
        Error::IndexError(e)
    }
}
