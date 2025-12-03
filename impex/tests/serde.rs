#[allow(unused)]
mod generated_struct;
#[allow(unused)]
mod manual_struct;

// ============================================================================
// Switch between manual and generated implementations by commenting/uncommenting:
// ============================================================================

// --- Use GENERATED (default, checked in) ---
use crate::generated_struct::{
    EnumConfig, EnumConfigImpex, KeyStructConfigImpex, MixedEnumConfig, MixedEnumConfigImpex,
    StructWithUnitEnumImpex, TupleStructConfig, TupleStructConfigImpex, UnionEnumConfig,
    UnionEnumConfigImpex,
};

// --- Use MANUAL (for debugging) ---
// use crate::manual_struct::{
//     EnumConfig, EnumConfigImpex, KeyStructConfigImpex, MixedEnumConfig, MixedEnumConfigImpex,
//     StructWithUnitEnumImpex, TupleStructConfig, TupleStructConfigImpex, UnionEnumConfig,
//     UnionEnumConfigImpex,
// };

#[test]
fn serialize_with_defaults() {
    use impex::Impex;

    let text = r#"{"num_cores":3}"#;
    let mut obj =
        serde_json::from_str::<KeyStructConfigImpex<::impex::DefaultWrapperSettings>>(text)
            .unwrap();
    assert_eq!(3, *obj.num_cores);
    assert_eq!(42, *obj.num_threads[0]);
    assert!(obj.num_cores.is_explicit());
    assert!(obj.num_threads[0].is_implicit());
    assert_eq!(text, serde_json::to_string(&obj).unwrap().as_str());

    match &mut obj.enum_config {
        EnumConfigImpex::Bar(_, x, _) => x.set_explicit(43),
        _ => panic!(),
    }
    let after_set_bar_field_text = serde_json::to_string(&obj).unwrap();

    assert_eq!(
        r#"{"num_cores":3,"enum_config":{"Bar":[null,43,[null,null]]}}"#,
        after_set_bar_field_text.as_str()
    );

    obj.enum_config
        .set_explicit(EnumConfig::Bar("Custom".into(), 42, Default::default()));
    match &obj.enum_config {
        EnumConfigImpex::Bar(x1, x2, x3) => {
            assert!(x1.is_explicit());
            assert_eq!(x1.as_str(), "Custom");

            assert!(x2.is_explicit());
            assert_eq!(**x2, 42);

            assert!(x3.is_explicit());
            assert_eq!(
                x3,
                &::impex::IntoImpex::into_explicit(TupleStructConfig::default())
            );
        }
        _ => panic!(),
    }
    assert_eq!(
        r#"{"num_cores":3,"enum_config":{"Bar":["Custom",42,[42,43]]}}"#,
        serde_json::to_string(&obj).unwrap().as_str()
    );
}

#[test]
#[ignore = "Feature was temporarily removed to allow !Default primitives, will be re-added later"]
fn test_serialize_field_enum_skips_implicit_fields() {
    use impex::Impex;
    let text = r#"{"enum_config":{"Foo":{}}}"#;
    let x: KeyStructConfigImpex<::impex::DefaultWrapperSettings> =
        serde_json::from_str(text).unwrap();

    let EnumConfigImpex::Foo {
        foo_value,
        tuple_struct_config,
    } = x.enum_config
    else {
        panic!("Should be a FooConfig")
    };
    assert!(foo_value.is_implicit());
    assert_eq!(foo_value.into_value(), String::default());
    assert!(tuple_struct_config.is_implicit());
    assert_eq!(
        tuple_struct_config.into_value(),
        TupleStructConfig::default()
    );
}

#[test]
fn tuple_struct() {
    let text = r#"[42, 84]"#;
    let tuple_struct: TupleStructConfigImpex<::impex::DefaultWrapperSettings> =
        serde_json::from_str(text).unwrap();
    assert!(tuple_struct.0.is_explicit());
}

