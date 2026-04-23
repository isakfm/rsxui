//! Render trait for converting types to HTML strings.

use std::fmt;

pub trait Render {
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

macro_rules! impl_render_for_basic_types {
    ($($t:ty)*) => ($(
        impl Render for $t {
            #[inline(always)]
            fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(self, f)
            }
        }
    )*)
}

impl_render_for_basic_types! {
    bool char f32 f64
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
    String str
}

impl<T: Render> Render for Box<T> {
    #[inline(always)]
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Render::render(&**self, f)
    }
}

impl<T: Render> Render for &T {
    #[inline(always)]
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Render::render(&**self, f)
    }
}

impl<T: Render> Render for Option<T> {
    #[inline(always)]
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(inner) = self {
            inner.render(f)
        } else {
            Ok(())
        }
    }
}

impl Render for () {
    #[inline(always)]
    fn render(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl Render for std::borrow::Cow<'_, str> {
    #[inline(always)]
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub struct FormatWrapper<T> {
    inner: T,
}

impl<T> FormatWrapper<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Render> fmt::Display for FormatWrapper<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.render(f)
    }
}
