//! HTML escaping utilities.

use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreEscaped(pub String);

impl std::fmt::Display for PreEscaped {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PreEscaped {
    pub fn into_string(self) -> String {
        self.0
    }
}

#[inline]
pub fn escape_content(s: &str) -> Cow<'_, str> {
    html_escape::encode_text(s)
}

pub fn escape_content_opt(s: &str) -> Cow<'_, str> {
    html_escape::encode_text(s)
}

#[inline]
pub fn escape_attribute(s: &str) -> Cow<'_, str> {
    html_escape::encode_unquoted_attribute(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_content() {
        assert_eq!(escape_content("<div>"), "&lt;div&gt;");
        assert_eq!(escape_content("A & B"), "A &amp; B");
        assert_eq!(escape_content("\"quotes\""), "\"quotes\"");
    }

    #[test]
    fn test_escape_attribute() {
        assert_eq!(escape_attribute("a<b"), "a&lt;b");
        assert_eq!(escape_attribute("a\"b"), "a&quot;b");
        assert_eq!(escape_attribute("a>b"), "a&gt;b");
    }

    #[test]
    fn test_pre_escaped() {
        let html = "<div>hello</div>";
        let pre_escaped = PreEscaped(html.to_string());
        assert_eq!(format!("{}", pre_escaped), html);
    }
}
