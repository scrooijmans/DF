
Title: SQLite - Rust Cookbook

Description: Collection of useful Rust code examples

*   Light
*   Rust
*   Coal
*   Navy
*   Ayu

Rust Cookbook
=============

SQLite
======

Create a SQLite database
------------------------

Use the `rusqlite` crate to open SQLite databases. See crate for compiling on Windows.

`Connection::open` will create the database if it doesn't already exist.

```rust
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
let conn = Connection::open("cats.db")?;

conn.execute(
"create table if not exists cat_colors (
id integer primary key,
name text not null unique
)",
(),
)?;
conn.execute(
"create table if not exists cats (
id integer primary key,
name text not null,
color_id integer not null references cat_colors(id)
)",
(),
)?;

Ok(())
}
```

Insert and Select data
----------------------

`Connection::open` will open the database `cats` created in the earlier recipe. This recipe inserts data into `cat_colors` and `cats` tables using the `execute` method of `Connection`. First, the data is inserted into the `cat_colors` table. After a record for a color is inserted, `last_insert_rowid` method of `Connection` is used to get `id` of the last color inserted. This `id` is used while inserting data into the `cats` table. Then, the select query is prepared using the `prepare` method which gives a `statement` struct. Then, query is executed using `query_map` method of `statement`.

```rust
use rusqlite::{params, Connection, Result};
use std::collections::HashMap;

#[derive(Debug)]
struct Cat {
name: String,
color: String,
}

fn main() -> Result<()> {
let conn = Connection::open("cats.db")?;

let mut cat_colors = HashMap::new();
cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

for (color, catnames) in &cat_colors {
conn.execute(
"INSERT INTO cat_colors (name) VALUES (?1)",
[color],
)?;
let last_id = conn.last_insert_rowid();

for cat in catnames {
conn.execute(
"INSERT INTO cats (name, color_id) values (?1, ?2)",
params![cat, last_id],
)?;
}
}
let mut stmt = conn.prepare(
"SELECT c.name, cc.name FROM cats c
INNER JOIN cat_colors cc
ON cc.id = c.color_id;",
)?;

let cats = stmt.query_map([], |row| {
Ok(Cat {
name: row.get(0)?,
color: row.get(1)?,
})
})?;

for cat in cats {
if let Ok(found_cat) = cat {
println!(
"Found cat {:?} {} is {}", 
found_cat,
found_cat.name,
found_cat.color,
);
}
}

Ok(())
}
```

Using transactions
------------------

`Connection::open` will open the `cats.db` database from the top recipe.

Begin a transaction with `Connection::transaction`. Transactions will roll back unless committed explicitly with `Transaction::commit`.

In the following example, colors add to a table having a unique constraint on the color name. When an attempt to insert a duplicate color is made, the transaction rolls back.

```rust
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
let mut conn = Connection::open("cats.db")?;

successful_tx(&mut conn)?;

let res = rolled_back_tx(&mut conn);
assert!(res.is_err());

Ok(())
}

fn successful_tx(conn: &mut Connection) -> Result<()> {
let tx = conn.transaction()?;

tx.execute("delete from cat_colors", [])?;
tx.execute("insert into cat_colors (name) values (?1)", ["lavender"])?;
tx.execute("insert into cat_colors (name) values (?1)", ["blue"])?;

tx.commit()
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
let tx = conn.transaction()?;

tx.execute("delete from cat_colors", [])?;
tx.execute("insert into cat_colors (name) values (?1)", ["lavender"])?;
tx.execute("insert into cat_colors (name) values (?1)", ["blue"])?;
tx.execute("insert into cat_colors (name) values (?1)", ["lavender"])?;

tx.commit()
}
```