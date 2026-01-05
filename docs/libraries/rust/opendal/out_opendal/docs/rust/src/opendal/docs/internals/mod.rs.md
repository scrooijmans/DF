# 

opendal/docs/internals/

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
18//! The internal implement details of OpenDAL.
19//!
20//! OpenDAL has provides unified abstraction via two-level API sets:
21//!
22//! - Public API like [`Operator`] provides user level API.
23//! - Raw API like [`Access`], [`Layer`] provides developer level API.
24//!
25//! OpenDAL tries it's best to keep the public API stable. But raw APIs
26//! may change between minor releases from time to time. So most users
27//! should only use the public API. And only developers need to implement
28//! with raw API to implement a new service [`Access`] or their own
29//! [`Layer`].
30//!
31//! In this section, we will talk about the following components:
32//!
33//! - [`Access`][accessor]: to connect underlying storage services.
34//! - [`Layer`][layer]: middleware/interceptor between storage services.
35//!
36//! The relation between [`Access`], [`Layer`] and [`Operator`] looks like the following:
37//!
38//! ```text
39//! âââââââââââââââââââââââââââââââââââââââââââââââââââ¬âââââââââââ
40//! â                                                 â          â
41//! â              ââââââââââââ  ââââââââââ           â          â
42//! â              â          â  â        â¼           â          â
43//! â      s3âââ   â          â  â Tracing Layer      â          â
44//! â          â   â          â  â        â           â          â
45//! â     gcsâââ¤   â          â  â        â¼           â          â
46//! â          ââââºâ Accessor ââââ Metrics Layer âââââºâ Operator â
47//! â  azblobâââ¤   â          â           â      â    â          â
48//! â          â   â          â           â¼      â    â          â
49//! â    hdfsâââ   â          â    Logging Layer â    â          â
50//! â              â          â           â      â    â          â
51//! â              ââââââââââââ           ââââââââ    â          â
52//! â                                                 â          â
53//! âââââââââââââââââââââââââââââââââââââââââââââââââââ´âââââââââââ
54//! ```
55//!
56//! [`Builder`]: crate::Builder
57//! [`Operator`]: crate::Operator
58//! [`Access`]: crate::raw::Access
59//! [`Layer`]: crate::raw::Layer
60
61pub mod accessor;
62pub mod layer;
```
