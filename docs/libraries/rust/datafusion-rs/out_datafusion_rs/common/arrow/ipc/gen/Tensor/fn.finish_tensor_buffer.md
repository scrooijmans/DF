# Function finish_tensor_buffer Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/Tensor.rs.html#1195-1198" class="src">Source</a>

``` rust
pub fn finish_tensor_buffer<'a, 'b, A>(
    fbb: &'b mut FlatBufferBuilder<'a, A>,
    root: WIPOffset<Tensor<'a>>,
)where
    A: Allocator + 'a,
```
