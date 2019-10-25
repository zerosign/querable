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
    fn index_parse(key: &str) -> Result<usize, IndexError> {
        if key.starts_with('[') && key.ends_with(']') && key.len() > 2 {
            let index = &key[1..key.len() - 1];
            index.parse::<usize>().map_err(IndexError::ParseError)
        } else {
            Err(IndexError::EmptyIndex)
        }
    }

    #[inline]
    fn dict_parse(key: &str) -> Result<Vec<&str>, KeyError> {
        let r = key.splitn(2, '.').collect::<Vec<_>>();

        if r.is_empty() {
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
        key.parse::<usize>().map_err(IndexError::ParseError)
    }

    fn dict_parse(key: &str) -> Result<Vec<&str>, KeyError> {
        let key = key.trim();
        if key.is_empty() {
            Err(KeyError::EmptyKey)
        } else if !key.starts_with('/') {
            // key should always prefixed with slash
            Err(KeyError::ParseError)
        } else {
            let r = key[1..key.len()].splitn(2, '/').collect::<Vec<_>>();
            // r will always have at least size of 1
            // since key is not empty
            assert!(r.len() >= 1);
            // check whether first index value are not empty
            if r[0].is_empty() {
                Err(KeyError::EmptyKey)
            } else {
                Ok(r)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DefaultTokenizer, SlashTokenizer};
    use crate::{
        error::{Error, IndexError, KeyError},
        types::Tokenizer,
    };

    #[test]
    fn test_slash_tokenizer_empty_keys() {
        assert_eq!(SlashTokenizer::dict_parse(""), Err(KeyError::EmptyKey));
    }

    #[test]
    fn test_slash_tokenizer_wrong_paths() {
        assert_eq!(
            SlashTokenizer::dict_parse("test."),
            Err(KeyError::ParseError)
        );
    }

    #[test]
    fn test_slash_tokenizer_empty_first_key() {
        assert_eq!(SlashTokenizer::dict_parse("//"), Err(KeyError::EmptyKey));
    }
}
