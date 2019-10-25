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

///
/// Type that represents the return state of [Tokenizer::dict_parse](Tokenizer::dict_parse).
///
/// (current, next).
///
pub type State<'a> = (Option<&'a str>, Option<&'a str>);

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
    fn dict_parse(key: &str) -> Result<State, KeyError>;
}

/// Queryable trait.
///
/// The main trait that need to be implemented by data structure.
/// This trait assume that `Self` are sum types or linear? type.
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

        match self.query_kind() {
            Some(QueryKind::Dictionary) => match tokens {
                (Some(key), Some(next)) => self
                    .query_dict(key)
                    .and_then(move |child| child.query::<T>(next)),
                // base case
                (Some(key), None) => self.query_dict(key),
                _ => Err(Error::EmptyPath(QueryKind::Dictionary)),
            },
            Some(QueryKind::Array) => match tokens {
                (Some(key), Some(next)) => {
                    let index = T::index_parse(key)?;
                    match self.query_array(index) {
                        Ok(child) => child.query::<T>(next),
                        _ => Err(Error::IndexNotExist(index)),
                    }
                }
                // base case
                (Some(key), None) => {
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
