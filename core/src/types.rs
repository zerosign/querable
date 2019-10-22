//!
//! Queryable trait for data structure.
//!
//! `Queryable` trait are being implemented in generic manner to parse
//! both `query_array` and `query_dict`. So, most of the implementor for
//! the data structure only need to implement which type of Self ~ QueryKind.
//!
//!
use crate::{
    error::{Error, IndexError},
    kind::QueryKind,
};

fn index_parse(key: &str, block: [char; 2]) -> Result<usize, IndexError> {
    if key.starts_with(block[0]) && key.ends_with(block[1]) && key.len() > 2 {
        let index = &key[1..key.len() - 1];

        index.parse::<usize>().map_err(IndexError::ParseError)
    } else {
        Err(IndexError::EmptyIndex)
    }
}

const ARRAY_BLOCK: [char; 2] = ['[', ']'];
const DICT_SEP: char = '.';

pub trait Queryable
where
    Self: Sized,
{
    fn query(&self, path: &str) -> Result<Self, Error> {
        let tokens = path.splitn(2, DICT_SEP).collect::<Vec<_>>();
        let slices = tokens.as_slice();

        match self.query_kind() {
            Some(QueryKind::Dictionary) => match *slices {
                [key, next] => self
                    .query_dict(key)
                    .and_then(move |child| child.query(next)),
                // base case
                [key] => self.query_dict(key),
                _ => Err(Error::EmptyPath(QueryKind::Dictionary)),
            },
            Some(QueryKind::Array) => match *slices {
                [key, next] => {
                    let index = index_parse(key, ARRAY_BLOCK)?;
                    match self.query_array(index) {
                        Ok(child) => child.query(next),
                        _ => Err(Error::IndexNotExist(index)),
                    }
                }
                // base case
                [key] => {
                    let index = index_parse(key, ARRAY_BLOCK)?;
                    self.query_array(index)
                }
                _ => Err(Error::EmptyPath(QueryKind::Array)),
            },
            _ => Err(Error::UnknownType(String::from(path))),
        }
    }

    fn query_kind(&self) -> Option<QueryKind>;
    fn query_dict(&self, path: &str) -> Result<Self, Error>;
    fn query_array(&self, idx: usize) -> Result<Self, Error>;
}
