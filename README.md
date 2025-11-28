# Hotspots

A simple crate for working with two dimensional hotspots, supports working with multiple coordinate systems, lossless scaling, and hit testing.

## Usage

```toml
[dependencies]
hotspots = "0.1"
```

```rust
use hotspots::{Hotspot, HotspotCollection, CoordinateSystem, Origin};

fn main() {
    // Create a hotspot in pixel coordinates
    let hotspot = Hotspot::new(50.0, 50.0, CoordinateSystem::Pixels);
    
    // By default we assume that you want to use bottom left as the origin, but this can be changed.
    hotspot.set_origin(Origin::BottomLeft);

    // Create a collection of hotspots
    let mut collection = HotspotCollection::new();
    collection.add_hotspot(hotspot);

    // Scale the hotspots to a different coordinate system
    let scaled_collection = collection.scale_to(CoordinateSystem::Percentage, 200.0, 200.0);

    // Check if a point hits any hotspot
    let hit = scaled_collection.hit_test(25.0, 25.0);
    println!("Hit: {}", hit);
}
```

## Not Supported

This crate is quite limited at the moment, designed to be a simple utility for working with hotspots.

Limitations:
- Subpixel accuracy is not supported.
- Rotated hotspots are not supported, they must lie on axis-aligned bounding boxes.
- Only rectangular hotspots are supported.

I am open to expanding on this crate in the future - please open an issue!

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
