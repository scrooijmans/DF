# 

opendal/types/execute/executors/

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
18//! executors module provides implementations for the [`Execute`](crate::Execute) trait for widely used runtimes.
19//!
20//! Every executor will be hide behind the feature like `executors-xxx`. Users can switch or enable
21//! the executors they want by enabling the corresponding feature. Also, users can provide their
22//! own executor by implementing the [`Execute`](crate::Execute) trait directly.
23
24#[cfg(feature = "executors-tokio")]
25mod tokio_executor;
26#[cfg(feature = "executors-tokio")]
27pub use tokio_executor::TokioExecutor;
```
