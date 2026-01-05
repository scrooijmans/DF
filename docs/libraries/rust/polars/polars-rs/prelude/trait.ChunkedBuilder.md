# Trait ChunkedBuilder Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/builder/mod.rs.html#30" class="src">Source</a>

``` rust
pub trait ChunkedBuilder<N, T>where
    T: PolarsDataType,{
    // Required methods
    fn append_value(&mut self, val: N);
    fn append_null(&mut self);
    fn finish(self) -> ChunkedArray<T>;
    fn shrink_to_fit(&mut self);

    // Provided method
    fn append_option(&mut self, opt_val: Option<N>) { ... }
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html#tymethod.append_value" class="fn">append_value</a>(&mut self, val: N)

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html#tymethod.append_null" class="fn">append_null</a>(&mut self)

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html#tymethod.finish" class="fn">finish</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html#tymethod.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html#method.append_option" class="fn">append_option</a>(&mut self, opt_val: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<N\>)

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html#impl-ChunkedBuilder%3Cbool,+BooleanType%3E-for-BooleanChunkedBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html" class="trait" title="trait polars::prelude::ChunkedBuilder">ChunkedBuilder</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanChunkedBuilder.html" class="struct" title="struct polars::prelude::BooleanChunkedBuilder">BooleanChunkedBuilder</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html#impl-ChunkedBuilder%3C%3CT+as+PolarsNumericType%3E::Native,+T%3E-for-PrimitiveChunkedBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html" class="trait" title="trait polars::prelude::ChunkedBuilder">ChunkedBuilder</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>, T\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,
