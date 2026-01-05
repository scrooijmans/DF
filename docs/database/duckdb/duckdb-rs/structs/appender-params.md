# AppenderParamsFromIter in duckdb - Rust

## Struct AppenderParamsFromIter 

[Source](about:blank/src/duckdb/appender_params.rs.html#276)

```
pub struct AppenderParamsFromIter<I>(/* private fields */);
```

Expand description

Adapter type which allows any iterator over [`ToSql`](types/trait.ToSql.html "trait duckdb::types::ToSql") values to implement \[`Params`\].

This struct is created by the \[`params_from_iter`\] function.

This can be useful if you have something like an `&[String]` (of unknown length), and you want to use them with an API that wants something implementing `Params`. This way, you can avoid having to allocate storage for something like a `&[&dyn ToSql]`.

This essentially is only ever actually needed when dynamically generating SQL — static SQL (by definition) has the number of parameters known statically. As dynamically generating SQL is itself pretty advanced, this API is itself for advanced use cases (See “Realistic use case” in the examples).

## [§](#example)Example

### [§](#basic-usage)Basic usage

```
use duckdb::{Connection, Result, params_from_iter};
use std::collections::BTreeSet;

fn query(conn: &Connection, ids: &BTreeSet<String>) -> Result<()> {
    assert_eq!(ids.len(), 3, "Unrealistic sample code");

    let mut stmt = conn.prepare("SELECT * FROM users WHERE id IN (?, ?, ?)")?;
    let _rows = stmt.query(params_from_iter(ids.iter()))?;

    // use _rows...
    Ok(())
}
```

### [§](#realistic-use-case)Realistic use case

Here’s how you’d use `ParamsFromIter` to call \[`Statement::exists`\] with a dynamic number of parameters.

```
use duckdb::{Connection, Result};

pub fn any_active_users(conn: &Connection, usernames: &[String]) -> Result<bool> {
    if usernames.is_empty() {
        return Ok(false);
    }

    // Note: `repeat_vars` never returns anything attacker-controlled, so
    // it's fine to use it in a dynamically-built SQL string.
    let vars = repeat_vars(usernames.len());

    let sql = format!(
        // In practice this would probably be better as an `EXISTS` query.
        "SELECT 1 FROM user WHERE is_active AND name IN ({}) LIMIT 1",
        vars,
    );
    let mut stmt = conn.prepare(&sql)?;
    stmt.exists(duckdb::params_from_iter(usernames))
}

// Helper function to return a comma-separated sequence of `?`.
// - `repeat_vars(0) => panic!(...)`
// - `repeat_vars(1) => "?"`
// - `repeat_vars(2) => "?,?"`
// - `repeat_vars(3) => "?,?,?"`
// - ...
fn repeat_vars(count: usize) -> String {
    assert_ne!(count, 0);
    let mut s = "?,".repeat(count);
    // Remove trailing comma
    s.pop();
    s
}
```

That is fairly complex, and even so would need even more work to be fully production-ready:

- production code should ensure `usernames` isn’t so large that it will surpass [`conn.limit(Limit::SQLITE_LIMIT_VARIABLE_NUMBER)`](crate::Connection::limit)), chunking if too large. (Note that the limits api requires duckdb to have the “limits” feature).
- `repeat_vars` can be implemented in a way that avoids needing to allocate a String.
- Etc…

This complexity reflects the fact that `ParamsFromIter` is mainly intended for advanced use cases — most of the time you should know how many parameters you have statically (and if you don’t, you’re either doing something tricky, or should take a moment to think about the design).
