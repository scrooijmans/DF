# 

opendal/services/memory/

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
18/// Default scheme for memory service.
19pub const MEMORY_SCHEME: &str = "memory";
20mod backend;
21pub use backend::MemoryBuilder as Memory;
22
23mod config;
24pub use config::MemoryConfig;
25
26mod core;
27mod delete;
28mod lister;
29mod writer;
```
