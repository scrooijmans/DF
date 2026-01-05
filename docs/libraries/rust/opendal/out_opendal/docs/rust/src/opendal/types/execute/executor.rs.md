# 

opendal/types/execute/

executor.rs

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
18use std::fmt::Debug;
19use std::fmt::Formatter;
20use std::future::Future;
21use std::sync::Arc;
22
23use futures::FutureExt;
24
25use super::*;
26use crate::raw::BoxedStaticFuture;
27use crate::raw::MaybeSend;
28
29/// Executor that runs futures in background.
30///
31/// Executor is created by users and used by opendal. So it's by design that Executor only
32/// expose constructor methods.
33///
34/// Executor will run futures in background and return a `Task` as handle to the future. Users
35/// can call `task.await` to wait for the future to complete or drop the `Task` to cancel it.
36#[derive(Clone)]
37pub struct Executor {
38    executor: Arc<dyn Execute>,
39}
40
41impl Debug for Executor {
42    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
43        write!(f, "Executor")
44    }
45}
46
47impl Default for Executor {
48    fn default() -> Self {
49        Self::new()
50    }
51}
52
53impl Executor {
54    /// Create a default executor.
55    ///
56    /// The default executor is enabled by feature flags. If no feature flags enabled, the default
57    /// executor will always return error if users try to perform concurrent tasks.
58    pub fn new() -> Self {
59        #[cfg(feature = "executors-tokio")]
60        {
61            Self::with(executors::TokioExecutor::default())
62        }
63        #[cfg(not(feature = "executors-tokio"))]
64        {
65            Self::with(())
66        }
67    }
68
69    /// Create a new executor with given execute impl.
70    pub fn with(exec: impl Execute) -> Self {
71        Self {
72            executor: Arc::new(exec),
73        }
74    }
75
76    /// Return the inner executor.
77    pub(crate) fn into_inner(self) -> Arc<dyn Execute> {
78        self.executor
79    }
80
81    /// Return a future that will be resolved after the given timeout.
82    pub(crate) fn timeout(&self) -> Option<BoxedStaticFuture<()>> {
83        self.executor.timeout()
84    }
85
86    /// Run given future in background immediately.
87    pub(crate) fn execute<F>(&self, f: F) -> Task<F::Output>
88    where
89        F: Future + MaybeSend + 'static,
90        F::Output: MaybeSend + 'static,
91    {
92        let (fut, handle) = f.remote_handle();
93        self.executor.execute(Box::pin(fut));
94        Task::new(handle)
95    }
96}
```
