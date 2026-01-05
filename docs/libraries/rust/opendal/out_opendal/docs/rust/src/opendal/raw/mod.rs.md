# 

opendal/raw/

mod.rs

``` rust
1// Licensed to the Apache Software Foundation (ASF) under one
2// or more contributor license agreements.  See the NOTICE file
3// distributed with this work for additional information
4// regarding copyright ownership.  The ASF licenses this file
5// to you under the Apache License, Version 2.0 (the
6// "License"); you may not use this file except in compliance
7// with the License.  You may obtain a copy of the License at
8//
9//   http://www.apache.org/licenses/LICENSE-2.0
10//
11// Unless required by applicable law or agreed to in writing,
12// software distributed under the License is distributed on an
13// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
14// KIND, either express or implied.  See the License for the
15// specific language governing permissions and limitations
16// under the License.
17
18//! Raw modules provide raw APIs that used by underlying services
19//!
20//! ## Notes
21//!
22//! - Only developers who want to develop new services or layers need to
23//!   access raw APIs.
24//! - Raw APIs should only be accessed via `opendal::raw::Xxxx`, any public
25//!   API should never expose raw API directly.
26//! - Raw APIs are far less stable than public API, please don't rely on
27//!   them whenever possible.
28
29mod accessor;
30pub use accessor::*;
31
32#[cfg(any(
33    feature = "services-azblob",
34    feature = "services-azdls",
35    feature = "services-azfile"
36))]
37mod azure;
38#[cfg(any(
39    feature = "services-azblob",
40    feature = "services-azdls",
41    feature = "services-azfile"
42))]
43pub(crate) use azure::*;
44
45mod layer;
46pub use layer::*;
47
48mod path;
49pub use path::*;
50
51#[cfg(feature = "internal-path-cache")]
52mod path_cache;
53#[cfg(feature = "internal-path-cache")]
54pub use path_cache::*;
55
56mod operation;
57pub use operation::*;
58
59mod version;
60pub use version::VERSION;
61
62mod rps;
63pub use rps::*;
64
65mod ops;
66pub use ops::*;
67
68mod http_util;
69pub use http_util::*;
70
71mod serde_util;
72pub use serde_util::*;
73
74mod time;
75pub use time::*;
76
77#[cfg(feature = "internal-tokio-rt")]
78mod tokio_util;
79#[cfg(feature = "internal-tokio-rt")]
80pub use tokio_util::*;
81
82mod std_io_util;
83pub use std_io_util::*;
84
85mod futures_util;
86pub use futures_util::BoxedFuture;
87pub use futures_util::BoxedStaticFuture;
88pub use futures_util::ConcurrentTasks;
89pub use futures_util::MaybeSend;
90
91mod enum_utils;
92pub use enum_utils::*;
93
94mod atomic_util;
95pub use atomic_util::*;
96
97// Expose as a pub mod to avoid confusing.
98pub mod adapters;
99pub mod oio;
100#[cfg(feature = "tests")]
101pub mod tests;
```
