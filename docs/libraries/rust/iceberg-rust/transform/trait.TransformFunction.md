# Trait TransformFunction Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/transform/mod.rs.html#32-49" class="src">Source</a>

``` rust
pub trait TransformFunction: Send + Sync {
    // Required methods
    fn transform(&self, input: ArrayRef) -> Result<ArrayRef>;
    fn transform_literal(&self, input: &Datum) -> Result<Option<Datum>>;

    // Provided method
    fn transform_literal_result(&self, input: &Datum) -> Result<Datum> { ... }
}
```

Expand description

TransformFunction is a trait that defines the interface for all transform functions.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/transform/trait.TransformFunction.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/transform/trait.TransformFunction.html#tymethod.transform" class="fn">transform</a>(&self, input: <a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/array/type.ArrayRef.html" class="type" title="type arrow_array::array::ArrayRef">ArrayRef</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/array/type.ArrayRef.html" class="type" title="type arrow_array::array::ArrayRef">ArrayRef</a>\>

transform will take an input array and transform it into a new array. The implementation of this function will need to check and downcast the input to specific type.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/transform/trait.TransformFunction.html#tymethod.transform_literal" class="fn">transform_literal</a>(&self, input: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>\>\>

transform_literal will take an input literal and transform it into a new literal.

## Provided Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/transform/trait.TransformFunction.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/transform/trait.TransformFunction.html#method.transform_literal_result" class="fn">transform_literal_result</a>(&self, input: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>\>

A thin wrapper around `transform_literal` to return an error even when it’s `None`.

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/transform/trait.TransformFunction.html#implementors" class="anchor">§</a>
