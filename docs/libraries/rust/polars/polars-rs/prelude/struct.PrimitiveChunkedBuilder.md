# Struct PrimitiveChunkedBuilder Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/builder/primitive.rs.html#4" class="src">Source</a>

``` rust
pub struct PrimitiveChunkedBuilder<T>where
    T: PolarsNumericType,{ /* private fields */ }
```

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html#impl-PrimitiveChunkedBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html#method.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<T\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html#impl-ChunkedBuilder%3C%3CT+as+PolarsNumericType%3E::Native,+T%3E-for-PrimitiveChunkedBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html" class="trait" title="trait polars::prelude::ChunkedBuilder">ChunkedBuilder</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>, T\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html#method.append_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html#tymethod.append_value" class="fn">append_value</a>(&mut self, v: \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>)

Appends a value of type `T` into the builder

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html#method.append_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html#tymethod.append_null" class="fn">append_null</a>(&mut self)

Appends a null slot into the builder

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html#tymethod.finish" class="fn">finish</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html#method.shrink_to_fit" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html#tymethod.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html#method.append_option" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedBuilder.html#method.append_option" class="fn">append_option</a>(&mut self, opt_val: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<N\>)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html#impl-Clone-for-PrimitiveChunkedBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<T\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html#blanket-implementations" class="anchor">§</a>
