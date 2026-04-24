//! HTML element definitions and utilities.

pub struct Elements;

impl Elements {
    pub const VOID_ELEMENTS: &'static [&'static str] = &[
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
    ];

    #[inline]
    pub fn is_void_element(name: &str) -> bool {
        Self::VOID_ELEMENTS.contains(&name)
    }

    #[inline]
    pub fn is_valid_element_name(_name: &str) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_void_elements() {
        assert!(Elements::is_void_element("br"));
        assert!(Elements::is_void_element("input"));
        assert!(Elements::is_void_element("img"));
        assert!(Elements::is_void_element("hr"));
    }

    #[test]
    fn test_non_void_elements() {
        assert!(!Elements::is_void_element("div"));
        assert!(!Elements::is_void_element("span"));
        assert!(!Elements::is_void_element("button"));
    }
}
