# Trait IndexToUsize Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/legacy/index.rs.html#9" class="src">Source</a>

``` rust
pub trait IndexToUsize: Display {
    // Required method
    fn negative_to_usize(self, len: usize) -> Option<usize>;

    // Provided method
    fn try_negative_to_usize(self, len: usize) -> Result<usize, PolarsError>
       where Self: Sized + Copy { ... }
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.IndexToUsize.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IndexToUsize.html#tymethod.negative_to_usize" class="fn">negative_to_usize</a>(self, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Translate the negative index to an offset.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.IndexToUsize.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IndexToUsize.html#method.try_negative_to_usize" class="fn">try_negative_to_usize</a>(self, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.IndexToUsize.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.IndexToUsize.html#impl-IndexToUsize-for-I" class="anchor">§</a>

### impl\<I\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.IndexToUsize.html" class="trait" title="trait polars::prelude::IndexToUsize">IndexToUsize</a> for I

where I: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/sign/trait.Signed.html" class="trait" title="trait num_traits::sign::Signed">Signed</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/identities/trait.Zero.html" class="trait" title="trait num_traits::identities::Zero">Zero</a> + <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a>,
