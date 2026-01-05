# 

opendal/docs/

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
18//! This module holds documentation for OpenDAL.
19//!
20//! It's highly recommended that you start by reading [`concepts`] first.
21
22#![allow(rustdoc::bare_urls)]
23
24pub mod comparisons;
25
26pub mod concepts;
27
28pub mod internals;
29
30pub mod performance;
31
32/// Changes log for all OpenDAL released versions.
33#[doc = include_str!("../../CHANGELOG.md")]
34#[cfg(not(doctest))]
35pub mod changelog {}
36
37#[cfg(not(doctest))]
38pub mod rfcs;
39
40/// Upgrade and migrate procedures while OpenDAL meets breaking changes.
41#[doc = include_str!("upgrade.md")]
42#[cfg(not(doctest))]
43pub mod upgrade {}
```
