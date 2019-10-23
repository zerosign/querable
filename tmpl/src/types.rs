use tmpl_value::types::Value;

impl Queryable for Value {
    #[inline]
    fn query_kind(&self) -> Option<QueryKind> {
        match self {
            Value::Literal(_) => None,
            Value::Array(_) => Some(QueryKind::Array),
            Value::Dictionary(_) => Some(QueryKind::Dictionary),
        }
    }

    fn query_dict(&self, path: &str) -> Result<Self, Error> {
        match self {
            Value::Dictionary(d) => d
                .get(path)
                .map(|v| v.clone())
                .ok_or(Error::KeyNotExist(String::from(path))),
            Value::Array(_) => Err(Error::TypeError(
                String::from(path),
                QueryKind::Array,
                QueryKind::Dictionary,
            )),
            _ => Err(Error::UnknownType(String::from(path))),
        }
    }

    fn query_array(&self, idx: usize) -> Result<Self, Error> {
        match self {
            Value::Array(d) => d
                .get(idx)
                .map(|v| v.clone())
                .ok_or(Error::IndexNotExist(idx)),
            Value::Dictionary(_) => Err(Error::TypeError(
                format!("[{}]", idx),
                QueryKind::Dictionary,
                QueryKind::Array,
            )),
            _ => Err(Error::UnknownType(format!("[{}]", idx))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        querable_core::{
            default::{DefaultTokenizer, SlashTokenizer},
            error::Error,
        },
        tmpl_value::types::{array, dict, Value},
    };

    #[test]
    fn test_lookup_simple_array() {
        let sample = array!["Hello world"];

        let found = lookup::<_, _, DefaultTokenizer>(&sample, "[0]");
        assert_eq!(found, Ok(Value::string("Hello world")));

        let found = lookup::<_, _, SlashTokenizer>(&sample, "/0");
        assert_eq!(found, Ok(Value::string("Hello world")));
    }

    #[test]
    fn test_lookup_complex_array() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = array![array!["Hello world"]];

        let found = lookup::<_, _, DefaultTokenizer>(&sample, "[0].[0]");

        assert_eq!(found, Ok(Value::string("Hello world")));
    }

    #[test]
    fn test_lookup_index_not_exists_array() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = array![array!["test"]];

        let found = lookup::<_, _, DefaultTokenizer>(&sample, "[1]");

        assert!(found.is_err());

        assert_eq!(found, Err(Error::IndexNotExist(1)),);
    }
}
