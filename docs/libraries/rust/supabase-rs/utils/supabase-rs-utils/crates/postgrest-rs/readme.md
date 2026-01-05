# GitHub - supabase-community/postgrest-rs: Rust client for PostgREST

## postgrest-rs

[](#postgrest-rs)

[![Build](https://github.com/supabase/postgrest-rs/workflows/CI/badge.svg)](https://github.com/supabase/postgrest-rs/actions?query=branch%3Amaster) [![Crate](https://camo.githubusercontent.com/4ebdf0d2b61183a09e1ee048836d2509d55d5b63fa8466be3a89faa7b85b931d/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f762f706f737467726573742e737667)](https://crates.io/crates/postgrest) [![API](https://camo.githubusercontent.com/863599c216db99e642befb983b27c0cb8757a1e7db1ff45ed365fc0194741507/68747470733a2f2f646f63732e72732f706f737467726573742f62616467652e737667)](https://docs.rs/postgrest) [![License: Apache-2.0 OR MIT](https://camo.githubusercontent.com/e87e70667c0fe21ac8d6e822aa96947d89a098a506694851613f3b47deb7b8d0/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f6c2f706f737467726573742e737667)](#license)

[PostgREST](https://postgrest.org/) client-side library 🦀. This library provides an ORM interface to PostgREST.

## Usage

[](#usage)

Add this to your `Cargo.toml`:

```
[dependencies]
postgrest = "1.0"
```

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

Simple example with JWT auth

```
use postgrest::Postgrest;

let client = Postgrest::new("https://your.postgrest.endpoint");
let resp = client
    .from("your_table")
    .auth("VerySensitiveJWTValueAsStringOrStr")
    .select("*")
    .execute()
    .await?;
let body = resp
    .text()
    .await?;
```

Simplified example using a custom header (e.g. for API gateway authentication with Supabase).

```
use postgrest::Postgrest;

let client = Postgrest::new("https://your.supabase.endpoint/rest/v1")
    .insert_header("apikey", "ExampleAPIKeyValue"); // EXAMPLE ONLY!
// Don't actually hard code this value, that's really bad. Use environment
// variables like with the dotenv(https://crates.io/crates/dotenv) crate to inject
let resp = client
    .from("your_table")
    .select("*")
    .execute()
    .await?;
let body = resp
    .text()
    .await?;
```

**Secure** example with authenticated API gateway using the dotenv crate to correctly retrieve sensitive values.

```
use postgrest::Postgrest;
use dotenv;

dotenv::dotenv().ok();

let client = Postgrest::new("https://your.supabase.endpoint/rest/v1")
    .insert_header(
        "apikey",
        dotenv::var("SUPABASE_PUBLIC_API_KEY").unwrap())
let resp = client
    .from("your_table")
    .select("*")
    .execute()
    .await?;
let body = resp
    .text()
    .await?;
```

if you have RLS enabled you're required to add an extra header for this libary to function correctly.

```
use postgrest::Postgrest;
use dotenv;

dotenv::dotenv().ok();

let client = Postgrest::new("https://your.supabase.endpoint/rest/v1/")
    .insert_header(
        "apikey",
        dotenv::var("SUPABASE_PUBLIC_API_KEY").unwrap())
    .insert_header("Authorization", format!("Bearer {}", SERVICE_KEY));

let resp = client
    .from("your_table")
    .select("*")
    .execute()
    .await?;
let body = resp
    .text()
    .await?;
```

### Building Queries

[](#building-queries)

These examples assume you've already initialized the client. The methods `.from()` and `.rpc()` initalizes the query builder inside the client.

Using filters:

```
let resp = client
    .from("your_table")
    .eq("country", "Germany")
    .gte("id", "20")
    .select("*")
    .execute()
    .await?;
```

Updating a table:

```
let resp = client
    .from("your_table")
    .eq("username", "soedirgo")
    .update("{\"organization\": \"supabase\"}")
    .execute()
    .await?;
```

Executing stored procedures:

```
let resp = client
    .rpc("add", "{\"a\": 1, \"b\": 2}")
    .execute()
    .await?;
```

_Not enough filters_:

```
let resp = client
    .from("countries")
    .eq("name", "New Zealand")                        // You can filter for equality...
    .gt("id", "20")
    .lt("id", "20")
    .gte("id", "20")
    .lte("id", "20")
    .like("name", "%United%")                         // ...do pattern matching...
    .ilike("name", "%United%")
    .is("name", "null")
    .in_("name", vec!["China", "France"])
    .neq("name", "China")
    .fts("phrase", "The Fat Cats", Some("english"))   // ...do full text search...
    .plfts("phrase", "The Fat Cats", None)
    .phfts("phrase", "The Fat Cats", Some("english"))
    .wfts("phrase", "The Fat Cats", None)
    .cs("countries", "(10,20)")
    .cd("countries", "(10,20)")
    .ov("population_range", "(100,500)")
    .sl("population_range", (100, 500))               // ...and range operations!
    .sr("population_range", (100, 500))               // Find out more about the filters at:
    .nxl("population_range", (100, 500))              // https://postgrest.org/en/stable/api.html#operators
    .nxr("population_range", (100, 500))
    .adj("population_range", (100, 500))
    .select("*")
    .execute()
    .await?;
```

Check out the [API docs](https://docs.rs/postgrest) for more info!

## Contributing

[](#contributing)

Contributions are welcome! There might be some features you want in, or some unclear documentation, or a bug—either way, feel free to create an issue, and we'll work it out!

Boring stuff below.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as below, without any additional terms or conditions.

## License

[](#license)

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](https://github.com/supabase-community/postgrest-rs/blob/master/LICENSE-APACHE) or [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](https://github.com/supabase-community/postgrest-rs/blob/master/LICENSE-MIT) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))

at your option.

---

[![](https://camo.githubusercontent.com/f266fbf746ee25b75480696176e356b84688f1e9/68747470733a2f2f67697463646e2e78797a2f7265706f2f73757061626173652f6d6f6e6f7265706f2f6d61737465722f7765622f7374617469632f77617463682d7265706f2e676966)](https://camo.githubusercontent.com/f266fbf746ee25b75480696176e356b84688f1e9/68747470733a2f2f67697463646e2e78797a2f7265706f2f73757061626173652f6d6f6e6f7265706f2f6d61737465722f7765622f7374617469632f77617463682d7265706f2e676966)
