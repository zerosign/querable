use crate::{
    error::{IndexError, KeyError},
    types::{State, Tokenizer},
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
    fn dict_parse(key: &str) -> Result<State, KeyError> {
        if key.is_empty() {
            Err(KeyError::EmptyKey)
        } else {
            let size = key.len();

            match key.find('.') {
                Some(0) => Err(KeyError::EmptyKey),
                Some(idx) => {
                    let current = &key[0..idx];

                    match current.find(char::is_whitespace) {
                        Some(_) => Err(KeyError::ParseError(String::from(current))),
                        _ => {
                            let pivot = idx + 1;
                            Ok((Some(current), Some(&key[pivot..size])))
                        }
                    }
                }
                _ => Ok((Some(&key[0..size]), None)),
            }
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
    fn dict_parse(key: &str) -> Result<State, KeyError> {
        if key.is_empty() {
            Err(KeyError::EmptyKey)
        } else if !key.starts_with('/') {
            // key should always prefixed with slash
            Err(KeyError::ParseError(String::from(key)))
        } else {
            let size = key.len();
            // /1/2
            // 1/2
            match key[1..size].find('/') {
                // since path is empty (case "//")
                Some(0) => Err(KeyError::EmptyKey),
                // if there is '/', then there will be next
                Some(idx) => {
                    let pivot = idx + 1;
                    let current = &key[1..pivot];
                    // check whether current have a whitespace or not
                    // key shouldn't have a whitespace
                    match current.find(char::is_whitespace) {
                        Some(_) => Err(KeyError::ParseError(String::from(current))),
                        _ => Ok((Some(current), Some(&key[pivot..size]))),
                    }
                }
                _ => Ok((Some(&key[1..size]), None)),
            }
        }
    }
}
