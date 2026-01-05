# Struct MetricsAccessor Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/observe/metrics.rs.html#541-545" class="src">Source</a>

``` rust
pub struct MetricsAccessor<A: Access, I: MetricsIntercept> { /* private fields */ }
```

Expand description

The metrics accessor for opendal.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#impl-Debug-for-MetricsAccessor%3CA,+I%3E" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>, I: <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/trait.MetricsIntercept.html" class="trait" title="trait opendal::layers::observe::MetricsIntercept">MetricsIntercept</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html" class="struct" title="struct opendal::layers::observe::MetricsAccessor">MetricsAccessor</a>\<A, I\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#impl-LayeredAccess-for-MetricsAccessor%3CA,+I%3E" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>, I: <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/trait.MetricsIntercept.html" class="trait" title="trait opendal::layers::observe::MetricsIntercept">MetricsIntercept</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html" class="trait" title="trait opendal::raw::LayeredAccess">LayeredAccess</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html" class="struct" title="struct opendal::layers::observe::MetricsAccessor">MetricsAccessor</a>\<A, I\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#associatedtype.Inner" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Inner" class="associatedtype">Inner</a> = A

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#associatedtype.Reader" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Reader" class="associatedtype">Reader</a> = MetricsWrapper\<\<A as <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader" class="associatedtype" title="type opendal::raw::Access::Reader">Reader</a>, I\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#associatedtype.Writer" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Writer" class="associatedtype">Writer</a> = MetricsWrapper\<\<A as <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer" class="associatedtype" title="type opendal::raw::Access::Writer">Writer</a>, I\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#associatedtype.Lister" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Lister" class="associatedtype">Lister</a> = MetricsWrapper\<\<A as <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister" class="associatedtype" title="type opendal::raw::Access::Lister">Lister</a>, I\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#associatedtype.Deleter" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Deleter" class="associatedtype">Deleter</a> = MetricsWrapper\<\<A as <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter" class="associatedtype" title="type opendal::raw::Access::Deleter">Deleter</a>, I\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#method.inner" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#tymethod.inner" class="fn">inner</a>(&self) -\> &Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Inner" class="associatedtype" title="type opendal::raw::LayeredAccess::Inner">Inner</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#method.create_dir" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#method.create_dir" class="fn">create_dir</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpCreateDir.html" class="struct" title="struct opendal::raw::OpCreateDir">OpCreateDir</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpCreateDir.html" class="struct" title="struct opendal::raw::RpCreateDir">RpCreateDir</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#method.read" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#tymethod.read" class="fn">read</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html" class="struct" title="struct opendal::raw::RpRead">RpRead</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Reader" class="associatedtype" title="type opendal::raw::LayeredAccess::Reader">Reader</a>)\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#method.write" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#tymethod.write" class="fn">write</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpWrite.html" class="struct" title="struct opendal::raw::RpWrite">RpWrite</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Writer" class="associatedtype" title="type opendal::raw::LayeredAccess::Writer">Writer</a>)\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#method.copy" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#method.copy" class="fn">copy</a>(&self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpCopy.html" class="struct" title="struct opendal::raw::OpCopy">OpCopy</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpCopy.html" class="struct" title="struct opendal::raw::RpCopy">RpCopy</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#method.rename" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#method.rename" class="fn">rename</a>(&self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRename.html" class="struct" title="struct opendal::raw::OpRename">OpRename</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRename.html" class="struct" title="struct opendal::raw::RpRename">RpRename</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#method.stat" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#method.stat" class="fn">stat</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpStat.html" class="struct" title="struct opendal::raw::OpStat">OpStat</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpStat.html" class="struct" title="struct opendal::raw::RpStat">RpStat</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#method.delete" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#tymethod.delete" class="fn">delete</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpDelete.html" class="struct" title="struct opendal::raw::RpDelete">RpDelete</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Deleter" class="associatedtype" title="type opendal::raw::LayeredAccess::Deleter">Deleter</a>)\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#method.list" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#tymethod.list" class="fn">list</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html" class="struct" title="struct opendal::raw::OpList">OpList</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpList.html" class="struct" title="struct opendal::raw::RpList">RpList</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Lister" class="associatedtype" title="type opendal::raw::LayeredAccess::Lister">Lister</a>)\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#method.presign" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#method.presign" class="fn">presign</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html" class="struct" title="struct opendal::raw::OpPresign">OpPresign</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html" class="struct" title="struct opendal::raw::RpPresign">RpPresign</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#method.info" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#method.info" class="fn">info</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>\>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html#blanket-implementations" class="anchor">Â§</a>
