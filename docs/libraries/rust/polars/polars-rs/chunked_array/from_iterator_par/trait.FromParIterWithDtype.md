# Trait FromParIterWithDtype Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/from_iterator_par.rs.html#143" class="src">Source</a>

``` rust
pub trait FromParIterWithDtype<K> {
    // Required method
    fn from_par_iter_with_dtype<I>(
        iter: I,
        name: PlSmallStr,
        dtype: DataType,
    ) -> Self
       where I: IntoParallelIterator<Item = K>,
             Self: Sized;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.FromParIterWithDtype.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.FromParIterWithDtype.html#tymethod.from_par_iter_with_dtype" class="fn">from_par_iter_with_dtype</a>\<I\>( iter: I, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> Self

where I: <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\<Item = K\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

## Implementations on Foreign Types<a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.FromParIterWithDtype.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.FromParIterWithDtype.html#impl-FromParIterWithDtype%3CResult%3CT,+E%3E%3E-for-Result%3CC,+E%3E" class="anchor">§</a>

### impl\<C, T, E\> <a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.FromParIterWithDtype.html" class="trait" title="trait polars::chunked_array::from_iterator_par::FromParIterWithDtype">FromParIterWithDtype</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, E\>\> for <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<C, E\>

where C: <a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.FromParIterWithDtype.html" class="trait" title="trait polars::chunked_array::from_iterator_par::FromParIterWithDtype">FromParIterWithDtype</a>\<T\>, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, E: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.FromParIterWithDtype.html#method.from_par_iter_with_dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.FromParIterWithDtype.html#tymethod.from_par_iter_with_dtype" class="fn">from_par_iter_with_dtype</a>\<I\>( par_iter: I, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<C, E\>

where I: <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, E\>\>,

## Implementors<a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.FromParIterWithDtype.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.FromParIterWithDtype.html#impl-FromParIterWithDtype%3COption%3CSeries%3E%3E-for-ChunkedArray%3CListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.FromParIterWithDtype.html" class="trait" title="trait polars::chunked_array::from_iterator_par::FromParIterWithDtype">FromParIterWithDtype</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>
