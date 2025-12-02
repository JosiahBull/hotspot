extern crate alloc;

use alloc::{collections::BTreeSet, string::String, vec};
use reflectapi::{Input, Output};

use crate::{Coordinate, CoordinateValue, Hotspot};

impl Input for Coordinate {
    fn reflectapi_input_type(schema: &mut reflectapi::Typespace) -> reflectapi::TypeReference {
        <(CoordinateValue, CoordinateValue) as Input>::reflectapi_input_type(schema)
    }
}

impl Output for Coordinate {
    fn reflectapi_output_type(schema: &mut reflectapi::Typespace) -> reflectapi::TypeReference {
        <(CoordinateValue, CoordinateValue) as Output>::reflectapi_output_type(schema)
    }
}

fn hotspot_type_def(coordinate_value_type: reflectapi::TypeReference) -> reflectapi::Type {
    reflectapi::Type::Struct(reflectapi::Struct {
        name: "Hotspot".into(),
        serde_name: "".into(),
        description: "A rectangular hotspot represented as a rectangle with two corners.".into(),
        parameters: vec![],
        fields: reflectapi::Fields::Named(vec![
            reflectapi::Field {
                name: "x1".into(),
                serde_name: "".into(),
                description: "".into(),
                deprecation_note: None,
                type_ref: coordinate_value_type.clone(),
                required: true,
                flattened: false,
                transform_callback: String::new(),
                transform_callback_fn: None,
            },
            reflectapi::Field {
                name: "y1".into(),
                serde_name: "".into(),
                description: "".into(),
                deprecation_note: None,
                type_ref: coordinate_value_type.clone(),
                required: true,
                flattened: false,
                transform_callback: String::new(),
                transform_callback_fn: None,
            },
            reflectapi::Field {
                name: "x2".into(),
                serde_name: "".into(),
                description: "".into(),
                deprecation_note: None,
                type_ref: coordinate_value_type.clone(),
                required: true,
                flattened: false,
                transform_callback: String::new(),
                transform_callback_fn: None,
            },
            reflectapi::Field {
                name: "y2".into(),
                serde_name: "".into(),
                description: "".into(),
                deprecation_note: None,
                type_ref: coordinate_value_type,
                required: true,
                flattened: false,
                transform_callback: String::new(),
                transform_callback_fn: None,
            },
        ]),
        transparent: false,
        codegen_config: reflectapi::LanguageSpecificTypeCodegenConfig {
            rust: reflectapi::RustTypeCodegenConfig {
                additional_derives: BTreeSet::new(),
            },
        },
    })
}

impl<R> Input for Hotspot<R> {
    fn reflectapi_input_type(schema: &mut reflectapi::Typespace) -> reflectapi::TypeReference {
        let resolved_type_name = "Hotspot";
        let coordinate_value_type = <CoordinateValue as Input>::reflectapi_input_type(schema);
        if schema.reserve_type(resolved_type_name.as_ref()) {
            schema.insert_type(hotspot_type_def(coordinate_value_type));
        }
        reflectapi::TypeReference::new(resolved_type_name, vec![])
    }
}

impl<R> Output for Hotspot<R> {
    fn reflectapi_output_type(schema: &mut reflectapi::Typespace) -> reflectapi::TypeReference {
        let resolved_type_name = "Hotspot";
        let coordinate_value_type = <CoordinateValue as Output>::reflectapi_output_type(schema);
        if schema.reserve_type(resolved_type_name.as_ref()) {
            schema.insert_type(hotspot_type_def(coordinate_value_type));
        }
        reflectapi::TypeReference::new(resolved_type_name, vec![])
    }
}
