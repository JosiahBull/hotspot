//! Internal representation of the hotspot.

mod private {
    pub trait Sealed {}
}

/// This type dictates how the hotspot is represented, as a percentage of the overall image size,
/// or as absolute pixel values.
///
/// This trait is sealed and cannot be implemented by external crates.
pub trait InternalRepr: private::Sealed {}

/// The hotspot is represented as absolute pixel values.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PixelRepr;
impl private::Sealed for PixelRepr {}
impl InternalRepr for PixelRepr {}

/// The hotspot is represented as a percentage of the overall image size.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PercentageRepr;
impl private::Sealed for PercentageRepr {}
impl InternalRepr for PercentageRepr {}
