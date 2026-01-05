# Module registry Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/object/mod.rs.html#18" class="src">Source</a>

Expand description

This is a heap allocated utility that can be used to register an object type.

That object type will know its own generic type parameter `T` and callers can simply send `&Any` values and don’t have to know the generic type themselves.

## Structs<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/struct.ObjectRegistry.html" class="struct" title="struct polars::chunked_array::object::registry::ObjectRegistry">ObjectRegistry</a>

## Traits<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html" class="trait" title="trait polars::chunked_array::object::registry::AnonymousObjectBuilder">AnonymousObjectBuilder</a>  
This trait can be registered, after which that global registration can be used to materialize object types

## Functions<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/fn.get_object_builder.html" class="fn" title="fn polars::chunked_array::object::registry::get_object_builder">get_object_builder</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/fn.get_object_converter.html" class="fn" title="fn polars::chunked_array::object::registry::get_object_converter">get_object_converter</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/fn.get_object_physical_type.html" class="fn" title="fn polars::chunked_array::object::registry::get_object_physical_type">get_object_physical_type</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/fn.get_pyobject_converter.html" class="fn" title="fn polars::chunked_array::object::registry::get_pyobject_converter">get_pyobject_converter</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/fn.register_object_builder.html" class="fn" title="fn polars::chunked_array::object::registry::register_object_builder">register_object_builder</a>

## Type Aliases<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/type.BuilderConstructor.html" class="type" title="type polars::chunked_array::object::registry::BuilderConstructor">BuilderConstructor</a>

Takes a `name` and `capacity` and constructs a new builder.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/type.ObjectConverter.html" class="type" title="type polars::chunked_array::object::registry::ObjectConverter">ObjectConverter</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/type.PyObjectConverter.html" class="type" title="type polars::chunked_array::object::registry::PyObjectConverter">PyObjectConverter</a>
