# Function create_transform_function Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/transform/mod.rs.html#55-70" class="src">Source</a>

``` rust
pub fn create_transform_function(
    transform: &Transform,
) -> Result<BoxedTransformFunction>
```

Expand description

create_transform_function creates a boxed trait object of TransformFunction from a Transform.
