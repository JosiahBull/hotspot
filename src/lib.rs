#![no_std]

mod repr;

use core::marker::PhantomData;

use repr::*;

/// Coordinate type definition.
///
/// By default, coordinates are stored as u16 values (0 to 65535), with 1
/// decimal point of precision, allowing for coordinate values from 0.0 to
/// 6553.5. If higher precision is needed, enabling the "high_precision" feature
/// which uses u32 values (0 to 4,294,967,295) with 4 decimal points of
/// precision, allowing for coordinate values from 0.0000 to 429496.7295.
///
/// If you need more than 4 decimal points of precision, consider implementing a
/// custom coordinate type.
#[cfg(not(feature = "high_precision"))]
type CoordinateValue = u16;

/// Coordinate type definition.
///
/// By default, coordinates are stored as u16 values (0 to 65535), with 1
/// decimal point of precision, allowing for coordinate values from 0.0 to
/// 6553.5. If higher precision is needed, enabling the "high_precision" feature
/// which uses u32 values (0 to 4,294,967,295) with 4 decimal points of
/// precision, allowing for coordinate values from 0.0000 to 429496.7295.
///
/// If you need more than 4 decimal points of precision, consider implementing a
/// custom coordinate type.
#[cfg(feature = "high_precision")]
type CoordinateValue = u32;

/// An internal type used for the result from multiplication between two
/// [`CoordinateValue`] to ensure no loss of precision.
#[cfg(not(feature = "high_precision"))]
#[doc(hidden)]
type InternalCalculationType = u32;

/// An internal type used for the result from multiplication between two
/// [`CoordinateValue`] to ensure no loss of precision.
#[cfg(feature = "high_precision")]
#[doc(hidden)]
type InternalCalculationType = u64;

/// A function which rounds two numbers to the closest value using integer divison.
#[inline]
const fn div_round_closest(
    dividend: InternalCalculationType,
    divider: InternalCalculationType,
) -> InternalCalculationType {
    (dividend + (divider / 2)) / divider
}

/// A const function which selects the smaller of two values.
///
/// Needed because Ord is not const-stable yet.
///
/// See: https://github.com/rust-lang/rust/issues/143874 for more information.
macro_rules! min {
    ($a:expr, $b:expr) => {{ if $a < $b { $a } else { $b } }};
}

/// A const function which selects the greater of two values.
///
/// Needed because Ord is not const-stable yet.
///
/// See: https://github.com/rust-lang/rust/issues/143874 for more information.
macro_rules! max {
    ($a:expr, $b:expr) => {{ if $a > $b { $a } else { $b } }};
}

/// A coordinate in 2 Dimensional space.
///
/// The coordinates contained in this struct are always non-negative and bounded
/// by the maximum allowed value based on the current precision settings. See
/// [`CoordinateValue`] for more details.
///
/// Can store one of two internal representations:
/// - Pixel-based: Absolute pixel values relative to the image dimensions,
///   e.g., (150, 300).
/// - Percentage-based: Relative percentage values of the image dimensions,
///   e.g., (15.0%, 30.0%), stored as (1500, 3000) with 2 decimal points of
///   precision or (15000, 30000) with 4 decimal points of precision.
///
/// By default, coordinates use a pixel-based internal representation.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinate {
    x: CoordinateValue,
    y: CoordinateValue,
}

/// The dimensions of an image.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImageDimensions {
    height: CoordinateValue,
    width: CoordinateValue,
}

/// A rectangular hotspot represented as a rectangle with two corners.
#[derive(Debug, Clone, Copy)]
pub struct Hotspot<R: InternalRepr = PixelRepr> {
    top_right: Coordinate,
    lower_left: Coordinate,
    _repr: core::marker::PhantomData<R>,
}

impl Hotspot<PixelRepr> {
    #[inline]
    pub const fn top_right(&self) -> Coordinate {
        self.top_right
    }

    #[inline]
    pub const fn top_left(&self) -> Coordinate {
        Coordinate {
            x: self.top_right.x,
            y: self.lower_left.y,
        }
    }

    #[inline]
    pub const fn lower_left(&self) -> Coordinate {
        self.lower_left
    }

    #[inline]
    pub const fn lower_right(&self) -> Coordinate {
        Coordinate {
            x: self.lower_left.x,
            y: self.top_right.y,
        }
    }

