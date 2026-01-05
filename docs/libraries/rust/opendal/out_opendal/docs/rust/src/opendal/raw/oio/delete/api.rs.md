# 

opendal/raw/oio/delete/

api.rs

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
19use std::ops::DerefMut;
20
21use crate::raw::BoxedFuture;
22use crate::raw::MaybeSend;
23use crate::raw::OpDelete;
24use crate::*;
25
26/// Deleter is a type erased [`Delete`]
27pub type Deleter = Box<dyn DeleteDyn>;
28
29/// The Delete trait defines interfaces for performing deletion operations.
30pub trait Delete: Unpin + Send + Sync {
31    /// Requests deletion of a resource at the specified path with optional arguments
32    ///
33    /// # Parameters
34    /// - `path`: The path of the resource to delete
35    /// - `args`: Additional arguments for the delete operation
36    ///
37    /// # Returns
38    /// - `Ok(())`: The deletion request has been successfully queued (does not guarantee actual deletion)
39    /// - `Err(err)`: An error occurred and the deletion request was not queued
40    ///
41    /// # Notes
42    /// This method just queue the delete request. The actual deletion will be
43    /// performed when `flush` is called.
44    fn delete(&mut self, path: &str, args: OpDelete) -> Result<()>;
45
46    /// Flushes the deletion queue to ensure queued deletions are executed
47    ///
48    /// # Returns
49    /// - `Ok(0)`: All queued deletions have been processed or the queue is empty.
50    /// - `Ok(count)`: The number of resources successfully deleted. Implementations should
51    ///   return an error if the queue is non-empty but no resources were deleted
52    /// - `Err(err)`: An error occurred while performing the deletions
53    ///
54    /// # Notes
55    /// - This method is asynchronous and will wait for queued deletions to complete
56    fn flush(&mut self) -> impl Future<Output = Result<usize>> + MaybeSend;
57}
58
59impl Delete for () {
60    fn delete(&mut self, _: &str, _: OpDelete) -> Result<()> {
61        Err(Error::new(
62            ErrorKind::Unsupported,
63            "output deleter doesn't support delete",
64        ))
65    }
66
67    async fn flush(&mut self) -> Result<usize> {
68        Err(Error::new(
69            ErrorKind::Unsupported,
70            "output deleter doesn't support flush",
71        ))
72    }
73}
74
75/// The dyn version of [`Delete`]
76pub trait DeleteDyn: Unpin + Send + Sync {
77    /// The dyn version of [`Delete::delete`]
78    fn delete_dyn(&mut self, path: &str, args: OpDelete) -> Result<()>;
79
80    /// The dyn version of [`Delete::flush`]
81    fn flush_dyn(&mut self) -> BoxedFuture<'_, Result<usize>>;
82}
83
84impl<T: Delete + ?Sized> DeleteDyn for T {
85    fn delete_dyn(&mut self, path: &str, args: OpDelete) -> Result<()> {
86        Delete::delete(self, path, args)
87    }
88
89    fn flush_dyn(&mut self) -> BoxedFuture<'_, Result<usize>> {
90        Box::pin(self.flush())
91    }
92}
93
94impl<T: DeleteDyn + ?Sized> Delete for Box<T> {
95    fn delete(&mut self, path: &str, args: OpDelete) -> Result<()> {
96        self.deref_mut().delete_dyn(path, args)
97    }
98
99    async fn flush(&mut self) -> Result<usize> {
100        self.deref_mut().flush_dyn().await
101    }
102}
```
