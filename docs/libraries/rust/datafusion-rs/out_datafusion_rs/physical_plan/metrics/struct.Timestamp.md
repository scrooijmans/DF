# Struct Timestamp Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/metrics/value.rs.html#239" class="src">Source</a>

``` rust
pub struct Timestamp { /* private fields */ }
```

Expand description

Stores a single timestamp, stored as the number of nanoseconds elapsed from Jan 1, 1970 UTC

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#impl-Timestamp" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html" class="struct" title="struct datafusion::physical_plan::metrics::Timestamp">Timestamp</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html" class="struct" title="struct datafusion::physical_plan::metrics::Timestamp">Timestamp</a>

Create a new timestamp and sets its value to 0

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#method.record" class="fn">record</a>(&self)

Sets the timestamps value to the current time

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#method.set" class="fn">set</a>(&self, now: <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/offset/utc/struct.Utc.html" class="struct" title="struct chrono::offset::utc::Utc">Utc</a>\>)

Sets the timestamps value to a specified time

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#method.value" class="fn">value</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/offset/utc/struct.Utc.html" class="struct" title="struct chrono::offset::utc::Utc">Utc</a>\>\>

return the timestamps value at the last time `record()` was called.

Returns `None` if `record()` has not been called

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#method.update_to_min" class="fn">update_to_min</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html" class="struct" title="struct datafusion::physical_plan::metrics::Timestamp">Timestamp</a>)

sets the value of this timestamp to the minimum of this and other

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#method.update_to_max" class="fn">update_to_max</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html" class="struct" title="struct datafusion::physical_plan::metrics::Timestamp">Timestamp</a>)

sets the value of this timestamp to the maximum of this and other

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#impl-Clone-for-Timestamp" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html" class="struct" title="struct datafusion::physical_plan::metrics::Timestamp">Timestamp</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html" class="struct" title="struct datafusion::physical_plan::metrics::Timestamp">Timestamp</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#impl-Debug-for-Timestamp" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html" class="struct" title="struct datafusion::physical_plan::metrics::Timestamp">Timestamp</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#impl-Default-for-Timestamp" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html" class="struct" title="struct datafusion::physical_plan::metrics::Timestamp">Timestamp</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html" class="struct" title="struct datafusion::physical_plan::metrics::Timestamp">Timestamp</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#impl-Display-for-Timestamp" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html" class="struct" title="struct datafusion::physical_plan::metrics::Timestamp">Timestamp</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#impl-PartialEq-for-Timestamp" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html" class="struct" title="struct datafusion::physical_plan::metrics::Timestamp">Timestamp</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html" class="struct" title="struct datafusion::physical_plan::metrics::Timestamp">Timestamp</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html#blanket-implementations" class="anchor">§</a>
