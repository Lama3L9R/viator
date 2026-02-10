

///
/// Deduce a enum into a specific variant.
/// Used when you are sure about that enum is the correct variant.
/// Will panic if variant mismatch.
///
/// Example:
/// ```
/// use viator_utils::deduce_enum;
///
/// let opt: Option<String> = Some("Some text".to_string());
///
/// let str: String = deduce_enum!(opt, Option::Some); // Works fine
/// let str: () = deduce_enum!(opt, Option::None); // this won't compile and will panic
/// ```
///
/// Currently, does not support deducing complex structs or tuples.
/// Only tuple1 is supported.
///
#[macro_export]
macro_rules! deduce_enum {
    ($var:expr, $enum_name:path) => {
        if let $enum_name(body) = $var {
            body
        } else {
            unreachable!()
        }
    };
}