    #[inline]
    pub const fn as_percentage(
        this: Self,
        image_dimensions: ImageDimensions,
    ) -> Hotspot<PercentageRepr> {
        let Self {
            top_right,
            lower_left,
            _repr,
        } = this;
        // TODO: technically not the most efficient becuase `from_percentage` performs a bunch of checks that we don't really need anymore.
        Hotspot::builder()
            .with_repr::<PercentageRepr>()
            .from_percentage((top_right, lower_left), image_dimensions)
    }
}

impl Hotspot<PercentageRepr> {
    #[inline]
    pub const fn as_pixels(this: Self, image_dimensions: ImageDimensions) -> Hotspot<PixelRepr> {
        Hotspot {
            top_right: this.top_right(image_dimensions),
            lower_left: this.lower_left(image_dimensions),
            _repr: PhantomData,
        }
    }
}

macro_rules! impl_corner {
    ($func:ident, $name:literal) => {
        impl Hotspot<PercentageRepr> {
            #[doc = concat!("Get the ", $name, " coordinate in pixel values, given the image dimensions.")]
            ///
            /// This will take the internal percentage and multiply it against the
            /// height and width of the image to produce exact coordinates.
            ///
            /// Note that we will round to the closest pixel automatically.
            #[inline]
            pub const fn $func(
                &self,
                ImageDimensions { height, width }: ImageDimensions,
            ) -> Coordinate {
                // Exact the exact values as integers
                let Coordinate { x, y } = Hotspot::<PixelRepr>::$func(unsafe {
                    core::mem::transmute::<&Hotspot<PercentageRepr>, &Hotspot<PixelRepr>>(self)
                });

                let x: CoordinateValue = div_round_closest(
                    x as InternalCalculationType * width as InternalCalculationType,
                    CoordinateValue::MAX as InternalCalculationType,
                ) as CoordinateValue;

                let y: CoordinateValue = div_round_closest(
                    y as InternalCalculationType * height as InternalCalculationType,
                    CoordinateValue::MAX as InternalCalculationType,
                ) as CoordinateValue;

                Coordinate { x, y }
            }
        }
    };
}

impl_corner!(top_right, "top-right");
impl_corner!(top_left, "top-left");
impl_corner!(lower_left, "lower-left");
impl_corner!(lower_right, "lower-right");

