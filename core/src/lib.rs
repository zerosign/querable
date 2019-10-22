use std::borrow::Cow;

pub mod error;
pub mod kind;
pub mod types;

use error::Error;
use kind::QueryKind;
use types::Queryable;

pub fn lookup<'a, V: 'a, Q>(v: V, query: Q) -> Result<V, Error>
where
    Q: Into<Cow<'a, str>>,
    V: Queryable,
{
    v.query(&query.into())
}

#[cfg(test)]
mod tests {

    #![feature(slice_patterns)]
    use super::{
        error::{Error, IndexError},
        kind::QueryKind,
        types::Queryable,
    };

    use std::{borrow::Cow, collections::HashMap};

    #[derive(Debug, Clone, PartialEq)]
    pub enum Number {
        Integer(i64),
        Double(f64),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Literal {
        Number(Number),
        String(String),
        Bool(bool),
        None,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Value {
        Literal(Literal),
        Dictionary(HashMap<String, Value>),
        Array(Vec<Value>),
    }

    impl Value {
        #[inline]
        pub fn integer<V>(v: V) -> Value
        where
            V: Into<i64>,
        {
            Value::Literal(Literal::Number(Number::Integer(v.into())))
        }

        #[inline]
        pub fn double<V>(v: V) -> Value
        where
            V: Into<f64>,
        {
            Value::Literal(Literal::Number(Number::Double(v.into())))
        }

        #[inline]
        pub fn dict() -> Value {
            Value::Dictionary(HashMap::new())
        }

        #[inline]
        pub fn list() -> Value {
            Value::Array(vec![])
        }
    }

    //
    // dict! {
    //   "test" => dict! {
    //      "data" => array!("test", 0),
    //      "another" => dict! {
    //         "key" => "value",
    //      },
    //   },
    // }
    //
    //
    // macro_rules! dict { }
    // macro_rules! array { }

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

    #[test]
    fn test_lookup_dictionary() {}
}
