# 

opendal/docs/performance/

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
18//! OpenDAL Performance Guide
19//!
20//! OpenDAL is designed in a zero-cost abstraction way, which means that
21//! users won't pay for the abstraction cost if they don't use it. But this
22//! can also means that users can't maximize the performance of OpenDAL
23//! if they don't know how to use it.
24//!
25//! This document will introduce some tips to improve the performance of
26//! OpenDAL.
27
28#[doc = include_str!("concurrent_write.md")]
29pub mod concurrent_write {}
30
31#[doc = include_str!("http_optimization.md")]
32pub mod http_optimization {}
```