impl<R: InternalRepr> Hotspot<R> {
    /// Calculate the overlap between two hotspots as a value between 0 and 1
    /// where 0 is no overlap and 1 is complete overlap.
    ///
    /// Note that this implementation is not perfect for calculating e.g. if two
    /// hotspots should be merged becuase if the size of two hotspots are of
    /// very different sizes then this may not report much overlap at all.
    ///
    /// E.g.
    /// > h1: 0,0 to 20,20 (area 400) \
    /// > h2: 5,5 to 15,15 (area 100) \
    /// > intersection: 5,5 to 15,15 (area 100) \
    /// > union: 400 + 100 - 100 = 400 \
    /// > overlap: 100 / 400 = 0.25
    ///
    /// If you need to decide if one hotspot should be merged into another
    /// consider using the [`overlap_in`] function instead.
    pub const fn overlap(&self, other: &Self) -> f32 {
        // https://stackoverflow.com/questions/9324339/how-much-do-two-rectangles-overlap
        let Coordinate { x: xa2, y: ya2 } = self.top_right;
        let Coordinate { x: xa1, y: ya1 } = self.lower_left;
        let Coordinate { x: xb2, y: yb2 } = other.top_right;
        let Coordinate { x: xb1, y: yb1 } = other.lower_left;

        // Cast to InternalCalculationType to prevent overflow during area calculation
        let xa1 = xa1 as InternalCalculationType;
        let xa2 = xa2 as InternalCalculationType;
        let ya1 = ya1 as InternalCalculationType;
        let ya2 = ya2 as InternalCalculationType;
        let xb1 = xb1 as InternalCalculationType;
        let xb2 = xb2 as InternalCalculationType;
        let yb1 = yb1 as InternalCalculationType;
        let yb2 = yb2 as InternalCalculationType;

        // Should always be true, but just in case.
        #[allow(
            clippy::absurd_extreme_comparisons,
            reason = "These types change based on features, this helps to reduce brittleness."
        )]
        {
            debug_assert!(
                CoordinateValue::MAX as InternalCalculationType
                    * CoordinateValue::MAX as InternalCalculationType
                    <= InternalCalculationType::MAX
            );
        }
        debug_assert!(
            core::mem::size_of::<InternalCalculationType>()
                > core::mem::size_of::<CoordinateValue>()
        );

        // Calculate area of rectangle A
        debug_assert!(xa2 >= xa1);
        debug_assert!(ya2 >= ya1);
        // SAFETY: We guarantee that x2 will be > x1 and y2 will be > y1 in the constructor so we can use unchecked_sub here.
        // Because the input types can be at most u16::MAX and our output type is a u32 the mul will always be safe too and so can become a unchecked_mul.
        let sa = unsafe { xa2.unchecked_sub(xa1).unchecked_mul(ya2.unchecked_sub(ya1)) };

        // Calculate area of rectangle B
        debug_assert!(xb2 >= xb1);
        debug_assert!(yb2 >= yb1);
        // SAFETY: We guarantee that x2 will be > x1 and y2 will be > y1 in the constructor so we can use unchecked_sub here.
        // Because the input types can be at most u16::MAX and our output type is a u32 the mul will always be safe too and so can become a unchecked_mul.
        let sb = unsafe { xb2.unchecked_sub(xb1).unchecked_mul(yb2.unchecked_sub(yb1)) };

        // Calculate intersection dimensions
        // We use saturating_sub because if the rectangles are disjoint,
        // min(right) - max(left) would be negative (underflow in unsigned).

        let intersection_w = min!(xa2, xb2).saturating_sub(max!(xa1, xb1));
        let intersection_h = min!(ya2, yb2).saturating_sub(max!(ya1, yb1));

        // Calculate area of intersection
        // SAFETY: The maximum overlap between two rectangles that were defined with u16 values is u16::MAX*u16::MAX
        // therefore we cannot overflow the U32 here.
        let si = unsafe { intersection_w.unchecked_mul(intersection_h) };

        // Calculate area of union
        // We subtract the intersection from the sum of the two areas.
        // However, sa + sb can overflow InternalCalculationType if both are large (e.g. u32::MAX).
        // Since we are calculating a ratio (si / su), we can cast to f32 before summing to avoid overflow
        // and maintain precision for the division.
        let su = sa as f32 + sb as f32 - si as f32;

        // Handle zero area union to avoid NaN
        if su == 0.0 {
            return 0.0;
        }

        // Calculate overlap %
        si as f32 / su
    }

    /// Calculate the % of this Hotspot that is in the other hotspot, returns an
    /// f32 where 0 is no overlap and 1 is complete overlap.
    ///
    /// Differs from [`overlap`] in that instead of calculating the total area /
    /// by the overlap it will return the area of self that is overlapped by
    /// other - regardless of the remaining area of other.
    ///
    /// E.g.
    /// > h1: 0,0 to 20,20 (area 400) \
    /// > h2: 5,5 to 15,15 (area 100) \
    /// > intersection: 5,5 to 15,15 (area 100) \
    /// > union: 400 + 100 - 100 = 400 \
    /// > overlap: 100 / 400 = 1.0
    pub const fn overlap_in(&self, other: &Self) -> f32 {
        let Coordinate { x: xa2, y: ya2 } = self.top_right;
        let Coordinate { x: xa1, y: ya1 } = self.lower_left;
        let Coordinate { x: xb2, y: yb2 } = other.top_right;
        let Coordinate { x: xb1, y: yb1 } = other.lower_left;

        // Cast to InternalCalculationType to prevent overflow during area calculation
        let xa1 = xa1 as InternalCalculationType;
        let xa2 = xa2 as InternalCalculationType;
        let ya1 = ya1 as InternalCalculationType;
        let ya2 = ya2 as InternalCalculationType;
        let xb1 = xb1 as InternalCalculationType;
        let xb2 = xb2 as InternalCalculationType;
        let yb1 = yb1 as InternalCalculationType;
        let yb2 = yb2 as InternalCalculationType;

        // Calculate area of rectangle A (self)
        debug_assert!(xa2 >= xa1);
        debug_assert!(ya2 >= ya1);
        // SAFETY: We guarantee that x2 will be > x1 and y2 will be > y1 in the constructor so we can use unchecked_sub here.
        // Because the input types can be at most u16::MAX and our output type is a u32 the mul will always be safe too and so can become a unchecked_mul.
        let sa = unsafe { xa2.unchecked_sub(xa1).unchecked_mul(ya2.unchecked_sub(ya1)) };

        // Calculate intersection dimensions
        // We use saturating_sub because if the rectangles are disjoint,
        // min(right) - max(left) would be negative (underflow in unsigned).
        let intersection_w = min!(xa2, xb2).saturating_sub(max!(xa1, xb1));
        let intersection_h = min!(ya2, yb2).saturating_sub(max!(ya1, yb1));

        // Calculate area of intersection
        // SAFETY: The maximum overlap between two rectangles that were defined with u16 values is u16::MAX*u16::MAX
        // therefore we cannot overflow the U32 here.
        let si = unsafe { intersection_w.unchecked_mul(intersection_h) };

        // Handle zero area self to avoid NaN
        if sa == 0 {
            return 0.0;
        }

        // Calculate overlap % relative to self
        si as f32 / sa as f32
    }

    /// Calculates the highest overlap between these two hotspots by taking the maximum value
    /// of calling [`overlap_in`] for each combination of self and other.
    #[inline]
    pub const fn max_overlap(&self, other: &Self) -> f32 {
        self.overlap_in(other).max(other.overlap_in(self))
    }

    /// Combines two hotspots and returns a new hotspot which will fully encompass the two provided hotspots.
    #[inline]
    pub const fn combine_hotspots(this: Self, other: Self) -> Self {
        Self {
            top_right: Coordinate {
                x: max!(this.top_right.x, other.top_right.x),
                y: max!(this.top_right.y, other.top_right.y),
            },
            lower_left: Coordinate {
                x: min!(this.lower_left.x, other.lower_left.x),
                y: min!(this.lower_left.y, other.lower_left.y),
            },
            _repr: PhantomData,
        }
    }
}

