//!
//! Querable library.
//!
//! Querable give user a choice for implementing queryable data structure
//! by giving some customization on how datastructure can be traversed.
//!
//! Currently, it give a user a freedom for defining how to :
//! - resolve an array query by defining `Queryable::kind`, `Queryable::query_dict` and `Queryable::query_array`
//! - how to parse or identify an index or key resolution by implementing custom `Tokenizer`
//!
//! There is default `Tokenizer` defined in this crates at `crate::default::DefaultTokenizer`
//! that uses `[_]` for array index and `path.*` as dictionary resolutions.
//!
#![deny(intra_doc_link_resolution_failure)]

use std::borrow::Cow;

pub mod default;
pub mod error;
pub mod kind;
pub mod types;

use error::Error;
use types::{Queryable, Tokenizer};

///
/// The entrypoint function for doing a lookup over data structure.
///
/// You need to specify `T` for [Tokenizer](Tokenizer) when calling the function.
///
/// example :
/// ```
/// // lookup<_, _, DefaultTokenizer>(value, "[0]");
/// ```
///
pub fn lookup<'a, V: 'a, Q, T>(v: &V, query: Q) -> Result<V, Error>
where
    Q: Into<Cow<'a, str>>,
    V: Queryable,
    T: Tokenizer,
{
    v.query::<T>(&query.into())
}

#[cfg(test)]
mod tests {

    extern crate env_logger;
    extern crate log;

    use super::{
        default::{DefaultTokenizer, SlashTokenizer},
        error::Error,
        kind::QueryKind,
        lookup,
        types::Queryable,
    };

    use std::collections::HashMap;

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
        pub fn string<V>(v: V) -> Value
        where
            V: Into<String>,
        {
            Value::Literal(Literal::String(v.into()))
        }

        #[inline]
        pub fn dict() -> Value {
            Value::Dictionary(HashMap::new())
        }

        #[inline]
        pub fn bool<V>(v: V) -> Value
        where
            V: Into<bool>,
        {
            Value::Literal(Literal::Bool(v.into()))
        }
    }

    macro_rules! value_conv {
        ($($conv:path => [$($src:ty),*]),*) => {
            $($(impl From<$src> for Value {

                #[inline]
                fn from(v: $src) -> Self {
                    $conv(v)
                }
            })*)*
        }
    }

    value_conv!(
        Value::integer => [u8, u16, u32, i8, i16, i32, i64],
        Value::double  => [f32, f64],
        Value::string  => [String, &'static str],
        Value::bool    => [bool]
    );

    // array!["test", 1, 2 "test"]
    macro_rules! array {
        [] => (Value::Array(Vec::<Value>::new()));
        [$($val:expr),*] => (Value::Array(<[_]>::into_vec(Box::new([$(Value::from($val)),*]))));
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
    //
    // copied from https://github.com/bluss/maplit/blob/master/src/lib.rs#L46-L61
    macro_rules! dict {
        (@single $($x:tt)*) => (());
        (@count $($rest:expr),*) => (<[()]>::len(&[$(dict!(@single $rest)),*]));

        ($($key:expr => $value:expr,)+) => { dict!($(String::from($key) => Value::from($value)),+) };
        ($($key:expr => $value:expr),*) => {
            {
                let _cap = dict!(@count $($key),*);
                let mut _map = ::std::collections::HashMap::with_capacity(_cap);
                $(
                    let _ = _map.insert(String::from($key), Value::from($value));
                )*
                Value::Dictionary(_map)
            }
        };
    }

    #[test]
    fn test_macro_rule_empty_dict() {
        assert_eq!(dict! {}, Value::dict());
    }

    #[test]
    fn test_macro_rule_literal_dict() {
        let sample = dict! {
            "test" => dict! {
                "hello" => array!["world"],
            }
        };

        let expected = {
            let mut inner = HashMap::new();
            inner.insert(String::from("test"), {
                let mut inner2 = HashMap::new();
                inner2.insert(
                    String::from("hello"),
                    Value::Array(vec![Value::string("world")]),
                );
                Value::Dictionary(inner2)
            });
            Value::Dictionary(inner)
        };

        assert_eq!(sample, expected);
    }

    #[test]
    fn test_macro_rule_empty_array() {
        assert_eq!(array![], Value::Array(vec![]));
    }

    #[test]
    fn test_macro_rule_literal_array() {
        assert_eq!(
            array![1, 2, 3.2, 4, "test"],
            Value::Array(vec![
                Value::integer(1),
                Value::integer(2),
                Value::double(3.2),
                Value::integer(4),
                Value::string("test"),
            ])
        );
    }

    #[test]
    fn test_macro_rule_complex_array() {
        assert_eq!(
            array![1, array![1, 2]],
            Value::Array(vec![
                Value::integer(1),
                Value::Array(vec![Value::integer(1), Value::integer(2),])
            ])
        );
    }

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
                    .cloned()
                    .ok_or_else(|| Error::KeyNotExist(String::from(path))),
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
                Value::Array(d) => d.get(idx).cloned().ok_or(Error::IndexNotExist(idx)),
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

    #[test]
    fn test_lookup_value_dict_slash_tokenizer() {
        let data = array![dict! {
            "id" => 12,
            "child" => 2
        }];

        let found = lookup::<_, _, SlashTokenizer>(&data, "/0/id");

        println!("{:?}", found);

        assert!(found.is_ok());
    }
}
