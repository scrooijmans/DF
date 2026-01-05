# Function new_std_io_error Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/std_io_util.rs.html#29-49" class="src">Source</a>

``` rust
pub fn new_std_io_error(err: Error) -> Error
```

Expand description

Parse std io error into opendal::Error.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.new_std_io_error.html#todo" class="doc-anchor">Â§</a>TODO

Add `NotADirectory` and `IsADirectory` once they are stable.

ref: <https://github.com/rust-lang/rust/issues/86442>
