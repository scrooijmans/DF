# Function root_as_message_with_opts Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/Message.rs.html#1453-1456" class="src">Source</a>

``` rust
pub fn root_as_message_with_opts<'b, 'o>(
    opts: &'o VerifierOptions,
    buf: &'b [u8],
) -> Result<Message<'b>, InvalidFlatbuffer>
```

Expand description

Verifies, with the given options, that a buffer of bytes contains a `Message` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_message_unchecked`.