/// A builder for creating hotspots.
pub struct HotspotBuilder<R> {
    _marker: PhantomData<R>,
}

impl Hotspot {
    /// Create a builder for a hotspot.
    #[inline]
    pub const fn builder() -> HotspotBuilder<PixelRepr> {
        HotspotBuilder {
            _marker: core::marker::PhantomData,
        }
    }
}

impl<R: InternalRepr> HotspotBuilder<R> {
    /// Set the internal representation for the hotspot.
    #[inline]
    pub const fn with_repr<NewR: InternalRepr>(self) -> HotspotBuilder<NewR> {
        HotspotBuilder {
            _marker: core::marker::PhantomData,
        }
    }
}

impl HotspotBuilder<PixelRepr> {
    /// Create a pixel-based hotspot from top-left and bottom-right coordinates.
    ///
    /// NOTE: we assume that these are provided with the origin in the bottom left, e.g.
    ///
    /// X is expected to be up/down (i.e. vertical), Y is expected to be left/right (i.e. Horizontal).
    #[inline]
    pub const fn from_pixels(
        self,
        (Coordinate { x: x1, y: y1 }, Coordinate { x: x2, y: y2 }): (Coordinate, Coordinate),
    ) -> Hotspot<PixelRepr> {
        let top_right = Coordinate {
            x: max!(x1, x2),
            y: max!(y1, y2),
        };

        let lower_left = Coordinate {
            x: min!(x1, x2),
            y: min!(y1, y2),
        };

        Hotspot {
            top_right,
            lower_left,
            _repr: core::marker::PhantomData,
        }
    }
}

