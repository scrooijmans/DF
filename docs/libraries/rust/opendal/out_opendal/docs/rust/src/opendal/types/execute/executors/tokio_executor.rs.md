# 

opendal/types/execute/executors/

tokio_executor.rs

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
18use crate::raw::BoxedStaticFuture;
19use crate::*;
20
21/// Executor that uses the [`tokio::task::spawn`] to execute futures.
22#[derive(Default)]
23pub struct TokioExecutor {}
24
25impl Execute for TokioExecutor {
26    /// Tokio's JoinHandle has its own `abort` support, so dropping handle won't cancel the task.
27    fn execute(&self, f: BoxedStaticFuture<()>) {
28        let _handle = tokio::task::spawn(f);
29    }
30}
31
32#[cfg(test)]
33mod tests {
34    use std::sync::Arc;
35    use std::sync::atomic::AtomicBool;
36    use std::sync::atomic::Ordering;
37    use std::time::Duration;
38
39    use tokio::time::sleep;
40
41    use super::*;
42    use crate::Executor;
43
44    #[tokio::test]
45    async fn test_tokio_executor() {
46        let executor = Executor::with(TokioExecutor::default());
47
48        let finished = Arc::new(AtomicBool::new(false));
49
50        let finished_clone = finished.clone();
51        let _task = executor.execute(async move {
52            sleep(Duration::from_secs(1)).await;
53            finished_clone.store(true, Ordering::Relaxed);
54        });
55
56        sleep(Duration::from_secs(2)).await;
57        // Task must have been finished even without await task.
58        assert!(finished.load(Ordering::Relaxed))
59    }
60}
```
