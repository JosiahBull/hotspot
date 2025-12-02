//! Internal representation of the hotspot.

mod private {
    pub trait Sealed {}
}

/// This type dictates how the hotspot is represented, as a percentage of the overall image size,
/// or as absolute pixel values.
///
/// This trait is sealed and cannot be implemented by external crates.
pub trait InternalRepr: private::Sealed {}

/// Trait for providing serde-related metadata for each representation type.
///
/// This trait is sealed and cannot be implemented by external crates.
#[cfg(feature = "serde")]
pub trait HotspotRepr: InternalRepr {
    /// The struct name used during serialization/deserialization.
    const STRUCT_NAME: &'static str;
}

/// The hotspot is represented as absolute pixel values.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PixelRepr;
impl private::Sealed for PixelRepr {}
impl InternalRepr for PixelRepr {}

#[cfg(feature = "serde")]
impl HotspotRepr for PixelRepr {
    const STRUCT_NAME: &'static str = "HotspotPx";
}

/// The hotspot is represented as a percentage of the overall image size.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PercentageRepr;
impl private::Sealed for PercentageRepr {}
impl InternalRepr for PercentageRepr {}

#[cfg(feature = "serde")]
impl HotspotRepr for PercentageRepr {
    const STRUCT_NAME: &'static str = "HotspotRel";
}