impl HotspotBuilder<PercentageRepr> {
    /// Create a percentage-based hotspot from top-left and bottom-right coordinates and image dimensions.
    #[inline]
    pub const fn from_percentage(
        self,
        input: (Coordinate, Coordinate),
        ImageDimensions { height, width }: ImageDimensions,
    ) -> Hotspot<PercentageRepr> {
        // Use the HotspotBuilder<PixelRepr>::from_pixel representation to handle
        // which point is which.
        let Hotspot {
            top_right,
            lower_left,
            _repr,
        } = Hotspot::<PixelRepr>::builder().from_pixels(input);

        let height = height as InternalCalculationType;
        let width = width as InternalCalculationType;

        // Convert the pixel coordinates to internal percentages of the maximum possible value.
        let top_right = Coordinate {
            x: div_round_closest(
                top_right.x as InternalCalculationType
                    * CoordinateValue::MAX as InternalCalculationType,
                width,
            ) as CoordinateValue,
            y: div_round_closest(
                top_right.y as InternalCalculationType
                    * CoordinateValue::MAX as InternalCalculationType,
                height,
            ) as CoordinateValue,
        };
        let lower_left = Coordinate {
            x: div_round_closest(
                lower_left.x as InternalCalculationType
                    * CoordinateValue::MAX as InternalCalculationType,
                width,
            ) as CoordinateValue,
            y: div_round_closest(
                lower_left.y as InternalCalculationType
                    * CoordinateValue::MAX as InternalCalculationType,
                height,
            ) as CoordinateValue,
        };

        Hotspot {
            top_right,
            lower_left,
            _repr: core::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(feature = "high_precision"))]
    #[test]
    fn test_percentage_repr() {
        let hotspot = Hotspot::builder()
            .with_repr::<PercentageRepr>()
            .from_percentage(
                (Coordinate { x: 50, y: 50 }, Coordinate { x: 2622, y: 2622 }),
                crate::ImageDimensions {
                    height: 5000,
                    width: 5000,
                },
            );

        assert_eq!(hotspot.top_right, Coordinate { x: 34367, y: 34367 });
        assert_eq!(hotspot.lower_left, Coordinate { x: 655, y: 655 });

        assert_eq!(
            hotspot.top_right(crate::ImageDimensions {
                height: 5000,
                width: 5000,
            }),
            Coordinate { x: 2622, y: 2622 }
        );

        assert_eq!(
            hotspot.lower_right(crate::ImageDimensions {
                height: 10000,
                width: 5000,
            }),
            Coordinate { x: 50, y: 5244 }
        );
    }

    #[cfg(feature = "high_precision")]
    #[test]
    fn test_percentage_repr() {
        let hotspot = Hotspot::builder()
            .with_repr::<PercentageRepr>()
            .from_percentage(
                (Coordinate { x: 50, y: 50 }, Coordinate { x: 2622, y: 2622 }),
                crate::ImageDimensions {
                    height: 5000,
                    width: 5000,
                },
            );

        assert_eq!(
            hotspot.top_right,
            Coordinate {
                x: 2252280849,
                y: 2252280849
            }
        );
        assert_eq!(
            hotspot.lower_left,
            Coordinate {
                x: 42949673,
                y: 42949673
            }
        );

        assert_eq!(
            hotspot.top_right(crate::ImageDimensions {
                height: 5000,
                width: 5000,
            }),
            Coordinate { x: 2622, y: 2622 }
        );

        assert_eq!(
            hotspot.lower_right(crate::ImageDimensions {
                height: 10000,
                width: 5000,
            }),
            Coordinate { x: 50, y: 5244 }
        );
    }

    fn make_hotspot(x1: u16, y1: u16, x2: u16, y2: u16) -> Hotspot<PixelRepr> {
        Hotspot::builder().from_pixels((
            Coordinate {
                x: x1 as CoordinateValue,
                y: y1 as CoordinateValue,
            },
            Coordinate {
                x: x2 as CoordinateValue,
                y: y2 as CoordinateValue,
            },
        ))
    }

    #[test]
    fn test_no_overlap() {
        let h1 = make_hotspot(0, 0, 10, 10);
        let h2 = make_hotspot(20, 20, 30, 30);
        assert_eq!(h1.overlap(&h2), 0.0);
    }

    #[test]
    fn test_complete_overlap() {
        let h1 = make_hotspot(0, 0, 10, 10);
        let h2 = make_hotspot(0, 0, 10, 10);
        assert_eq!(h1.overlap(&h2), 1.0);
    }

