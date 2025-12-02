# Hotspots

A lightweight, Rust library for working with 2D rectangular hotspots. Supports
multiple internal representations (pixel-based and percentage-based), lossless
conversions, overlap detection, utiltiies for handling conversions between
different points of origins and generally working with hotspots are also
provided.

## Important Note on Precision

Coordinates are stored as fractions of the maximum value:
- **Default (u16)**: 65,536 discrete positions
- **With `high_precision` (u32)**: 4,294,967,296 discrete positions

**When precision loss occurs**: If your image dimensions exceed these values,
multiple adjacent pixels map to the same internal representation. For
percentage-based hotspots on a 100,000×100,000 pixel image with u16, expect ~1-2
pixel rounding errors. Use `high_precision` for images larger than ~65,000
pixels in either dimension.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
hotspots = "0.1"
```

### Features

- `serde`: Enable serialization/deserialization support
- `reflectapi`: Enable ReflectAPI schema generation
- `high_precision`: Use `u32` coordinates with instead of `u16`. See [Important Note on Precision](##important-note-on-precision) for more information.

## Usage

### Basic Pixel Hotspots

```rust
use hotspots::{Hotspot, Coordinate};

// Create a hotspot from two corners (pixel coordinates)
let hotspot = Hotspot::builder().from_pixels((
    Coordinate { x: 100, y: 150 },
    Coordinate { x: 200, y: 250 },
));

// Access corners
let upper_right = hotspot.upper_right();
let lower_left = hotspot.lower_left();
let upper_left = hotspot.upper_left();
let lower_right = hotspot.lower_right();
```

### Percentage-Based Hotspots

```rust
use hotspots::{Hotspot, Coordinate, ImageDimensions, repr::PercentageRepr};

// Create a percentage-based hotspot
let dimensions = ImageDimensions { width: 1920, height: 1080 };

let hotspot = Hotspot::builder()
    .with_repr::<PercentageRepr>()
    .from_percentage(
        (
            Coordinate { x: 100, y: 200 },  // Internal percentage representation
            Coordinate { x: 300, y: 400 }
        ),
        dimensions
    );

// Get pixel coordinates for a specific image size
let pixel_coords = hotspot.upper_right(dimensions);
```

## Limitations

- **No Rotation**: Hotspots must be axis-aligned rectangles
- **Rectangular Only**: Only rectangular hotspots are supported,
  though we do provide utilties for converting point-based coordinates.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
