# Function finish_message_buffer Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/Message.rs.html#1487-1490" class="src">Source</a>

``` rust
pub fn finish_message_buffer<'a, 'b, A>(
    fbb: &'b mut FlatBufferBuilder<'a, A>,
    root: WIPOffset<Message<'a>>,
)where
    A: Allocator + 'a,
```
