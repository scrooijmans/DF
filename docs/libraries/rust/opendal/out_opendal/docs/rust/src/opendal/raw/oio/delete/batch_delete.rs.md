# 

opendal/raw/oio/delete/

batch_delete.rs

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
18use std::collections::HashSet;
19use std::future::Future;
20
21use crate::raw::*;
22use crate::*;
23
24/// BatchDelete is used to implement [`oio::Delete`] based on batch delete operation.
25///
26/// OneShotDeleter will perform delete operation while calling `flush`.
27pub trait BatchDelete: Send + Sync + Unpin + 'static {
28    /// delete_once delete one path at once.
29    ///
30    /// Implementations should make sure that the data is deleted correctly at once.
31    ///
32    /// BatchDeleter may call this method while there are only one path to delete.
33    fn delete_once(
34        &self,
35        path: String,
36        args: OpDelete,
37    ) -> impl Future<Output = Result<()>> + MaybeSend;
38
39    /// delete_batch delete multiple paths at once.
40    ///
41    /// - Implementations should make sure that the length of `batch` equals to the return result's length.
42    /// - Implementations should return error no path is deleted.
43    fn delete_batch(
44        &self,
45        batch: Vec<(String, OpDelete)>,
46    ) -> impl Future<Output = Result<BatchDeleteResult>> + MaybeSend;
47}
48
49/// BatchDeleteResult is the result of batch delete operation.
50#[derive(Default)]
51pub struct BatchDeleteResult {
52    /// Collection of successful deletions, containing tuples of (path, args)
53    pub succeeded: Vec<(String, OpDelete)>,
54    /// Collection of failed deletions, containing tuples of (path, args, error)
55    pub failed: Vec<(String, OpDelete, Error)>,
56}
57
58/// BatchDeleter is used to implement [`oio::Delete`] based on batch delete.
59pub struct BatchDeleter<D: BatchDelete> {
60    inner: D,
61    buffer: HashSet<(String, OpDelete)>,
62}
63
64impl<D: BatchDelete> BatchDeleter<D> {
65    /// Create a new batch deleter.
66    pub fn new(inner: D) -> Self {
67        Self {
68            inner,
69            buffer: HashSet::default(),
70        }
71    }
72}
73
74impl<D: BatchDelete> oio::Delete for BatchDeleter<D> {
75    fn delete(&mut self, path: &str, args: OpDelete) -> Result<()> {
76        self.buffer.insert((path.to_string(), args));
77        Ok(())
78    }
79
80    async fn flush(&mut self) -> Result<usize> {
81        if self.buffer.is_empty() {
82            return Ok(0);
83        }
84        if self.buffer.len() == 1 {
85            let (path, args) = self
86                .buffer
87                .iter()
88                .next()
89                .expect("the delete buffer size must be 1")
90                .clone();
91            self.inner.delete_once(path, args).await?;
92            self.buffer.clear();
93            return Ok(1);
94        }
95
96        let batch = self.buffer.iter().cloned().collect();
97        let result = self.inner.delete_batch(batch).await?;
98        debug_assert!(
99            !result.succeeded.is_empty(),
100            "the number of succeeded operations must be greater than 0"
101        );
102        debug_assert_eq!(
103            result.succeeded.len() + result.failed.len(),
104            self.buffer.len(),
105            "the number of succeeded and failed operations must be equal to the input batch size"
106        );
107
108        // Remove all succeeded operations from the buffer.
109        let deleted = result.succeeded.len();
110        for i in result.succeeded {
111            self.buffer.remove(&i);
112        }
113
114        // Return directly if there are non-temporary errors.
115        for (path, op, err) in result.failed {
116            if !err.is_temporary() {
117                return Err(err
118                    .with_context("path", path)
119                    .with_context("version", op.version().unwrap_or("<latest>")));
120            }
121        }
122
123        // Return the number of succeeded operations to allow users to decide whether
124        // to retry or push more delete operations.
125        Ok(deleted)
126    }
127}
```
