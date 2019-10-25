use crate::kind::QueryKind;
use std::{convert, error::Error as StdError, num::ParseIntError};

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
    IntError(ParseIntError),
    ParseError(String),
    // TODO: @zerosign, maybe use StdError ?
    CustomError(String),
}

#[derive(Debug, PartialEq)]
pub enum KeyError {
    ParseError(String),
    EmptyKey,
    // TODO: @zerosign, maybe use StdError ?
    CustomError(String),
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
