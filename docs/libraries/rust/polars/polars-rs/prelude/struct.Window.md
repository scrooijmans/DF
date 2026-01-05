# Struct Window Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/windows/window.rs.html#44" class="src">Source</a>

``` rust
pub struct Window {
    pub offset: Duration,
    /* private fields */
}
```

Available on **crate feature `temporal`** only.

Expand description

Represents a window in time

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#structfield.offset" class="anchor field">§</a>`offset: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration"><code>Duration</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#impl-Window" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html" class="struct" title="struct polars::prelude::Window">Window</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#method.new" class="fn">new</a>(every: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration">Duration</a>, period: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration">Duration</a>, offset: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration">Duration</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html" class="struct" title="struct polars::prelude::Window">Window</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#method.truncate_ns" class="fn">truncate_ns</a>(&self, t: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Truncate the given ns timestamp by the window boundary.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#method.truncate_us" class="fn">truncate_us</a>(&self, t: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Truncate the given us timestamp by the window boundary.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#method.truncate_ms" class="fn">truncate_ms</a>(&self, t: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Truncate the given ms timestamp by the window boundary.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#method.round_ns" class="fn">round_ns</a>(&self, t: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Round the given ns timestamp by the window boundary.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#method.round_us" class="fn">round_us</a>(&self, t: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Round the given us timestamp by the window boundary.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#method.round_ms" class="fn">round_ms</a>(&self, t: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Round the given ms timestamp by the window boundary.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#method.get_earliest_bounds_ns" class="fn">get_earliest_bounds_ns</a>( &self, t: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, closed_window: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ClosedWindow.html" class="enum" title="enum polars::prelude::ClosedWindow">ClosedWindow</a>, tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Bounds.html" class="struct" title="struct polars::prelude::Bounds">Bounds</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

returns the bounds for the earliest window bounds that contains the given time t. For underlapping windows that do not contain time t, the window directly after time t will be returned.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#method.get_earliest_bounds_us" class="fn">get_earliest_bounds_us</a>( &self, t: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, closed_window: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ClosedWindow.html" class="enum" title="enum polars::prelude::ClosedWindow">ClosedWindow</a>, tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Bounds.html" class="struct" title="struct polars::prelude::Bounds">Bounds</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#method.get_earliest_bounds_ms" class="fn">get_earliest_bounds_ms</a>( &self, t: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, closed_window: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ClosedWindow.html" class="enum" title="enum polars::prelude::ClosedWindow">ClosedWindow</a>, tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Bounds.html" class="struct" title="struct polars::prelude::Bounds">Bounds</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#method.get_overlapping_bounds_iter" class="fn">get_overlapping_bounds_iter</a>\<'a\>( &'a self, boundary: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Bounds.html" class="struct" title="struct polars::prelude::Bounds">Bounds</a>, closed_window: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ClosedWindow.html" class="enum" title="enum polars::prelude::ClosedWindow">ClosedWindow</a>, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>, tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>\>, start_by: <a href="https://docs.rs/polars/latest/polars/prelude/enum.StartBy.html" class="enum" title="enum polars::prelude::StartBy">StartBy</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BoundsIter.html" class="struct" title="struct polars::prelude::BoundsIter">BoundsIter</a>\<'a\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#impl-Clone-for-Window" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html" class="struct" title="struct polars::prelude::Window">Window</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html" class="struct" title="struct polars::prelude::Window">Window</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#impl-Copy-for-Window" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html" class="struct" title="struct polars::prelude::Window">Window</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.Window.html#blanket-implementations" class="anchor">§</a>
