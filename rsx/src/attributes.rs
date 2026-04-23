//! Attribute handling for HTML elements.

use std::borrow::Cow;

pub trait EscapeAttribute {
    fn escape_attribute(&self) -> Cow<'_, str>;
}

impl EscapeAttribute for &str {
    fn escape_attribute(&self) -> Cow<'_, str> {
        html_escape::encode_double_quoted_attribute(self)
    }
}

impl EscapeAttribute for str {
    fn escape_attribute(&self) -> Cow<'_, str> {
        html_escape::encode_double_quoted_attribute(self)
    }
}

impl EscapeAttribute for String {
    fn escape_attribute(&self) -> Cow<'_, str> {
        html_escape::encode_double_quoted_attribute(self)
    }
}

impl EscapeAttribute for &String {
    fn escape_attribute(&self) -> Cow<'_, str> {
        html_escape::encode_double_quoted_attribute(self)
    }
}

macro_rules! impl_escape_attribute_literal {
    ($($t:ty),*) => {
        $(
            impl EscapeAttribute for $t {
                fn escape_attribute(&self) -> Cow<'_, str> {
                    Cow::Owned(self.to_string())
                }
            }
        )*
    };
}

impl_escape_attribute_literal!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

impl EscapeAttribute for bool {
    fn escape_attribute(&self) -> Cow<'_, str> {
        Cow::Owned(self.to_string())
    }
}
