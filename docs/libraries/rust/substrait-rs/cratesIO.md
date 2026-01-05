# substrait - Rust
Expand description

[Substrait](https://substrait.io/): Cross-Language Serialization for Relational Algebra

[§](#serialization-and-deserialization)Serialization and deserialization
------------------------------------------------------------------------

This crate provides generated types to serialize and deserialize Substrait data.

### [§](#protobuf)Protobuf

Protobuf serialization and deserialization are provided via [prost](https://docs.rs/prost/0.14.1/x86_64-unknown-linux-gnu/prost/index.html "mod prost") in the [proto](proto/index.html "mod substrait::proto") module.

#### [§](#example)Example

##### [§](#serialize-and-deserialize-a-plan)Serialize and deserialize a plan

```
use prost::Message;
use substrait::proto::Plan;

let plan = Plan::default();

// Serialize the plan
let encoded = plan.encode_to_vec();

// Deserialize the buffer to a Plan
let decoded = Plan::decode(encoded.as_slice())?;

assert_eq!(plan, decoded);
```


#### [§](#serde-support)Serde support

The `serde` feature generates serde implementations that match the [Protobuf JSON Mapping](https://developers.google.com/protocol-buffers/docs/proto3#json) via [pbjson](https://docs.rs/pbjson).

###### [§](#example-1)Example

###### [§](#deserialize-a-plan-version-using-the-serde-feature)Deserialize a plan version using the `serde` feature

```
use substrait::proto::Version;

let version_json = r#"{
  "minorNumber": 21
}"#;

let version = serde_json::from_str::<Version>(version_json)?;
assert_eq!(
  version,
  Version {
    minor_number: 21,
    ..Default::default()
  }
);
```


### [§](#text)Text

Substrait defines a YAML schema for extensions. Types with serialization and deserialization support for these are provided via [typify](https://docs.rs/typify) in the [text](text/index.html "mod substrait::text") module.

#### [§](#example-2)Example

##### [§](#read-a-simple-extension)Read a simple extension

```
use substrait::text::simple_extensions::SimpleExtensions;

let simple_extension_yaml = r#"
%YAML 1.2
---
urn: extension:substrait-rs:add-example
scalar_functions:
  -
    name: "add"
    description: "Add two values."
    impls:
      - args:
         - name: x
           value: i8
         - name: y
           value: i8
        options:
          overflow:
            values: [ SILENT, SATURATE, ERROR ]
        return: i8
"#;

let simple_extension = serde_yaml::from_str::<SimpleExtensions>(simple_extension_yaml)?;

assert_eq!(simple_extension.urn, "extension:substrait-rs:add-example");
assert_eq!(simple_extension.scalar_functions.len(), 1);
assert_eq!(simple_extension.scalar_functions[0].name, "add");
```


[extensions](extensions/index.html "mod substrait::extensions")`extensions`

Substrait core extensions

[parse](parse/index.html "mod substrait::parse")`parse`

Parsing of Substrait data.

[proto](proto/index.html "mod substrait::proto")

Generated types for the protobuf `substrait` package.

[text](text/index.html "mod substrait::text")

Generated types for text-based definitions.

[urn](urn/index.html "mod substrait::urn")

Extension URNs

[version](version/index.html "mod substrait::version")

Substrait version information.