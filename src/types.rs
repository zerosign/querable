//!
//! Queryable trait for data structure.
//!
//! `Queryable` trait are being implemented in generic manner to parse
//! both `query_array` and `query_dict`. So, most of the implementor for
//! the data structure only need to implement which type of Self ~ QueryKind.
//!
use crate::{
    error::{Error, IndexError, KeyError},
    kind::QueryKind,
};

/// Tokenizer trait.
///
/// This trait should be implemented if you need to have custom
/// tokenizer for parsing array index & dictionary index.
///
/// On how you might want to implemented it, you could see
/// [SlashTokenizer](crate::default::SlashTokenizer) or
/// [DefaultTokenizer](crate::default::DefaultTokenizer)
///
pub trait Tokenizer {
    /// Parse key passed when [Queryable::query_kind](Queryable::query_kind)
    /// returns [QueryKind::Array](QueryKind::Array).
    ///
    fn index_parse(key: &str) -> Result<usize, IndexError>;

    /// Tokenizing path steps.
    ///
    fn dict_parse(key: &str) -> Result<Vec<&str>, KeyError>;
}

/// Queryable
///
pub trait Queryable
where
    Self: Sized,
{
    fn query<T>(&self, path: &str) -> Result<Self, Error>
    where
        T: Tokenizer,
    {
        let tokens = T::dict_parse(path)?;
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

    ///
    /// Identify `Self` as either one of [QueryKind](QueryKind) value.
    ///
    /// Since traversal only happens in data structure like dictionary type
    /// and array type, other that mostly are literal (leaf).
    ///
    fn query_kind(&self) -> Option<QueryKind>;

    ///
    /// Querying based on key `str` on `Self`.
    ///
    /// This method need to be implemented in case `Self` supports
    /// querying by path/key `&str`.
    ///
    fn query_dict(&self, path: &str) -> Result<Self, Error>;

    ///
    /// Querying based on index on `Self`.
    ///
    /// This method need to be implemented in case of `Self` supports
    /// querying by index `usize`.
    ///
    fn query_array(&self, idx: usize) -> Result<Self, Error>;
}
