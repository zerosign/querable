use crate::{error::IndexError, types::Tokenizer};

const ARRAY_BLOCK: [char; 2] = ['[', ']'];
const DICT_SEP: char = '.';

pub struct DefaultTokenizer;

impl Tokenizer for DefaultTokenizer {
    fn index_parse(key: &str) -> Result<usize, IndexError> {
        if key.starts_with(ARRAY_BLOCK[0]) && key.ends_with(ARRAY_BLOCK[1]) && key.len() > 2 {
            let index = &key[1..key.len() - 1];

            index.parse::<usize>().map_err(IndexError::ParseError)
        } else {
            Err(IndexError::EmptyIndex)
        }
    }

    fn dict_parse(key: &str) -> Vec<&str> {
        key.splitn(2, DICT_SEP).collect::<Vec<_>>()
    }
}
