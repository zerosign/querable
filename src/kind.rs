///
/// Since traversals only supports for traversable data structure
/// like dictionary or array, other than that are mostly literals.
///
/// - [QueryKind::Array](QueryKind::Array) are being used in case
///   underlying data structure support indexing by `usize`.
///
/// - [QueryKind::Dictionary](QueryKind::Dictionary) are being used in
///   case underlying data structure support fetch value by key/path `&str`.
///
#[derive(Debug, PartialEq)]
pub enum QueryKind {
    Array,
    Dictionary,
}
