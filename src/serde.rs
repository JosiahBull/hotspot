//! Serde implementations for Coordinate and Hotspot types.
//!
//! Manually implemented to avoid calling the serde proc macros which tend to be quite slow at compile time.

use serde::{
    Deserialize,
    de::{self, Visitor},
    ser::{SerializeStruct, SerializeTupleStruct},
};

use crate::{Coordinate, CoordinateValue, Hotspot, ImageDimensions, repr::HotspotRepr};

impl serde::Serialize for ImageDimensions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Self { height, width } = &self;
        let mut ser = serializer.serialize_tuple_struct("ImageDimensions", 2)?;
        ser.serialize_field(width)?;
        ser.serialize_field(height)?;
        ser.end()
    }
}

impl<'de> serde::Deserialize<'de> for ImageDimensions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ImageDimensionsVisitor;

        impl<'de> Visitor<'de> for ImageDimensionsVisitor {
            type Value = ImageDimensions;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a dimension as [x, y]")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let width = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let height = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(ImageDimensions { width, height })
            }
        }

        deserializer.deserialize_tuple_struct("ImageDimensions", 2, ImageDimensionsVisitor)
    }
}

impl serde::Serialize for Coordinate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Self { x, y } = &self;
        let mut ser = serializer.serialize_tuple_struct("Coordinate", 2)?;
        ser.serialize_field(x)?;
        ser.serialize_field(y)?;
        ser.end()
    }
}

impl<'de> serde::Deserialize<'de> for Coordinate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CoordinateVisitor;

        impl<'de> Visitor<'de> for CoordinateVisitor {
            type Value = Coordinate;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a coordinate as either [x, y]")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let x = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let y = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Coordinate { x, y })
            }
        }

        deserializer.deserialize_tuple_struct("Coordinate", 2, CoordinateVisitor)
    }
}

impl<R: HotspotRepr> serde::Serialize for Hotspot<R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Self {
            upper_right: Coordinate { x: x1, y: y1 },
            lower_left: Coordinate { x: x2, y: y2 },
            _repr: _,
        } = &self;

        let mut ser = serializer.serialize_struct(R::STRUCT_NAME, 4)?;
        ser.serialize_field("x1", x1)?;
        ser.serialize_field("y1", y1)?;
        ser.serialize_field("x2", x2)?;
        ser.serialize_field("y2", y2)?;
        ser.end()
    }
}

