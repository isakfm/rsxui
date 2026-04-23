//! Buffer abstraction for efficient HTML string building.

use std::fmt::Write;

pub struct Buffer {
    inner: String,
}

impl Buffer {
    #[inline]
    pub fn new() -> Self {
        Self { inner: String::new() }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self { inner: String::with_capacity(capacity) }
    }

    #[inline]
    pub fn push(&mut self, s: impl AsRef<str>) {
        self.inner.push_str(s.as_ref());
    }

    #[inline]
    pub fn push_str(&mut self, s: &str) {
        self.inner.push_str(s);
    }

    #[inline]
    pub fn push_char(&mut self, c: char) {
        self.inner.push(c);
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.inner
    }

    pub fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        self.inner.write_fmt(args)
    }

    pub fn push_element_start(&mut self, name: &str) {
        self.inner.push('<');
        self.inner.push_str(name);
    }

    pub fn push_attribute(&mut self, name: &str, value: &str) {
        self.inner.push(' ');
        self.inner.push_str(name);
        self.inner.push_str("=\"");
        self.inner.push_str(&html_escape::encode_unquoted_attribute(value));
        self.inner.push('"');
    }

    pub fn push_boolean_attribute(&mut self, name: &str, value: bool) {
        if value {
            self.inner.push(' ');
            self.inner.push_str(name);
        }
    }

    pub fn push_element_end_open(&mut self) {
        self.inner.push('>');
    }

    pub fn push_element_end_close(&mut self, name: &str) {
        self.inner.push_str("</");
        self.inner.push_str(name);
        self.inner.push('>');
    }

    pub fn push_self_closing(&mut self) {
        self.inner.push_str(" />");
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl AsRef<str> for Buffer {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl From<Buffer> for String {
    fn from(buffer: Buffer) -> Self {
        buffer.into_string()
    }
}
