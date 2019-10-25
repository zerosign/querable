use crate::{
    error::{IndexError, KeyError},
    types::Tokenizer,
};

///
/// [DefaultTokenizer](DefaultTokenizer) have a format query likes :
/// ```
/// // [0].test.[1]
/// // test.test.[1]
/// ```
pub struct DefaultTokenizer;

impl Tokenizer for DefaultTokenizer {
    /// Parse index array.
    ///
    /// - should starts and end with '[' and ']'. No space allowed.
    ///
    /// ```rust
    /// use querable::{types::Tokenizer, default::DefaultTokenizer, error::{IndexError}};
    ///
    /// assert_eq!(DefaultTokenizer::index_parse("[]"), Err(IndexError::ParseError(String::from("[]"))));
    /// ```
    ///
    /// - index should be an integer, specificially, in the range of usize.
    ///
    /// ```rust
    /// use querable::{types::Tokenizer, default::DefaultTokenizer};
    ///
    /// assert!(DefaultTokenizer::index_parse("[x]").is_err());
    /// ```
    ///
    fn index_parse(key: &str) -> Result<usize, IndexError> {
        if key.starts_with('[') && key.ends_with(']') && key.len() > 2 {
            let index = &key[1..key.len() - 1];
            index.parse::<usize>().map_err(IndexError::IntError)
        } else {
            Err(IndexError::ParseError(String::from(key)))
        }
    }

    /// Parse key/path index.
    ///
    /// - shouldn't be an empty string or being prefixed & suffixed with empty string.
    ///
    /// ```rust
    /// use querable::{types::Tokenizer, default::DefaultTokenizer, error::{KeyError}};
    ///
    /// assert_eq!(DefaultTokenizer::dict_parse("   .test"), Err(KeyError::EmptyKey));
    /// assert_eq!(DefaultTokenizer::dict_parse(""), Err(KeyError::EmptyKey));
    /// ```
    ///
    #[inline]
    fn dict_parse(key: &str) -> Result<Vec<&str>, KeyError> {
        let r = key.splitn(2, '.').collect::<Vec<_>>();

        if r.is_empty() {
            Err(KeyError::EmptyKey)
        } else if r.len() > 0 && r[0].trim().is_empty() {
            Err(KeyError::EmptyKey)
        } else {
            Ok(r)
        }
    }
}

///
/// [SlashTokenizer](SlashTokenizer) have a format query likes :
/// ```
/// // /0/1/2/3
/// // /test/test/1/test/test/2
/// ```
pub struct SlashTokenizer;

impl Tokenizer for SlashTokenizer {
    #[inline]
    fn index_parse(key: &str) -> Result<usize, IndexError> {
        key.parse::<usize>().map_err(IndexError::IntError)
    }

    /// Parse dict key/path query.
    ///
    /// The query should :
    ///
    /// - starts with '/'
    ///
    /// ```rust
    /// use querable::{types::Tokenizer, default::SlashTokenizer, error::{KeyError, IndexError}};
    /// assert_eq!(SlashTokenizer::dict_parse("test."), Err(KeyError::ParseError(String::from("test."))));
    /// ```
    ///
    /// - have no empty path
    ///
    /// ```rust
    /// use querable::{types::Tokenizer, default::SlashTokenizer, error::{KeyError, IndexError}};
    /// assert_eq!(SlashTokenizer::dict_parse("//"), Err(KeyError::EmptyKey));
    /// ```
    ///
    /// - not an empty string
    ///
    /// ```rust
    /// use querable::{types::Tokenizer, default::SlashTokenizer, error::{KeyError, IndexError}};
    /// assert_eq!(SlashTokenizer::dict_parse(""), Err(KeyError::EmptyKey))
    /// ```
    fn dict_parse(key: &str) -> Result<Vec<&str>, KeyError> {
        if key.is_empty() {
            Err(KeyError::EmptyKey)
        } else if !key.starts_with('/') {
            // key should always prefixed with slash
            Err(KeyError::ParseError(String::from(key)))
        } else {
            let r = key[1..key.len()].splitn(2, '/').collect::<Vec<_>>();
            // r will always have at least size of 1
            // since key is not empty
            assert!(r.len() >= 1);
            // check whether first index value are not empty
            if r[0].trim().is_empty() {
                Err(KeyError::EmptyKey)
            } else {
                Ok(r)
            }
        }
    }
}
