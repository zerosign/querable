extern crate criterion;
extern crate querable;

use std::collections::HashMap;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use querable::{
    default::{DefaultTokenizer, SlashTokenizer},
    error::Error,
    kind::QueryKind,
    types::Queryable,
};

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
    pub fn list() -> Value {
        Value::Array(vec![])
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

pub fn querable_lookup(c: &mut Criterion) {
    let data = array![
        dict! {
            "id" => 12,
            "child" => dict! {
                "id" => 20,
                "child" => dict! {
                    "child" => dict! {
                        "id" => 20,
                        "child" => dict! {
                            "child" => dict! {
                                "id" => 20,
                                "child" => 10,
                            },
                        },
                    },
                },
            },
        },
        array![array![array![array![array![array![array![array![
            array![array![1]]
        ]]]]]]]],
        dict! {
            "id" => 12,
            "child" => dict! {
                "id" => 20,
                "child" => dict! {
                    "child" => dict! {
                        "id" => 20,
                        "child" => dict! {
                            "child" => dict! {
                                "id" => 20,
                                "child" => dict! {
                                    "id" => 20,
                                    "child" => dict! {
                                        "child" => dict! {
                                            "id" => 20,
                                            "child" => dict! {
                                                "child" => dict! {
                                                    "id" => 20,
                                                    "child" => dict! {
                                                        "id" => 20,
                                                        "child" => dict! {
                                                            "child" => dict! {
                                                                "id" => 20,
                                                                "child" => dict! {
                                                                    "child" => dict! {
                                                                        "id" => 20,
                                                                        "child" => 1,
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                            },
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
            },
        }
    ];

    let queries = vec![
        "[0]",
        "[0].id",
        "[0].child.id",
        "[0].child.child.child.child.child.child",
        "[1].[0].[0].[0].[0].[0].[0]",
        "[2].child.child.child.child.child.child.child.child.child.child.child",
    ];

    for query in queries {
        c.bench_with_input(
            BenchmarkId::new(
                "lookup_default_tokenizer",
                format!("{}-{}", "sample_1", query),
            ),
            &query,
            |b, &q| {
                b.iter(|| assert!(querable::lookup::<_, _, DefaultTokenizer>(&data, q).is_ok()))
            },
        );
    }

    let queries = vec![
        "/0",
        "/0/id",
        "/0/child/id",
        "/0/child/child/child/child/child/child",
        "/1/0/0/0/0/0/0",
        "/2/child/child/child/child/child/child/child/child/child/child/child",
    ];

    for query in queries {
        c.bench_with_input(
            BenchmarkId::new(
                "lookup_slash_tokenizer",
                format!("{}-{}", "sample_1", query),
            ),
            &query,
            |b, &q| b.iter(|| assert!(querable::lookup::<_, _, SlashTokenizer>(&data, q).is_ok())),
        );
    }
}

criterion_group!(benches, querable_lookup);
criterion_main!(benches);
