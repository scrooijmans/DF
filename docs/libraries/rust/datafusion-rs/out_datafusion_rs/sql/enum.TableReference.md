# Enum TableReference Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/table_reference.rs.html#74" class="src">Source</a>

``` rust
pub enum TableReference {
    Bare {
        table: Arc<str>,
    },
    Partial {
        schema: Arc<str>,
        table: Arc<str>,
    },
    Full {
        catalog: Arc<str>,
        schema: Arc<str>,
        table: Arc<str>,
    },
}
```

Expand description

A multi part identifier (path) to a table that may require further resolution (e.g. `foo.bar`).

[`TableReference`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html "enum datafusion::common::TableReference")s are cheap to `clone()` as they are implemented with `Arc`.

See [`ResolvedTableReference`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ResolvedTableReference.html "struct datafusion::common::ResolvedTableReference") for a fully resolved table reference.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#creating-tablereference" class="doc-anchor">§</a>Creating [`TableReference`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html "enum datafusion::common::TableReference")

When converting strings to [`TableReference`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html "enum datafusion::common::TableReference")s, the string is parsed as though it were a SQL identifier, normalizing (convert to lowercase) any unquoted identifiers. [`TableReference::bare`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html#method.bare "associated function datafusion::common::TableReference::bare") creates references without applying normalization semantics.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#examples" class="doc-anchor">§</a>Examples

``` rust
// Get a table reference to 'mytable'
let table_reference = TableReference::from("mytable");
assert_eq!(table_reference, TableReference::bare("mytable"));

// Get a table reference to 'mytable' (note the capitalization)
let table_reference = TableReference::from("MyTable");
assert_eq!(table_reference, TableReference::bare("mytable"));

// Get a table reference to 'MyTable' (note the capitalization) using double quotes
// (programmatically it is better to use `TableReference::bare` for this)
let table_reference = TableReference::from(r#""MyTable""#);
assert_eq!(table_reference, TableReference::bare("MyTable"));

// Get a table reference to 'myschema.mytable' (note the capitalization)
let table_reference = TableReference::from("MySchema.MyTable");
assert_eq!(table_reference, TableReference::partial("myschema", "mytable"));
```

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#variant.Bare" class="anchor">§</a>

### Bare

An unqualified table reference, e.g. “table”

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#variant.Bare.field.table" class="anchor field">§</a>`table: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>`>`

The table name

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#variant.Partial" class="anchor">§</a>

### Partial

A partially resolved table reference, e.g. “schema.table”

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#variant.Partial.field.schema" class="anchor field">§</a>`schema: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>`>`

The schema containing the table

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#variant.Partial.field.table" class="anchor field">§</a>`table: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>`>`

The table name

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#variant.Full" class="anchor">§</a>

### Full

A fully resolved table reference, e.g. “catalog.schema.table”

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#variant.Full.field.catalog" class="anchor field">§</a>`catalog: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>`>`

The catalog (aka database) containing the table

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#variant.Full.field.schema" class="anchor field">§</a>`schema: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>`>`

The schema containing the table

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#variant.Full.field.table" class="anchor field">§</a>`table: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>`>`

The table name

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#impl-TableReference" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.none" class="fn">none</a>() -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>

Convenience method for creating a typed none `None`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.bare" class="fn">bare</a>(table: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

Convenience method for creating a [`TableReference::Bare`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html#variant.Bare "variant datafusion::common::TableReference::Bare")

As described on [`TableReference`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html "enum datafusion::common::TableReference") this does *NO* normalization at all, so “Foo.Bar” stays as a reference to the table named “Foo.Bar” (rather than “foo”.“bar”)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.partial" class="fn">partial</a>( schema: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>, table: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

Convenience method for creating a [`TableReference::Partial`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html#variant.Partial "variant datafusion::common::TableReference::Partial").

Note: *NO* normalization is applied to the schema or table name.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.full" class="fn">full</a>( catalog: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>, schema: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>, table: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

Convenience method for creating a [`TableReference::Full`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html#variant.Full "variant datafusion::common::TableReference::Full")

Note: *NO* normalization is applied to the catalog, schema or table name.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.table" class="fn">table</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Retrieve the table name, regardless of qualification.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Retrieve the schema name if \[`Self::Partial]` or \[`Self::`Full`], `None\` otherwise.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.catalog" class="fn">catalog</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Retrieve the catalog name if [`Self::Full`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html#variant.Full "variant datafusion::common::TableReference::Full"), `None` otherwise.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.resolved_eq" class="fn">resolved_eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Compare with another [`TableReference`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html "enum datafusion::common::TableReference") as if both are resolved. This allows comparing across variants. If a field is not present in both variants being compared then it is ignored in the comparison.

e.g. this allows a [`TableReference::Bare`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html#variant.Bare "variant datafusion::common::TableReference::Bare") to be considered equal to a fully qualified [`TableReference::Full`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html#variant.Full "variant datafusion::common::TableReference::Full") if the table names match.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.resolve" class="fn">resolve</a>( self, default_catalog: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, default_schema: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ResolvedTableReference.html" class="struct" title="struct datafusion::common::ResolvedTableReference">ResolvedTableReference</a>

Given a default catalog and schema, ensure this table reference is fully resolved

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.to_quoted_string" class="fn">to_quoted_string</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Forms a string where the identifiers are quoted

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#example" class="doc-anchor">§</a>Example

``` rust
let table_reference = TableReference::partial("myschema", "mytable");
assert_eq!(table_reference.to_quoted_string(), "myschema.mytable");

let table_reference = TableReference::partial("MySchema", "MyTable");
assert_eq!(table_reference.to_quoted_string(), r#""MySchema"."MyTable""#);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.parse_str" class="fn">parse_str</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

Forms a [`TableReference`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html "enum datafusion::common::TableReference") by parsing `s` as a multipart SQL identifier. See docs on [`TableReference`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html "enum datafusion::common::TableReference") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.to_vec" class="fn">to_vec</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Decompose a [`TableReference`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html "enum datafusion::common::TableReference") to separate parts. The result vector contains at most three elements in the following sequence:

``` no_rust
[<catalog>, <schema>, table]
```

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#impl-Clone-for-TableReference" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#impl-Debug-for-TableReference" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#impl-Display-for-TableReference" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#impl-From%3C%26String%3E-for-TableReference" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&'a <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &'a <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#impl-From%3C%26str%3E-for-TableReference" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

Parse a string into a TableReference, normalizing where appropriate

See full details on [`TableReference::parse_str`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html#method.parse_str "associated function datafusion::common::TableReference::parse_str")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#impl-From%3CResolvedTableReference%3E-for-TableReference" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ResolvedTableReference.html" class="struct" title="struct datafusion::common::ResolvedTableReference">ResolvedTableReference</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(resolved: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ResolvedTableReference.html" class="struct" title="struct datafusion::common::ResolvedTableReference">ResolvedTableReference</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#impl-From%3CString%3E-for-TableReference" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#impl-Hash-for-TableReference" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#impl-Ord-for-TableReference" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#impl-PartialEq-for-TableReference" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#impl-PartialOrd-for-TableReference" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#impl-Eq-for-TableReference" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#impl-StructuralPartialEq-for-TableReference" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html#blanket-implementations" class="anchor">§</a>
