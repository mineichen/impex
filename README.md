# Impex (short for Implicit/Explicit)

```rust
use serde::{Deserialize, Serialize};
use impex::Impex;

#[derive(Debug, Impex, Serialize, Deserialize, Default)]
pub struct MyStruct {
    foo: i32,
    bar: String,
}

let input = r#"{"foo":42}"#;
let value: ImpexMyStruct = serde_json::from_str(input).unwrap();
let output = serde_json::to_string(&value).unwrap();

assert_eq!(*value.foo, 42);
assert_eq!(value.bar.as_str(), "");
assert_eq!(input, output);

```

Impex allows struct/enum fields to have metadata about wether they are implicit or explicit using Rust macros.
It solves the usecase to load partially defined data from a json (taking the rest from Default::default), using it and then saving it back without persisting default values.


## Status
This repo is still considered immature and breaking changes are still expected.

Thank you for all the inputs during the impl day at EuroRust in Paris. Feel welcome to participate.


