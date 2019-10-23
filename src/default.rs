use crate::{error::IndexError, types::Tokenizer};

///
/// [DefaultTokenizer](DefaultTokenizer) have a format query likes :
/// ```
/// // [0].test.[1]
/// // test.test.[1]
/// ```
pub struct DefaultTokenizer;

impl Tokenizer for DefaultTokenizer {
    fn index_parse(key: &str) -> Result<usize, IndexError> {
        if key.starts_with('[') && key.ends_with(']') && key.len() > 2 {
            let index = &key[1..key.len() - 1];

            index.parse::<usize>().map_err(IndexError::ParseError)
        } else {
            Err(IndexError::EmptyIndex)
        }
    }

    #[inline]
    fn dict_parse(key: &str) -> Vec<&str> {
        key.splitn(2, '.').collect::<Vec<_>>()
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
        key.parse::<usize>().map_err(IndexError::ParseError)
    }

    fn dict_parse(key: &str) -> Vec<&str> {
        // TODO: @zerosign (checks for empty string in key)
        if !key.is_empty() {
            key[1..key.len()].splitn(2, '/').collect::<Vec<_>>()
        } else {
            Vec::with_capacity(0)
        }
    }
}
