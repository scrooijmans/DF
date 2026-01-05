# Module file Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/mod.rs.html#7" class="src">Source</a>

Available on **crate feature `polars-io`** only.

## Enums<a href="https://docs.rs/polars/latest/polars/prelude/file/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html" class="enum" title="enum polars::prelude::file::AsyncWriteable">AsyncWriteable</a>  
Holds an async writeable file, abstracted over local files or cloud files.

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html" class="enum" title="enum polars::prelude::file::Writeable">Writeable</a>  
Holds a non-async writeable file, abstracted over local files or cloud files.

## Traits<a href="https://docs.rs/polars/latest/polars/prelude/file/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html" class="trait" title="trait polars::prelude::file::DynWriteable">DynWriteable</a>

## Functions<a href="https://docs.rs/polars/latest/polars/prelude/file/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/file/fn.try_get_writeable.html" class="fn" title="fn polars::prelude::file::try_get_writeable">try_get_writeable</a>  
Note: Prefer using [`Writeable`](https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html "enum polars::prelude::file::Writeable") / [`Writeable::try_new`](https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#method.try_new "associated function polars::prelude::file::Writeable::try_new") where possible.
