#[allow(unused)]
mod generated_struct;
#[allow(unused)]
mod manual_struct;

// Switch between manual and generated implementations:
// use crate::generated_struct::{
//     EnumConfig, EnumConfigImpex, KeyStructConfigImpex, TupleStructConfig, TupleStructConfigImpex,
// };
use crate::manual_struct::{
    EnumConfig, EnumConfigImpex, KeyStructConfigImpex, TupleStructConfig, TupleStructConfigImpex,
};

#[test]
fn serialize_with_defaults() {
    use impex::Impex;
    //use std::ops::{Deref, DerefMut};

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
    // *obj.sub.bar.make_defined() = "Custom".into();
    // assert_eq!("Custom", *obj.sub.bar);
    // assert_eq!("Foo", *obj.sub.foo);
    assert_eq!(
        r#"{"num_cores":3,"enum_config":{"Bar":["Custom",42,[42,43]]}}"#,
        serde_json::to_string(&obj).unwrap().as_str()
    );
}

#[test]
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