// ============================================================================
// Unit Enum Tests
// ============================================================================

#[test]
fn unit_enum_implicit_is_missing_when_serializing() {
    use impex::Impex;

    // Deserialize an empty object - the unit_enum field takes its default value implicitly
    let obj: StructWithUnitEnumImpex<impex::DefaultWrapperSettings> =
        serde_json::from_str(r#"{}"#).unwrap();

    // The unit_enum field should be implicit (loaded from default, not from JSON)
    assert!(<_ as Impex<impex::DefaultWrapperSettings>>::is_implicit(
        &obj.unit_enum
    ));

    // When serializing back, implicit fields should be missing
    let serialized = serde_json::to_string(&obj).unwrap();
    assert_eq!(
        serialized, r#"{}"#,
        "Implicit unit enum should not appear in output"
    );
}

#[test]
fn unit_enum_explicit_appears_when_serializing() {
    use impex::Impex;

    // Deserialize with the unit_enum field present - serializes as just "Foo"
    let obj: StructWithUnitEnumImpex<impex::DefaultWrapperSettings> =
        serde_json::from_str(r#"{"unit_enum":"Foo"}"#).unwrap();

    // The unit_enum field should be explicit (it was in the JSON)
    assert!(
        <_ as Impex<impex::DefaultWrapperSettings>>::is_explicit(&obj.unit_enum),
        "Field present in JSON should be explicit"
    );

    // When serializing, explicit fields should appear as just the variant name
    let serialized = serde_json::to_string(&obj).unwrap();
    assert_eq!(
        serialized, r#"{"unit_enum":"Foo"}"#,
        "Explicit unit enum should appear in output"
    );
}

#[test]
fn unit_enum_different_variant_explicit() {
    use impex::Impex;

    // Test with Bar variant explicitly set - serializes as just "Bar"
    let obj: StructWithUnitEnumImpex<impex::DefaultWrapperSettings> =
        serde_json::from_str(r#"{"unit_enum":"Bar"}"#).unwrap();

    assert!(<_ as Impex<impex::DefaultWrapperSettings>>::is_explicit(
        &obj.unit_enum
    ));
    let serialized = serde_json::to_string(&obj).unwrap();
    assert_eq!(serialized, r#"{"unit_enum":"Bar"}"#);

    // Verify the value is correct
    assert_eq!(
        <_ as Impex<impex::DefaultWrapperSettings>>::into_value(obj.unit_enum),
        UnionEnumConfig::Bar
    );
}

#[test]
fn unit_enum_into_impex_preserves_explicit_flag() {
    use impex::{Impex, IntoImpex};

    // Create explicit
    let explicit: UnionEnumConfigImpex = <UnionEnumConfig as IntoImpex<
        impex::DefaultWrapperSettings,
    >>::into_explicit(UnionEnumConfig::Foo);
    assert!(<_ as Impex<impex::DefaultWrapperSettings>>::is_explicit(
        &explicit
    ));

    // Create implicit
    let implicit: UnionEnumConfigImpex = <UnionEnumConfig as IntoImpex<
        impex::DefaultWrapperSettings,
    >>::into_implicit(UnionEnumConfig::Bar);
    assert!(<_ as Impex<impex::DefaultWrapperSettings>>::is_implicit(
        &implicit
    ));
    assert!(!<_ as Impex<impex::DefaultWrapperSettings>>::is_explicit(
        &implicit
    ));
}

// ============================================================================
// Mixed Enum Tests (unit + non-unit variants)
// ============================================================================

#[test]
fn mixed_enum_unit_variant_implicit() {
    use impex::Impex;

    // Default is Empty (unit variant), should be implicit
    let obj: StructWithUnitEnumImpex<impex::DefaultWrapperSettings> =
        serde_json::from_str(r#"{}"#).unwrap();

    assert!(<_ as Impex<impex::DefaultWrapperSettings>>::is_implicit(
        &obj.mixed_enum
    ));

    // Should not appear in serialization
    let serialized = serde_json::to_string(&obj).unwrap();
    assert_eq!(serialized, r#"{}"#);

    // Verify default
    assert!(obj.mixed_enum == MixedEnumConfigImpex::default());
}

#[test]
fn mixed_enum_unit_variant_explicit() {
    use impex::Impex;

    // Empty variant explicitly provided - serializes as just "Empty"
    let obj: StructWithUnitEnumImpex<impex::DefaultWrapperSettings> =
        serde_json::from_str(r#"{"mixed_enum":"Empty"}"#).unwrap();

    assert!(
        <_ as Impex<impex::DefaultWrapperSettings>>::is_explicit(&obj.mixed_enum),
        "Unit variant present in JSON should be explicit"
    );

    // Should appear in serialization as just the variant name
    let serialized = serde_json::to_string(&obj).unwrap();
    assert_eq!(serialized, r#"{"mixed_enum":"Empty"}"#);
}

#[test]
fn mixed_enum_named_variant_explicit() {
    use impex::Impex;

    // Named variant with explicit value
    let text = r#"{"mixed_enum":{"Named":{"value":"hello"}}}"#;
    let obj: StructWithUnitEnumImpex<impex::DefaultWrapperSettings> =
        serde_json::from_str(text).unwrap();

    assert!(<_ as Impex<impex::DefaultWrapperSettings>>::is_explicit(
        &obj.mixed_enum
    ));

    let serialized = serde_json::to_string(&obj).unwrap();
    assert_eq!(serialized, text);
}

#[test]
fn mixed_enum_tuple_variant_explicit() {
    use impex::Impex;

    // Tuple variant with explicit value
    let text = r#"{"mixed_enum":{"Tuple":42}}"#;
    let obj: StructWithUnitEnumImpex<impex::DefaultWrapperSettings> =
        serde_json::from_str(text).unwrap();

    assert!(<_ as Impex<impex::DefaultWrapperSettings>>::is_explicit(
        &obj.mixed_enum
    ));

    let serialized = serde_json::to_string(&obj).unwrap();
    assert_eq!(serialized, text);
}

#[test]
fn mixed_enum_into_value_roundtrip() {
    use impex::{Impex, IntoImpex};

    // Test Empty variant
    let empty: MixedEnumConfigImpex<impex::DefaultWrapperSettings> =
        <MixedEnumConfig as IntoImpex<impex::DefaultWrapperSettings>>::into_explicit(
            MixedEnumConfig::Empty,
        );
    assert!(<_ as Impex<impex::DefaultWrapperSettings>>::is_explicit(
        &empty
    ));
    assert_eq!(
        <_ as Impex<impex::DefaultWrapperSettings>>::into_value(empty),
        MixedEnumConfig::Empty
    );

    // Test Named variant
    let named: MixedEnumConfigImpex<impex::DefaultWrapperSettings> =
        <MixedEnumConfig as IntoImpex<impex::DefaultWrapperSettings>>::into_explicit(
            MixedEnumConfig::Named {
                value: "test".to_string(),
            },
        );
    assert!(<_ as Impex<impex::DefaultWrapperSettings>>::is_explicit(
        &named
    ));
    assert_eq!(
        <_ as Impex<impex::DefaultWrapperSettings>>::into_value(named),
        MixedEnumConfig::Named {
            value: "test".to_string()
        }
    );

    // Test Tuple variant
    let tuple: MixedEnumConfigImpex<impex::DefaultWrapperSettings> =
        <MixedEnumConfig as IntoImpex<impex::DefaultWrapperSettings>>::into_explicit(
            MixedEnumConfig::Tuple(123),
        );
    assert!(<_ as Impex<impex::DefaultWrapperSettings>>::is_explicit(
        &tuple
    ));
    assert_eq!(
        <_ as Impex<impex::DefaultWrapperSettings>>::into_value(tuple),
        MixedEnumConfig::Tuple(123)
    );
}

// ============================================================================
// OptionImpex Tests
// ============================================================================

#[derive(Default, impex::Impex)]
pub struct OptionTestStruct {
    pub opt: Option<i32>,
}

#[derive(impex::Impex)]
pub struct OptionTestStructWithSomeDefault {
    pub opt: Option<i32>,
}

impl Default for OptionTestStructWithSomeDefault {
    fn default() -> Self {
        Self { opt: Some(42) }
    }
}

#[test]
fn option_impex_explicit_null_reserializes_as_null() {
    // Deserialize with explicit null
    let json = r#"{"opt":null}"#;
    let obj: OptionTestStructImpex<impex::DefaultWrapperSettings> =
        serde_json::from_str(json).unwrap();

    // The field should be explicit (null was present in JSON)
    assert!(obj.opt.is_explicit(), "null in JSON should be explicit");
    assert!(obj.opt.is_none(), "Value should be None");

    // When serializing back, explicit null should appear
    let serialized = serde_json::to_string(&obj).unwrap();
    assert_eq!(serialized, r#"{"opt":null}"#);
}

#[test]
fn option_impex_missing_field_is_implicit_and_not_serialized() {
    // Deserialize with missing field
    let json = r#"{}"#;
    let obj: OptionTestStructImpex<impex::DefaultWrapperSettings> =
        serde_json::from_str(json).unwrap();

    // The field should be implicit (not present in JSON)
    assert!(obj.opt.is_implicit(), "Missing field should be implicit");
    assert!(obj.opt.is_none(), "Value should be None");

    // When serializing back, implicit field should NOT appear
    let serialized = serde_json::to_string(&obj).unwrap();
    assert_eq!(serialized, r#"{}"#);
}

#[test]
fn option_impex_explicit_value_reserializes() {
    // Deserialize with explicit value
    let json = r#"{"opt":42}"#;
    let obj: OptionTestStructImpex<impex::DefaultWrapperSettings> =
        serde_json::from_str(json).unwrap();

    // The field should be explicit
    assert!(obj.opt.is_explicit(), "Value in JSON should be explicit");
    assert_eq!(**obj.opt.as_ref().unwrap(), 42);

    // When serializing back, explicit value should appear
    let serialized = serde_json::to_string(&obj).unwrap();
    assert_eq!(serialized, r#"{"opt":42}"#);
}

#[test]
fn some_option_impex_explicit_null_reserializes_as_null() {
    // Deserialize with explicit null
    let json = r#"{"opt":null}"#;
    let obj: OptionTestStructWithSomeDefaultImpex<impex::DefaultWrapperSettings> =
        serde_json::from_str(json).unwrap();

    // The field should be explicit (null was present in JSON)
    assert!(obj.opt.is_explicit(), "null in JSON should be explicit");
    assert!(obj.opt.is_none(), "Value should be None");

    // When serializing back, explicit null should appear
    let serialized = serde_json::to_string(&obj).unwrap();
    assert_eq!(serialized, r#"{"opt":null}"#);
}

#[test]
fn some_option_impex_missing_field_is_implicit_and_not_serialized() {
    // Deserialize with missing field
    let json = r#"{}"#;
    let obj: OptionTestStructWithSomeDefaultImpex<impex::DefaultWrapperSettings> =
        serde_json::from_str(json).unwrap();

    // The field should be implicit (not present in JSON)
    assert!(obj.opt.is_implicit(), "Missing field should be implicit");
    assert_eq!(
        obj.opt.as_ref().map(|x| **x),
        Some(42),
        "Value should be None"
    );

    // When serializing back, implicit field should NOT appear
    let serialized = serde_json::to_string(&obj).unwrap();
    assert_eq!(serialized, r#"{}"#);
}