    #[test]
    fn test_partial_overlap() {
        // h1: 0,0 to 10,10 (area 100)
        // h2: 5,0 to 15,10 (area 100)
        // intersection: 5,0 to 10,10 (width 5, height 10, area 50)
        // union: 100 + 100 - 50 = 150
        // overlap: 50 / 150 = 1/3
        let h1 = make_hotspot(0, 0, 10, 10);
        let h2 = make_hotspot(5, 0, 15, 10);
        assert!((h1.overlap(&h2) - (1.0 / 3.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_contained_overlap() {
        // h1: 0,0 to 20,20 (area 400)
        // h2: 5,5 to 15,15 (area 100)
        // intersection: 5,5 to 15,15 (area 100)
        // union: 400 + 100 - 100 = 400
        // overlap: 100 / 400 = 0.25
        let h1 = make_hotspot(0, 0, 20, 20);
        let h2 = make_hotspot(5, 5, 15, 15);
        assert_eq!(h1.overlap(&h2), 0.25);

        assert_eq!(h2.overlap_in(&h1), 1.0);
        assert_eq!(h1.overlap_in(&h2), 0.25);
    }

    #[test]
    fn test_corner_overlap() {
        // h1: 0,0 to 10,10 (area 100)
        // h2: 5,5 to 15,15 (area 100)
        // intersection: 5,5 to 10,10 (width 5, height 5, area 25)
        // union: 100 + 100 - 25 = 175
        // overlap: 25 / 175 = 1/7
        let h1 = make_hotspot(0, 0, 10, 10);
        let h2 = make_hotspot(5, 5, 15, 15);
        assert!((h1.overlap(&h2) - (1.0 / 7.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_zero_area() {
        let h1 = make_hotspot(0, 0, 0, 0);
        let h2 = make_hotspot(0, 0, 10, 10);
        assert_eq!(h1.overlap(&h2), 0.0);

        let h1 = make_hotspot(0, 0, 0, 0);
        let h2 = make_hotspot(0, 0, 0, 0);
        assert_eq!(h1.overlap(&h2), 0.0);
    }

    #[test]
    fn test_overflow() {
        let h1 = make_hotspot(0, 0, u16::MAX, u16::MAX);
        let h2 = make_hotspot(0, 0, u16::MAX, u16::MAX);
        assert_eq!(h1.overlap(&h2), 1.0);
    }

    #[test]
    fn test_from_pixels_equal_coordinates() {
        // Test when x1 == x2 and y1 == y2 (single point)
        let h1 = make_hotspot(5, 5, 5, 5);
        assert_eq!(h1.lower_left, Coordinate { x: 5, y: 5 });
        assert_eq!(h1.top_right, Coordinate { x: 5, y: 5 });

        // Test when x1 == x2 but y1 != y2 (vertical line)
        let h2 = make_hotspot(5, 0, 5, 10);
        assert_eq!(h2.lower_left, Coordinate { x: 5, y: 0 });
        assert_eq!(h2.top_right, Coordinate { x: 5, y: 10 });

        // Test when y1 == y2 but x1 != x2 (horizontal line)
        let h3 = make_hotspot(0, 5, 10, 5);
        assert_eq!(h3.lower_left, Coordinate { x: 0, y: 5 });
        assert_eq!(h3.top_right, Coordinate { x: 10, y: 5 });

        // Test with reversed coordinates (x2 < x1, y2 < y1)
        let h4 = make_hotspot(10, 10, 0, 0);
        assert_eq!(h4.lower_left, Coordinate { x: 0, y: 0 });
        assert_eq!(h4.top_right, Coordinate { x: 10, y: 10 });
    }

    #[test]
    fn test_from_pixels_strict_inequality_logic() {
        // Test that the implementation uses strict < and > (not <= and >=)
        // by verifying behavior for all orderings including adjacent values

        // When x1 < x2: lower_left should get x1, top_right should get x2
        let h = make_hotspot(5, 7, 6, 8);
        assert_eq!(h.lower_left.x, 5);
        assert_eq!(h.lower_left.y, 7);
        assert_eq!(h.top_right.x, 6);
        assert_eq!(h.top_right.y, 8);

        // When x1 > x2: lower_left should get x2, top_right should get x1
        let h = make_hotspot(10, 20, 9, 19);
        assert_eq!(h.lower_left.x, 9);
        assert_eq!(h.lower_left.y, 19);
        assert_eq!(h.top_right.x, 10);
        assert_eq!(h.top_right.y, 20);

        // Test exhaustively with small values to cover all branches
        for x1 in 0u16..15 {
            for x2 in 0u16..15 {
                let h = make_hotspot(x1, 0, x2, 0);

                // Verify the correct value is in each position
                if x1 < x2 {
                    assert_eq!(h.lower_left.x, x1 as CoordinateValue);
                    assert_eq!(h.top_right.x, x2 as CoordinateValue);
                } else {
                    assert_eq!(h.lower_left.x, x2 as CoordinateValue);
                    assert_eq!(h.top_right.x, x1 as CoordinateValue);
                }
            }
        }

        // Same for y coordinates
        for y1 in 0u16..15 {
            for y2 in 0u16..15 {
                let h = make_hotspot(0, y1, 0, y2);

                if y1 < y2 {
                    assert_eq!(h.lower_left.y, y1 as CoordinateValue);
                    assert_eq!(h.top_right.y, y2 as CoordinateValue);
                } else {
                    assert_eq!(h.lower_left.y, y2 as CoordinateValue);
                    assert_eq!(h.top_right.y, y1 as CoordinateValue);
                }
            }
        }
    }

    #[test]
    fn test_max_overlap_symmetric() {
        // Test max_overlap with identical hotspots
        let h1 = make_hotspot(0, 0, 10, 10);
        let h2 = make_hotspot(0, 0, 10, 10);
        assert_eq!(h1.max_overlap(&h2), 1.0);
    }

    #[test]
    fn test_max_overlap_no_overlap() {
        // Test max_overlap with non-overlapping hotspots
        let h1 = make_hotspot(0, 0, 10, 10);
        let h2 = make_hotspot(20, 20, 30, 30);
        assert_eq!(h1.max_overlap(&h2), 0.0);
    }

    #[test]
    fn test_max_overlap_contained() {
        // h1: 0,0 to 20,20 (area 400)
        // h2: 5,5 to 15,15 (area 100)
        // h1.overlap_in(h2) = 100/400 = 0.25
        // h2.overlap_in(h1) = 100/100 = 1.0
        // max_overlap should return 1.0
        let h1 = make_hotspot(0, 0, 20, 20);
        let h2 = make_hotspot(5, 5, 15, 15);
        assert_eq!(h1.max_overlap(&h2), 1.0);
    }

    #[test]
    fn test_max_overlap_partial() {
        // h1: 0,0 to 10,10 (area 100)
        // h2: 5,0 to 15,10 (area 100)
        // intersection: 5,0 to 10,10 (area 50)
        // h1.overlap_in(h2) = 50/100 = 0.5
        // h2.overlap_in(h1) = 50/100 = 0.5
        // max_overlap should return 0.5
        let h1 = make_hotspot(0, 0, 10, 10);
        let h2 = make_hotspot(5, 0, 15, 10);
        assert_eq!(h1.max_overlap(&h2), 0.5);
    }

    // Property-based tests (fuzzing)
    #[cfg(not(miri))]
    mod fuzz_tests {
        use super::*;
        use proptest::prelude::*;

        prop_compose! {
            fn arb_coordinate()(x in 0..CoordinateValue::MAX, y in 0..CoordinateValue::MAX) -> Coordinate {
                Coordinate { x, y }
            }
        }

        prop_compose! {
            fn arb_hotspot()(c1 in arb_coordinate(), c2 in arb_coordinate()) -> Hotspot<PixelRepr> {
                Hotspot::builder().from_pixels((c1, c2))
            }
        }

        prop_compose! {
            fn arb_dimensions()(
                width in 1..CoordinateValue::MAX,
                height in 1..CoordinateValue::MAX
            ) -> ImageDimensions {
                ImageDimensions { width, height }
            }
        }

        proptest! {
            #[test]
            fn fuzz_from_pixels_invariants(c1 in arb_coordinate(), c2 in arb_coordinate()) {
                let h = Hotspot::builder().from_pixels((c1, c2));

                // Invariant: lower_left should have smaller or equal coordinates
                prop_assert!(h.lower_left.x <= h.top_right.x);
                prop_assert!(h.lower_left.y <= h.top_right.y);

                // Test the exact branching logic for each coordinate
                // For lower_left: should use the branch `if x1 < x2 { x1 } else { x2 }`
                let expected_lower_x = if c1.x < c2.x { c1.x } else { c2.x };
                let expected_lower_y = if c1.y < c2.y { c1.y } else { c2.y };
                prop_assert_eq!(h.lower_left.x, expected_lower_x);
                prop_assert_eq!(h.lower_left.y, expected_lower_y);

                // For top_right: should use the branch `if x1 > x2 { x1 } else { x2 }`
                let expected_top_x = if c1.x > c2.x { c1.x } else { c2.x };
                let expected_top_y = if c1.y > c2.y { c1.y } else { c2.y };
                prop_assert_eq!(h.top_right.x, expected_top_x);
                prop_assert_eq!(h.top_right.y, expected_top_y);
            }

            #[test]
            fn fuzz_overlap_symmetry(h1 in arb_hotspot(), h2 in arb_hotspot()) {
                let o1 = h1.overlap(&h2);
                let o2 = h2.overlap(&h1);
                prop_assert!((o1 - o2).abs() < f32::EPSILON);
            }

            #[test]
            fn fuzz_overlap_bounds(h1 in arb_hotspot(), h2 in arb_hotspot()) {
                let o = h1.overlap(&h2);
                prop_assert!(o >= 0.0);
                prop_assert!(o <= 1.0);
            }

            #[test]
            fn fuzz_overlap_in_bounds(h1 in arb_hotspot(), h2 in arb_hotspot()) {
                let o = h1.overlap_in(&h2);
                prop_assert!(o >= 0.0);
                prop_assert!(o <= 1.0);
            }

            #[test]
            fn fuzz_combine_hotspots_containment(h1 in arb_hotspot(), h2 in arb_hotspot()) {
                let combined = Hotspot::combine_hotspots(h1, h2);

                // Check h1 is inside
                prop_assert!(combined.top_right.x >= h1.top_right.x);
                prop_assert!(combined.top_right.y >= h1.top_right.y);
                prop_assert!(combined.lower_left.x <= h1.lower_left.x);
                prop_assert!(combined.lower_left.y <= h1.lower_left.y);

                // Check h2 is inside
                prop_assert!(combined.top_right.x >= h2.top_right.x);
                prop_assert!(combined.top_right.y >= h2.top_right.y);
                prop_assert!(combined.lower_left.x <= h2.lower_left.x);
                prop_assert!(combined.lower_left.y <= h2.lower_left.y);
            }

            #[test]
            fn fuzz_combine_hotspots_overlap_in(h1 in arb_hotspot(), h2 in arb_hotspot()) {
                let combined = Hotspot::combine_hotspots(h1, h2);

                let h1_area = (h1.top_right.x - h1.lower_left.x) as InternalCalculationType * (h1.top_right.y - h1.lower_left.y) as InternalCalculationType;
                if h1_area > 0 {
                    prop_assert!((h1.overlap_in(&combined) - 1.0).abs() < 1e-5);
                }

                let h2_area = (h2.top_right.x - h2.lower_left.x) as InternalCalculationType * (h2.top_right.y - h2.lower_left.y) as InternalCalculationType;
                if h2_area > 0 {
                    prop_assert!((h2.overlap_in(&combined) - 1.0).abs() < 1e-5);
                }
            }

            #[test]
            fn fuzz_percentage_roundtrip(
                h in arb_hotspot(),
                dims in arb_dimensions()
            ) {
                // Constrain hotspot to be within dimensions for valid percentage calculation
                let h_constrained = Hotspot::builder().from_pixels((
                    Coordinate {
                        x: h.lower_left.x % dims.width,
                        y: h.lower_left.y % dims.height
                    },
                    Coordinate {
                        x: h.top_right.x % dims.width,
                        y: h.top_right.y % dims.height
                    }
                ));

                let p = Hotspot::as_percentage(h_constrained, dims);
                let back = Hotspot::as_pixels(p, dims);

                // Calculate tolerance based on precision loss from u16 scaling
                let tolerance_x = (dims.width as f64 / u16::MAX as f64).ceil() as CoordinateValue + 1;
                let tolerance_y = (dims.height as f64 / u16::MAX as f64).ceil() as CoordinateValue + 1;

                let diff_x1 = if back.lower_left.x > h_constrained.lower_left.x { back.lower_left.x - h_constrained.lower_left.x } else { h_constrained.lower_left.x - back.lower_left.x };
                let diff_y1 = if back.lower_left.y > h_constrained.lower_left.y { back.lower_left.y - h_constrained.lower_left.y } else { h_constrained.lower_left.y - back.lower_left.y };
                let diff_x2 = if back.top_right.x > h_constrained.top_right.x { back.top_right.x - h_constrained.top_right.x } else { h_constrained.top_right.x - back.top_right.x };
                let diff_y2 = if back.top_right.y > h_constrained.top_right.y { back.top_right.y - h_constrained.top_right.y } else { h_constrained.top_right.y - back.top_right.y };

                prop_assert!(diff_x1 <= tolerance_x);
                prop_assert!(diff_y1 <= tolerance_y);
                prop_assert!(diff_x2 <= tolerance_x);
                prop_assert!(diff_y2 <= tolerance_y);
            }
        }
    }
}
