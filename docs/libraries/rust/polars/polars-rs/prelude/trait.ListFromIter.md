# Trait ListFromIter Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/legacy/array/mod.rs.html#46" class="src">Source</a>

``` rust
pub trait ListFromIter {
    // Provided methods
    unsafe fn from_iter_primitive_trusted_len<T, P, I>(
        iter: I,
        dtype: ArrowDataType,
    ) -> ListArray<i64>
       where T: NativeType,
             P: IntoIterator<Item = Option<T>>,
             I: IntoIterator<Item = Option<P>> { ... }
    unsafe fn from_iter_bool_trusted_len<I, P>(iter: I) -> ListArray<i64>
       where I: IntoIterator<Item = Option<P>>,
             P: IntoIterator<Item = Option<bool>> { ... }
    unsafe fn from_iter_binview_trusted_len<I, P, Ref, T>(
        iter: I,
        n_elements: usize,
    ) -> ListArray<i64>
       where T: ViewType + ?Sized,
             I: IntoIterator<Item = Option<P>>,
             P: IntoIterator<Item = Option<Ref>>,
             Ref: AsRef<T> { ... }
    unsafe fn from_iter_utf8_trusted_len<I, P, Ref>(
        iter: I,
        n_elements: usize,
    ) -> ListArray<i64>
       where I: IntoIterator<Item = Option<P>>,
             P: IntoIterator<Item = Option<Ref>>,
             Ref: AsRef<str> { ... }
    unsafe fn from_iter_binary_trusted_len<I, P, Ref>(
        iter: I,
        n_elements: usize,
    ) -> ListArray<i64>
       where I: IntoIterator<Item = Option<P>>,
             P: IntoIterator<Item = Option<Ref>>,
             Ref: AsRef<[u8]> { ... }
}
```

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ListFromIter.html#provided-methods" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListFromIter.html#method.from_iter_primitive_trusted_len" class="fn">from_iter_primitive_trusted_len</a>\<T, P, I\>( iter: I, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, ) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/list/struct.ListArray.html" class="struct" title="struct polars_arrow::array::list::ListArray">ListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

where T: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/types/native/trait.NativeType.html" class="trait" title="trait polars_arrow::types::native::NativeType">NativeType</a>, P: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>\>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<P\>\>,

Create a list-array from an iterator. Used in group_by agg-list

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListFromIter.html#safety" class="doc-anchor">§</a>Safety

Will produce incorrect arrays if size hint is incorrect.

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListFromIter.html#method.from_iter_bool_trusted_len" class="fn">from_iter_bool_trusted_len</a>\<I, P\>(iter: I) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/list/struct.ListArray.html" class="struct" title="struct polars_arrow::array::list::ListArray">ListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<P\>\>, P: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\>,

Create a list-array from an iterator. Used in group_by agg-list

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListFromIter.html#safety-1" class="doc-anchor">§</a>Safety

Will produce incorrect arrays if size hint is incorrect.

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListFromIter.html#method.from_iter_binview_trusted_len" class="fn">from_iter_binview_trusted_len</a>\<I, P, Ref, T\>( iter: I, n_elements: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/list/struct.ListArray.html" class="struct" title="struct polars_arrow::array::list::ListArray">ListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

where T: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binview/trait.ViewType.html" class="trait" title="trait polars_arrow::array::binview::ViewType">ViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<P\>\>, P: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Ref\>\>, Ref: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<T\>,

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListFromIter.html#safety-2" class="doc-anchor">§</a>Safety

Will produce incorrect arrays if size hint is incorrect.

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListFromIter.html#method.from_iter_utf8_trusted_len" class="fn">from_iter_utf8_trusted_len</a>\<I, P, Ref\>( iter: I, n_elements: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/list/struct.ListArray.html" class="struct" title="struct polars_arrow::array::list::ListArray">ListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<P\>\>, P: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Ref\>\>, Ref: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>,

Create a list-array from an iterator. Used in group_by agg-list

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListFromIter.html#safety-3" class="doc-anchor">§</a>Safety

Will produce incorrect arrays if size hint is incorrect.

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListFromIter.html#method.from_iter_binary_trusted_len" class="fn">from_iter_binary_trusted_len</a>\<I, P, Ref\>( iter: I, n_elements: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/list/struct.ListArray.html" class="struct" title="struct polars_arrow::array::list::ListArray">ListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<P\>\>, P: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Ref\>\>, Ref: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>,

Create a list-array from an iterator. Used in group_by agg-list

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListFromIter.html#safety-4" class="doc-anchor">§</a>Safety

Will produce incorrect arrays if size hint is incorrect.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.ListFromIter.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ListFromIter.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ListFromIter.html#impl-ListFromIter-for-ListArray%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListFromIter.html" class="trait" title="trait polars::prelude::ListFromIter">ListFromIter</a> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/list/struct.ListArray.html" class="struct" title="struct polars_arrow::array::list::ListArray">ListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>
