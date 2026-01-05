# Params in duckdb - Rust

```
pub trait Params: Sealed { }
```

Expand description

Trait used for [sets of parameter](https://duckdb.org/docs/stable/clients/c/prepared.html) passed into SQL statements/queries.

Note: Currently, this trait can only be implemented inside this crate. Additionally, its methods (which are `doc(hidden)`) should currently not be considered part of the stable API, although it’s possible they will stabilize in the future.

## [§](#passing-parameters-to-duckdb)Passing parameters to DuckDB

Many functions in this library let you pass parameters to DuckDB. Doing this lets you avoid any risk of SQL injection, and is simpler than escaping things manually. Aside from deprecated functions and a few helpers, this is indicated by the function taking a generic argument that implements `Params` (this trait).

### [§](#positional-parameters)Positional parameters

For cases where you want to pass a list of parameters where the number of parameters is known at compile time, this can be done in one of the following ways:

- Using the [`duckdb::params!`](macro.params.html "macro duckdb::params") macro, e.g. `thing.query(duckdb::params![1, "foo", bar])`. This is mostly useful for heterogeneous lists of parameters, or lists where the number of parameters exceeds 32.
- For small heterogeneous lists of parameters, they can either be passed as:
  - an array, as in `thing.query([1i32, 2, 3, 4])` or `thing.query(["foo", "bar", "baz"])`.
  - a reference to an array of references, as in `thing.query(&["foo", "bar", "baz"])` or `thing.query(&[&1i32, &2, &3])`.

    (Note: in this case we don’t implement this for slices for coherence reasons, so it really is only for the “reference to array” types — hence why the number of parameters must be <= 32 or you need to reach for `duckdb::params!`)

  Unfortunately, in the current design it’s not possible to allow this for references to arrays of non-references (e.g. `&[1i32, 2, 3]`). Code like this should instead either use `params!`, an array literal, a `&[&dyn ToSql]` or if none of those work, [`ParamsFromIter`](struct.ParamsFromIter.html "struct duckdb::ParamsFromIter").

- As a slice of `ToSql` trait object references, e.g. `&[&dyn ToSql]`. This is mostly useful for passing parameter lists around as arguments without having every function take a generic `P: Params`.

#### [§](#example-positional)Example (positional)

```
fn update_rows(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("INSERT INTO test (a, b) VALUES (?, ?)")?;

    // Using `duckdb::params!`:
    stmt.execute(params![1i32, "blah"])?;

    // array literal — non-references
    stmt.execute([2i32, 3i32])?;

    // array literal — references
    stmt.execute(["foo", "bar"])?;

    // Slice literal, references:
    stmt.execute(&[&2i32, &3i32])?;

    // Note: The types behind the references don't have to be `Sized`
    stmt.execute(&["foo", "bar"])?;

    // However, this doesn't work (see above):
    // stmt.execute(&[1i32, 2i32])?;
    Ok(())
}
```

### [§](#no-parameters)No parameters

You can just use an empty array literal for no params. The `duckdb::NO_PARAMS` constant which was so common in previous versions of this library is no longer needed (and is now deprecated).

#### [§](#example-no-parameters)Example (no parameters)

```
fn delete_all_users(conn: &Connection) -> Result<()> {
    // Just use an empty array (e.g. `[]`) for no params.
    conn.execute("DELETE FROM users", [])?;
    Ok(())
}
```

### [§](#dynamic-parameter-list)Dynamic parameter list

If you have a number of parameters which is unknown at compile time (for example, building a dynamic query at runtime), you have two choices:

- Use a `&[&dyn ToSql]`, which is nice if you have one otherwise might be annoying.
- Use the [`ParamsFromIter`](struct.ParamsFromIter.html "struct duckdb::ParamsFromIter") type. This essentially lets you wrap an iterator some `T: ToSql` with something that implements `Params`.

A lot of the considerations here are similar either way, so you should see the [`ParamsFromIter`](struct.ParamsFromIter.html "struct duckdb::ParamsFromIter") documentation for more info / examples.

[Source](about:blank/src/duckdb/params.rs.html#145-150)
[§](#impl-Params-for-%26%5B%26dyn+ToSql%5D)

[Source](about:blank/src/duckdb/params.rs.html#134-142)
[§](#impl-Params-for-%5B%26dyn+ToSql;+0%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+1%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+2%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+3%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+4%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+5%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+6%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+7%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+8%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+9%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+10%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+11%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+12%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+13%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+14%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+15%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+16%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+17%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+18%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+19%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+20%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+21%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+22%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+23%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+24%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+25%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+26%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+27%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+29%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+30%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+31%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%26%5B%26T;+32%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+1%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+2%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+3%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+4%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+5%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+6%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+7%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+8%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+9%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+10%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+11%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+12%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+13%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+14%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+15%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+16%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+17%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+18%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+19%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+20%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+21%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+22%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+23%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+24%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+25%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+26%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+27%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+29%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+30%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+31%5D)

[Source](about:blank/src/duckdb/params.rs.html#175-178)
[§](#impl-Params-for-%5BT;+32%5D)
