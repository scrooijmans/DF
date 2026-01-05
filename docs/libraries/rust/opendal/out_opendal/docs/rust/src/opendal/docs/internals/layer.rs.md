# 

opendal/docs/internals/

layer.rs

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
18//! The internal implementation details of [`Layer`].
19//!
20//! [`Layer`] itself is quite simple:
21//!
22//! ```ignore
23//! pub trait Layer<A: Access> {
24//!     type LayeredAccess: Access;
25//!
26//!     fn layer(&self, inner: A) -> Self::LayeredAccess;
27//! }
28//! ```
29//!
30//! `XxxLayer` will wrap input [`Access`] as inner and return a new [`Access`]. So normally the implementation of [`Layer`] will be split into two parts:
31//!
32//! - `XxxLayer` will implement [`Layer`] and return `XxxAccessor` as `Self::LayeredAccess`.
33//! - `XxxAccess` will implement [`Access`] and be built by `XxxLayer`.
34//!
35//! Most layer only implements part of [`Access`], so we provide
36//! [`LayeredAccess`] which will forward all unimplemented methods to
37//! `inner`. It's highly recommend to implement [`LayeredAccess`] trait
38//! instead.
39//!
40//! [`Layer`]: crate::raw::Layer
41//! [`Access`]: crate::raw::Access
42//! [`LayeredAccess`]: crate::raw::LayeredAccess
```
