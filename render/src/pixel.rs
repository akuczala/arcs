/// A dimension in pixels.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct px(pub i32);

impl px {
    pub const fn new(pixel: i32) -> Self { px(pixel) }
}