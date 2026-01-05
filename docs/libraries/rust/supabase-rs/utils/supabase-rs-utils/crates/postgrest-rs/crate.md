# postgrest - Rust

Expand description

[PostgREST](https://postgrest.org/) client-side library.

This library is a thin wrapper that brings an ORM-like interface to PostgREST.

### [§](#usage)Usage

Simple example:

```
use postgrest::Postgrest;

let client = Postgrest::new("https://your.postgrest.endpoint");
let resp = client
    .from("your_table")
    .select("*")
    .execute()
    .await?;
let body = resp
    .text()
    .await?;
```

Using filters:

```
let resp = client
    .from("countries")
    .eq("name", "Germany")
    .gte("id", "20")
    .select("*")
    .execute()
    .await?;
```

Updating a table:

```
let resp = client
    .from("users")
    .eq("username", "soedirgo")
    .update("{\"organization\": \"supabase\"}")
    .execute()
    .await?;
```

Executing stored procedures:

```
let resp = client
    .rpc("add", r#"{"a": 1, "b": 2}"#)
    .execute()
    .await?;
```

Check out the [README](https://github.com/supabase/postgrest-rs) for more info.

[Builder](struct.Builder.html "struct postgrest::Builder")

QueryBuilder struct

[Postgrest](struct.Postgrest.html "struct postgrest::Postgrest")
