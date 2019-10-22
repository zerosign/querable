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
    // path, expected, found
    TypeError(String, QueryKind, QueryKind),
}

#[derive(Debug, PartialEq)]
pub enum IndexError {
    ParseError(ParseIntError),
    EmptyIndex,
}

// impl<'a> convert::Into<Error<'a>> for IndexError {
//     #[inline]
//     fn into(self) -> Error<'a> {
//         Error::IndexError(self)
//     }
// }

impl convert::From<IndexError> for Error {
    #[inline]
    fn from(e: IndexError) -> Self {
        Error::IndexError(e)
    }
}
