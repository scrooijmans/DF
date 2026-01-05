# Enum PlPathRef Copy item path

<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/src/polars_utils/plpath.rs.html#17" class="src">Source</a>

``` rust
pub enum PlPathRef<'a> {
    Local(&'a Path),
    Cloud(PlCloudPathRef<'a>),
}
```

Expand description

A reference to a Path or URI

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#variant.Local" class="anchor">§</a>

### Local(&'a <a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#variant.Cloud" class="anchor">§</a>

### Cloud(<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/plpath/struct.PlCloudPathRef.html" class="struct" title="struct polars_utils::plpath::PlCloudPathRef">PlCloudPathRef</a>\<'a\>)

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#impl-PlPathRef%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.scheme" class="fn">scheme</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/plpath/enum.CloudScheme.html" class="enum" title="enum polars_utils::plpath::CloudScheme">CloudScheme</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.is_local" class="fn">is_local</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.is_cloud_url" class="fn">is_cloud_url</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.as_local_path" class="fn">as_local_path</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.as_cloud_addr" class="fn">as_cloud_addr</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/plpath/struct.PlCloudPathRef.html" class="struct" title="struct polars_utils::plpath::PlCloudPathRef">PlCloudPathRef</a>\<'\_\>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.join" class="fn">join</a>(&self, other: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.display" class="fn">display</a>(&self) -\> <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/plpath/struct.AddressDisplay.html" class="struct" title="struct polars_utils::plpath::AddressDisplay">AddressDisplay</a>\<'\_\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.from_local_path" class="fn">from_local_path</a>(path: &'a <a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.new" class="fn">new</a>(uri: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.into_owned" class="fn">into_owned</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.strip_scheme" class="fn">strip_scheme</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.parent" class="fn">parent</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.extension" class="fn">extension</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.to_str" class="fn">to_str</a>(&self) -\> &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.offset_bytes" class="fn">offset_bytes</a>(&'a self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf">PathBuf</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#impl-Clone-for-PlPathRef%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#impl-Debug-for-PlPathRef%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#impl-Hash-for-PlPathRef%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#impl-Ord-for-PlPathRef%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1023-1025" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1062-1064" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1088-1090" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#impl-PartialEq-for-PlPathRef%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#impl-PartialOrd-for-PlPathRef%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1399" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1417" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1435" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1453" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#impl-Copy-for-PlPathRef%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#impl-Eq-for-PlPathRef%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#impl-StructuralPartialEq-for-PlPathRef%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html#blanket-implementations" class="anchor">§</a>
