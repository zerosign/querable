use std::borrow::Cow;

pub trait Queryable {
    const ARRAY_BLOCK: [char; 2];
    const DICT_SEP: char;

    type Output;
    type Error;

    fn query(&self, path: &str) -> Result<Self::Output, Self::Error>;
}

pub fn lookup<'a, S, V>(v: V, query: S) -> Result<V::Output, V::Error>
where
    S: Into<Cow<'a, str>>,
    V: Queryable,
{
    v.query(&query.into())
}
