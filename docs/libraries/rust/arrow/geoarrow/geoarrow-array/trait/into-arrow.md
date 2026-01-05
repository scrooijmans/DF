# IntoArrow in geoarrow_array - Rust

```
pub trait IntoArrow {
    type ArrowArray: Array;
    type ExtensionType: ExtensionType;

    // Required methods
    fn into_arrow(self) -> Self::ArrowArray;
    fn extension_type(&self) -> &Self::ExtensionType;
}
```

Expand description

Convert GeoArrow arrays into their respective [arrow](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/index.html "mod arrow_array") arrays.

[Source](about:blank/src/geoarrow_array/trait_.rs.html#17)

The type of arrow array that this geoarrow array can be converted into.

[Source](about:blank/src/geoarrow_array/trait_.rs.html#21)

The extension type representing this array. It will always be a type defined by [geoarrow_schema](https://docs.rs/geoarrow-schema/0.5.0/x86_64-unknown-linux-gnu/geoarrow_schema/index.html "mod geoarrow_schema").

[Source](about:blank/src/geoarrow_array/trait_.rs.html#29)

Converts this geoarrow array into an arrow array.

Note that [arrow](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/index.html "mod arrow_array") arrays do not maintain Arrow extension metadata, so the result of this method will omit any spatial extension information. Ensure you call [Self::extension_type](about:blank/trait.IntoArrow.html#tymethod.extension_type) to get extension information that you can add to a [`Field`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html "struct arrow_schema::field::Field").

[Source](about:blank/src/geoarrow_array/trait_.rs.html#32)

Return the Arrow extension type representing this array.
