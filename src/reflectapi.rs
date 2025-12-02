extern crate alloc;

use alloc::{collections::BTreeSet, string::String, vec};
use reflectapi::{
    Field, Fields, Input, LanguageSpecificTypeCodegenConfig, Output, RustTypeCodegenConfig, Struct,
    Type, TypeParameter, TypeReference, codegen::rust::Config,
};

// TODO: don't have the `codegen` feature enabled by default.

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

impl<R> Input for Hotspot<R> {
    fn reflectapi_input_type(schema: &mut reflectapi::Typespace) -> reflectapi::TypeReference {
        let resolved_type_name = "Hotspot";
        let coordinate_value_type = <CoordinateValue as Input>::reflectapi_input_type(schema);
        if schema.reserve_type(resolved_type_name.as_ref()) {
            let reflected_type_def = reflectapi::Type::Struct(reflectapi::Struct {
                name: "Hotspot".into(),
                serde_name: "".into(),
                description: "A rectangular hotspot represented as a rectangle with two corners."
                    .into(),
                parameters: vec![],
                fields: reflectapi::Fields::Named(vec![
                    reflectapi::Field {
                        name: "x1".into(),
                        serde_name: "".into(),
                        description: "".into(),
                        deprecation_note: None.into(),
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
                        deprecation_note: None.into(),
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
                        deprecation_note: None.into(),
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
                        deprecation_note: None.into(),
                        type_ref: coordinate_value_type.clone(),
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
            });
            schema.insert_type(reflected_type_def);
        }
        reflectapi::TypeReference::new(resolved_type_name, vec![])
    }
}

impl<R> Output for Hotspot<R> {
    fn reflectapi_output_type(schema: &mut reflectapi::Typespace) -> reflectapi::TypeReference {
        let resolved_type_name = "Hotspot";
        let coordinate_value_type = <CoordinateValue as Output>::reflectapi_output_type(schema);
        if schema.reserve_type(resolved_type_name.as_ref()) {
            let reflected_type_def = reflectapi::Type::Struct(reflectapi::Struct {
                name: "Hotspot".into(),
                serde_name: "".into(),
                description: "A rectangular hotspot represented as a rectangle with two corners."
                    .into(),
                parameters: vec![],
                fields: reflectapi::Fields::Named(vec![
                    reflectapi::Field {
                        name: "x1".into(),
                        serde_name: "".into(),
                        description: "".into(),
                        deprecation_note: None.into(),
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
                        deprecation_note: None.into(),
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
                        deprecation_note: None.into(),
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
                        deprecation_note: None.into(),
                        type_ref: coordinate_value_type.clone(),
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
            });
            schema.insert_type(reflected_type_def);
        }
        reflectapi::TypeReference::new(resolved_type_name, vec![])
    }
}

#[cfg(test)]
mod tests {
    use crate::repr::PixelRepr;

    use super::*;

    #[ignore]
    #[test]
    fn test_generate_schema_with_coordinate() {
        #[derive(Debug)]
        pub struct AppState {}

        async fn health_check(
            _: (),
            _request: Coordinate,
            _headers: reflectapi::Empty,
        ) -> Hotspot<PixelRepr> {
            unimplemented!()
        }
        pub fn builder() -> reflectapi::Builder<()> {
            reflectapi::Builder::new()
                .name("Demo application")
                .description("This is a demo application")
                .route(health_check, |b| {
                    b.name("health.check")
                        .description("Check the health of the service")
                })
        }

        let (schema, _) = builder().build().unwrap();
        // panic!(
        //     "{}",
        //     serde_json::to_string_pretty(&reflectapi::codegen::openapi::Spec::from(&schema))
        //         .unwrap()
        // );

        let mut config = Config::default();
        config.format(true).typecheck(true);

        let generated = reflectapi::codegen::rust::generate(schema, &config).unwrap();

        panic!("{}", generated);
    }
}
