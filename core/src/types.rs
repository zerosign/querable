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

pub trait Tokenizer {
    fn index_parse(key: &str) -> Result<usize, IndexError>;
    fn dict_parse(key: &str) -> Vec<&str>;
}

pub trait Queryable
where
    Self: Sized,
{
    fn query<T>(&self, path: &str) -> Result<Self, Error>
    where
        T: Tokenizer,
    {
        let tokens = T::dict_parse(path);
        let slices = tokens.as_slice();

        match self.query_kind() {
            Some(QueryKind::Dictionary) => match *slices {
                [key, next] => self
                    .query_dict(key)
                    .and_then(move |child| child.query::<T>(next)),
                // base case
                [key] => self.query_dict(key),
                _ => Err(Error::EmptyPath(QueryKind::Dictionary)),
            },
            Some(QueryKind::Array) => match *slices {
                [key, next] => {
                    let index = T::index_parse(key)?;
                    match self.query_array(index) {
                        Ok(child) => child.query::<T>(next),
                        _ => Err(Error::IndexNotExist(index)),
                    }
                }
                // base case
                [key] => {
                    let index = T::index_parse(key)?;
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
