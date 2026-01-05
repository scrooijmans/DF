# 

opendal/raw/oio/

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
18//! `oio` provides OpenDAL's raw traits and types that opendal returns as
19//! output.
20//!
21//! Those types should only be used internally and we don't want users to
22//! depend on them.
23
24mod delete;
25pub use delete::*;
26
27mod read;
28pub use read::*;
29
30mod write;
31pub use write::*;
32
33mod list;
34pub use list::*;
35
36mod entry;
37pub use entry::Entry;
38
39mod buf;
40pub use buf::*;
```
