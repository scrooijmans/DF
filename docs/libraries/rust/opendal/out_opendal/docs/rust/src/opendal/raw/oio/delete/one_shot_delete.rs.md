# 

opendal/raw/oio/delete/

one_shot_delete.rs

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
18use std::future::Future;
19
20use crate::raw::*;
21use crate::*;
22
23/// OneShotDelete is used to implement [`oio::Delete`] based on one shot operation.
24///
25/// OneShotDeleter will perform delete operation while calling `flush`.
26pub trait OneShotDelete: Send + Sync + Unpin + 'static {
27    /// delete_once delete one path at once.
28    ///
29    /// Implementations should make sure that the data is deleted correctly at once.
30    fn delete_once(
31        &self,
32        path: String,
33        args: OpDelete,
34    ) -> impl Future<Output = Result<()>> + MaybeSend;
35}
36
37/// OneShotDelete is used to implement [`oio::Delete`] based on one shot.
38pub struct OneShotDeleter<D> {
39    inner: D,
40    delete: Option<(String, OpDelete)>,
41}
42
43impl<D> OneShotDeleter<D> {
44    /// Create a new one shot deleter.
45    pub fn new(inner: D) -> Self {
46        Self {
47            inner,
48            delete: None,
49        }
50    }
51
52    fn delete_inner(&mut self, path: String, args: OpDelete) -> Result<()> {
53        if self.delete.is_some() {
54            return Err(Error::new(
55                ErrorKind::Unsupported,
56                "OneShotDeleter doesn't support batch delete",
57            ));
58        }
59
60        self.delete = Some((path, args));
61        Ok(())
62    }
63}
64
65impl<D: OneShotDelete> oio::Delete for OneShotDeleter<D> {
66    fn delete(&mut self, path: &str, args: OpDelete) -> Result<()> {
67        self.delete_inner(path.to_string(), args)
68    }
69
70    async fn flush(&mut self) -> Result<usize> {
71        let Some((path, args)) = self.delete.clone() else {
72            return Ok(0);
73        };
74
75        self.inner.delete_once(path, args).await?;
76        self.delete = None;
77        Ok(1)
78    }
79}
```
