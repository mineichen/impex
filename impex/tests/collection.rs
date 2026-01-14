use impex::{DefaultWrapperSettings, Impex};
use serde::{Deserialize, Serialize};

#[test]
fn array_forward_children() {
    #[derive(Default, impex::Impex, Deserialize, Serialize)]
    struct Test {
        foo: Option<[i32; 3]>,
        bar: Vec<u8>,
    }

    let text = r#"{"bar":[1]}"#;
    let x: TestImpex = serde_json::from_str(text).unwrap();
    assert_eq!(x.foo.as_ref(), None);
    assert_eq!(
        vec![1],
        Impex::<DefaultWrapperSettings>::into_value(x.bar.clone())
    );
    assert_eq!(text, serde_json::to_string(&x).unwrap());
}
