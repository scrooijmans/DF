# Module transform Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/transform/mod.rs.html#18-189" class="src">Source</a>

Expand description

Transform function used to compute partition values.

## Traits<a href="https://docs.rs/iceberg/0.7.0/iceberg/transform/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/transform/trait.TransformFunction.html" class="trait" title="trait iceberg::transform::TransformFunction">TransformFunction</a>  
TransformFunction is a trait that defines the interface for all transform functions.

## Functions<a href="https://docs.rs/iceberg/0.7.0/iceberg/transform/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/transform/fn.create_transform_function.html" class="fn" title="fn iceberg::transform::create_transform_function">create_transform_function</a>  
create_transform_function creates a boxed trait object of TransformFunction from a Transform.

## Type Aliases<a href="https://docs.rs/iceberg/0.7.0/iceberg/transform/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/transform/type.BoxedTransformFunction.html" class="type" title="type iceberg::transform::BoxedTransformFunction">BoxedTransformFunction</a>  
BoxedTransformFunction is a boxed trait object of TransformFunction.