impl<'de, R: HotspotRepr> serde::Deserialize<'de> for Hotspot<R> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["x1", "y1", "x2", "y2"];

        enum Field {
            X1,
            Y1,
            X2,
            Y2,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                        formatter.write_str("`x1`, `y1`, `x2` or `y2`")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match v {
                            "x1" => Ok(Field::X1),
                            "y1" => Ok(Field::Y1),
                            "x2" => Ok(Field::X2),
                            "y2" => Ok(Field::Y2),
                            _ => Err(de::Error::unknown_field(v, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct HotspotFields {
            x1: CoordinateValue,
            y1: CoordinateValue,
            x2: CoordinateValue,
            y2: CoordinateValue,
        }

        struct HotspotVisitor;

        impl<'de> Visitor<'de> for HotspotVisitor {
            type Value = HotspotFields;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("struct Hotspot")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let x1 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let y1 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let x2 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let y2 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;

                Ok(HotspotFields { x1, y1, x2, y2 })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut x1: Option<CoordinateValue> = None;
                let mut y1: Option<CoordinateValue> = None;
                let mut x2: Option<CoordinateValue> = None;
                let mut y2: Option<CoordinateValue> = None;

                // Parse all fields from the map
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::X1 => {
                            if x1.is_some() {
                                return Err(de::Error::duplicate_field("x1"));
                            }
                            x1 = Some(map.next_value()?);
                        }
                        Field::Y1 => {
                            if y1.is_some() {
                                return Err(de::Error::duplicate_field("y1"));
                            }
                            y1 = Some(map.next_value()?);
                        }
                        Field::X2 => {
                            if x2.is_some() {
                                return Err(de::Error::duplicate_field("x2"));
                            }
                            x2 = Some(map.next_value()?);
                        }
                        Field::Y2 => {
                            if y2.is_some() {
                                return Err(de::Error::duplicate_field("y2"));
                            }
                            y2 = Some(map.next_value()?);
                        }
                    }
                }

                // Extract all fields, returning errors for any missing fields
                let x1 = x1.ok_or_else(|| de::Error::missing_field("x1"))?;
                let y1 = y1.ok_or_else(|| de::Error::missing_field("y1"))?;
                let x2 = x2.ok_or_else(|| de::Error::missing_field("x2"))?;
                let y2 = y2.ok_or_else(|| de::Error::missing_field("y2"))?;

                Ok(HotspotFields { x1, y1, x2, y2 })
            }
        }

        let internal_hotspot =
            deserializer.deserialize_struct(R::STRUCT_NAME, FIELDS, HotspotVisitor)?;

        Ok(Hotspot {
            upper_right: Coordinate {
                x: internal_hotspot.x1,
                y: internal_hotspot.y1,
            },
            lower_left: Coordinate {
                x: internal_hotspot.x2,
                y: internal_hotspot.y2,
            },
            _repr: core::marker::PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::{Hotspot, ImageDimensions, repr::PixelRepr};
    use alloc::string::ToString;
    use alloc::{vec, vec::Vec};

    use super::*;

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

    // ============================================================================
    // Coordinate Serialization Tests
    // ============================================================================

    #[test]
    fn test_coordinate_serialize_deserialize() {
        let coord = Coordinate { x: 100, y: 200 };
        let json = serde_json::to_string(&coord).unwrap();
        assert_eq!(json, "[100,200]");

        let deserialized: Coordinate = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, coord);
    }

    #[test]
    fn test_coordinate_deserialize_from_array() {
        // Test array format deserialization
        let json = "[100,200]";
        let coord: Coordinate = serde_json::from_str(json).unwrap();
        assert_eq!(coord, Coordinate { x: 100, y: 200 });
    }

    #[test]
    fn test_coordinate_zero_values() {
        let coord = Coordinate { x: 0, y: 0 };
        let json = serde_json::to_string(&coord).unwrap();
        assert_eq!(json, "[0,0]");

        let deserialized: Coordinate = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, coord);
    }

    #[test]
    fn test_coordinate_max_values() {
        let coord = Coordinate {
            x: CoordinateValue::MAX,
            y: CoordinateValue::MAX,
        };
        let json = serde_json::to_string(&coord).unwrap();
        let deserialized: Coordinate = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, coord);
    }

    #[test]
    fn test_coordinate_asymmetric_values() {
        let coord = Coordinate { x: 1, y: 65535 };
        let json = serde_json::to_string(&coord).unwrap();
        let deserialized: Coordinate = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, coord);
    }

    // ============================================================================
    // Coordinate Error Handling Tests
    // ============================================================================

    #[test]
    fn test_coordinate_deserialize_missing_field_array() {
        let json = "[100]";
        let result: Result<Coordinate, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_coordinate_deserialize_missing_field_object() {
        let json = r#"{"x":100}"#;
        let result: Result<Coordinate, _> = serde_json::from_str(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("y"));
    }

    #[test]
    fn test_coordinate_deserialize_too_many_fields() {
        let json = "[100,200,300]";
        let result: Result<Coordinate, _> = serde_json::from_str(json);
        // Tuple structs require exact number of fields
        assert!(result.is_err());
    }

    // ============================================================================
    // Hotspot Serialization Tests
    // ============================================================================

    #[test]
    fn test_hotspot_serialize_json() {
        let hotspot = make_hotspot(10, 20, 30, 40);
        let json = serde_json::to_string(&hotspot).unwrap();

        // Parse to verify structure
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["x1"], 30);
        assert_eq!(value["y1"], 40);
        assert_eq!(value["x2"], 10);
        assert_eq!(value["y2"], 20);
    }

    #[test]
    fn test_hotspot_deserialize_json() {
        let json = r#"{"x1":100,"y1":200,"x2":50,"y2":75}"#;
        let hotspot: Hotspot<PixelRepr> = serde_json::from_str(json).unwrap();

        assert_eq!(hotspot.upper_right, Coordinate { x: 100, y: 200 });
        assert_eq!(hotspot.lower_left, Coordinate { x: 50, y: 75 });
    }

    #[test]
    fn test_hotspot_roundtrip_json() {
        let original = make_hotspot(0, 0, 100, 100);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Hotspot<PixelRepr> = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_hotspot_zero_area() {
        let hotspot = make_hotspot(5, 5, 5, 5);
        let json = serde_json::to_string(&hotspot).unwrap();
        let deserialized: Hotspot<PixelRepr> = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, hotspot);
    }

    #[test]
    fn test_hotspot_max_values() {
        let hotspot = make_hotspot(0, 0, u16::MAX, u16::MAX);
        let json = serde_json::to_string(&hotspot).unwrap();
        let deserialized: Hotspot<PixelRepr> = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, hotspot);
    }

    #[test]
    fn test_hotspot_percentage_repr_roundtrip() {
        let hotspot: Hotspot<crate::repr::PercentageRepr> = Hotspot::builder()
            .with_repr::<crate::repr::PercentageRepr>()
            .from_percentage(
                (Coordinate { x: 100, y: 200 }, Coordinate { x: 300, y: 400 }),
                ImageDimensions {
                    width: 1000,
                    height: 1000,
                },
            );

        let json = serde_json::to_string(&hotspot).unwrap();
        let deserialized: Hotspot<crate::repr::PercentageRepr> =
            serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, hotspot);
    }

    #[test]
    fn test_hotspot_deserialize_with_field_order() {
        // Test different field orders
        let json1 = r#"{"x1":10,"y1":20,"x2":5,"y2":15}"#;
        let json2 = r#"{"y2":15,"x2":5,"y1":20,"x1":10}"#;
        let json3 = r#"{"x2":5,"x1":10,"y2":15,"y1":20}"#;

        let h1: Hotspot<PixelRepr> = serde_json::from_str(json1).unwrap();
        let h2: Hotspot<PixelRepr> = serde_json::from_str(json2).unwrap();
        let h3: Hotspot<PixelRepr> = serde_json::from_str(json3).unwrap();

        assert_eq!(h1, h2);
        assert_eq!(h2, h3);
    }

    // ============================================================================
    // Hotspot Error Handling Tests
    // ============================================================================

    #[test]
    fn test_hotspot_deserialize_missing_x1() {
        let json = r#"{"y1":20,"x2":5,"y2":15}"#;
        let result: Result<Hotspot<PixelRepr>, _> = serde_json::from_str(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("x1"));
    }

    #[test]
    fn test_hotspot_deserialize_missing_y1() {
        let json = r#"{"x1":10,"x2":5,"y2":15}"#;
        let result: Result<Hotspot<PixelRepr>, _> = serde_json::from_str(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("y1"));
    }

    #[test]
    fn test_hotspot_deserialize_missing_x2() {
        let json = r#"{"x1":10,"y1":20,"y2":15}"#;
        let result: Result<Hotspot<PixelRepr>, _> = serde_json::from_str(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("x2"));
    }

    #[test]
    fn test_hotspot_deserialize_missing_y2() {
        let json = r#"{"x1":10,"y1":20,"x2":5}"#;
        let result: Result<Hotspot<PixelRepr>, _> = serde_json::from_str(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("y2"));
    }

    #[test]
    fn test_hotspot_deserialize_duplicate_x1() {
        let json = r#"{"x1":10,"x1":15,"y1":20,"x2":5,"y2":15}"#;
        let result: Result<Hotspot<PixelRepr>, _> = serde_json::from_str(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("duplicate"));
    }

    #[test]
    fn test_hotspot_deserialize_duplicate_y1() {
        let json = r#"{"x1":10,"y1":20,"y1":25,"x2":5,"y2":15}"#;
        let result: Result<Hotspot<PixelRepr>, _> = serde_json::from_str(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("duplicate"));
    }

    #[test]
    fn test_hotspot_deserialize_duplicate_x2() {
        let json = r#"{"x1":10,"y1":20,"x2":5,"x2":8,"y2":15}"#;
        let result: Result<Hotspot<PixelRepr>, _> = serde_json::from_str(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("duplicate"));
    }

    #[test]
    fn test_hotspot_deserialize_duplicate_y2() {
        let json = r#"{"x1":10,"y1":20,"x2":5,"y2":15,"y2":18}"#;
        let result: Result<Hotspot<PixelRepr>, _> = serde_json::from_str(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("duplicate"));
    }

    #[test]
    fn test_hotspot_deserialize_invalid_field() {
        let json = r#"{"x1":10,"y1":20,"x2":5,"y2":15,"x3":100}"#;
        let result: Result<Hotspot<PixelRepr>, _> = serde_json::from_str(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unknown field"));
    }

    // ============================================================================
    // Collection and Format Tests
    // ============================================================================

    #[test]
    fn test_multiple_hotspots_serialize() {
        let hotspots = vec![
            make_hotspot(0, 0, 10, 10),
            make_hotspot(20, 20, 30, 30),
            make_hotspot(40, 40, 50, 50),
        ];

        let json = serde_json::to_string(&hotspots).unwrap();
        let deserialized: Vec<Hotspot<PixelRepr>> = serde_json::from_str(&json).unwrap();
        assert_eq!(hotspots, deserialized);
    }

    #[test]
    fn test_coordinate_pretty_print() {
        let coord = Coordinate { x: 100, y: 200 };
        let json = serde_json::to_string_pretty(&coord).unwrap();
        assert!(json.contains("100"));
        assert!(json.contains("200"));
    }

    #[test]
    fn test_hotspot_pretty_print() {
        let hotspot = make_hotspot(10, 20, 30, 40);
        let json = serde_json::to_string_pretty(&hotspot).unwrap();
        assert!(json.contains("x1"));
        assert!(json.contains("y1"));
        assert!(json.contains("x2"));
        assert!(json.contains("y2"));
    }

    #[test]
    fn test_coordinate_from_value() {
        let value = serde_json::json!([500, 600]);
        let coord: Coordinate = serde_json::from_value(value).unwrap();
        assert_eq!(coord, Coordinate { x: 500, y: 600 });
    }

    #[test]
    fn test_hotspot_from_value() {
        let value = serde_json::json!({
            "x1": 10,
            "y1": 20,
            "x2": 30,
            "y2": 40
        });
        let hotspot: Hotspot<PixelRepr> = serde_json::from_value(value).unwrap();
        assert_eq!(hotspot.upper_right, Coordinate { x: 10, y: 20 });
        assert_eq!(hotspot.lower_left, Coordinate { x: 30, y: 40 });
    }

    #[test]
    fn test_coordinate_to_value() {
        let coord = Coordinate { x: 777, y: 888 };
        let value = serde_json::to_value(coord).unwrap();
        assert_eq!(value, serde_json::json!([777, 888]));
    }

    #[test]
    fn test_hotspot_to_value() {
        let hotspot = make_hotspot(1, 2, 3, 4);
        let value = serde_json::to_value(hotspot).unwrap();
        assert!(value.is_object());
        assert_eq!(value["x1"], 3);
        assert_eq!(value["y1"], 4);
        assert_eq!(value["x2"], 1);
        assert_eq!(value["y2"], 2);
    }
}
