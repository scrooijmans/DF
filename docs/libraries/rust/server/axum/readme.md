# GitHub - tokio-rs/axum: Ergonomic and modular web framework built with Tokio, Tower, and Hyper
axum
----

[](#axum)

`axum` is a web application framework that focuses on ergonomics and modularity.

[![Build status](https://github.com/tokio-rs/axum/actions/workflows/CI.yml/badge.svg?branch=main)](https://github.com/tokio-rs/axum/actions/workflows/CI.yml) [![Crates.io](https://camo.githubusercontent.com/0636f88c6507dba7ce401faa57637af8a9237dbdcad132c323579743d4091ab9/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f762f6178756d)](https://crates.io/crates/axum) [![Documentation](https://camo.githubusercontent.com/194681819e1e75555dd9124488f6f2362c8a691049c849aa6d73b2de0a4ff0d3/68747470733a2f2f646f63732e72732f6178756d2f62616467652e737667)](https://docs.rs/axum)

More information about this crate can be found in the [crate documentation](https://docs.rs/axum).

High level features
-------------------

[](#high-level-features)

*   Route requests to handlers with a macro free API.
*   Declaratively parse requests using extractors.
*   Simple and predictable error handling model.
*   Generate responses with minimal boilerplate.
*   Take full advantage of the [`tower`](https://crates.io/crates/tower) and [`tower-http`](https://crates.io/crates/tower-http) ecosystem of middleware, services, and utilities.

In particular the last point is what sets `axum` apart from other frameworks. `axum` doesn't have its own middleware system but instead uses [`tower::Service`](https://docs.rs/tower/latest/tower/trait.Service.html). This means `axum` gets timeouts, tracing, compression, authorization, and more, for free. It also enables you to share middleware with applications written using [`hyper`](https://crates.io/crates/hyper) or [`tonic`](https://crates.io/crates/tonic).

⚠ Breaking changes ⚠
--------------------

[](#-breaking-changes-)

We are currently working towards axum 0.9 so the `main` branch contains breaking changes. See the [`0.8.x`](https://github.com/tokio-rs/axum/tree/v0.8.x) branch for what's released to crates.io.

Usage example
-------------

[](#usage-example)

```
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
```


You can find this [example](https://github.com/tokio-rs/axum/tree/main/examples/readme) as well as other example projects in the [example directory](https://github.com/tokio-rs/axum/tree/main/examples).

See the [crate documentation](https://docs.rs/axum) for way more examples.

Performance
-----------

[](#performance)

`axum` is a relatively thin layer on top of [`hyper`](https://crates.io/crates/hyper) and adds very little overhead. So `axum`'s performance is comparable to [`hyper`](https://crates.io/crates/hyper). You can find benchmarks [here](https://github.com/programatik29/rust-web-benchmarks) and [here](https://web-frameworks-benchmark.netlify.app/result?l=rust).

Safety
------

[](#safety)

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

Minimum supported Rust version
------------------------------

[](#minimum-supported-rust-version)

axum's MSRV is 1.75.

Examples
--------

[](#examples)

The [examples](https://github.com/tokio-rs/axum/tree/main/examples) folder contains various examples of how to use `axum`. The [docs](https://docs.rs/axum) also provide lots of code snippets and examples. For full-fledged examples, check out community-maintained [showcases](https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md#project-showcase) or [tutorials](https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md#tutorials).

Getting Help
------------

[](#getting-help)

In the `axum`'s repo we also have a [number of examples](https://github.com/tokio-rs/axum/tree/main/examples) showing how to put everything together. Community-maintained [showcases](https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md#project-showcase) and [tutorials](https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md#tutorials) also demonstrate how to use `axum` for real-world applications. You're also welcome to ask in the [Discord channel](https://discord.gg/tokio) or open a [discussion](https://github.com/tokio-rs/axum/discussions/new?category=q-a) with your question.

Community projects
------------------

[](#community-projects)

See [here](https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md) for a list of community maintained crates and projects built with `axum`.

Contributing
------------

[](#contributing)

🎈 Thanks for your help improving the project! We are so happy to have you! We have a [contributing guide](https://github.com/tokio-rs/axum/blob/main/CONTRIBUTING.md) to help you get involved in the `axum` project.

License
-------

[](#license)

This project is licensed under the [MIT license](https://github.com/tokio-rs/axum/blob/main/axum/LICENSE).

### Contribution

[](#contribution)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `axum` by you, shall be licensed as MIT, without any additional terms or conditions.