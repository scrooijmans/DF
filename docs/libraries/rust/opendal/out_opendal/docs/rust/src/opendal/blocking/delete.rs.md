# 

opendal/blocking/

delete.rs

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
18use crate::Deleter as AsyncDeleter;
19use crate::*;
20
21/// BlockingDeleter is designed to continuously remove content from storage.
22///
23/// It leverages batch deletion capabilities provided by storage services for efficient removal.
24pub struct Deleter {
25    handle: tokio::runtime::Handle,
26    inner: AsyncDeleter,
27}
28
29impl Deleter {
30    pub(crate) fn create(handle: tokio::runtime::Handle, inner: AsyncDeleter) -> Result<Self> {
31        Ok(Self { handle, inner })
32    }
33
34    /// Delete a path.
35    pub fn delete(&mut self, input: impl IntoDeleteInput) -> Result<()> {
36        self.handle.block_on(self.inner.delete(input))
37    }
38
39    /// Delete an infallible iterator of paths.
40    ///
41    /// Also see:
42    ///
43    /// - [`BlockingDeleter::delete_try_iter`]: delete an fallible iterator of paths.
44    pub fn delete_iter<I, D>(&mut self, iter: I) -> Result<()>
45    where
46        I: IntoIterator<Item = D>,
47        D: IntoDeleteInput,
48    {
49        self.handle.block_on(self.inner.delete_iter(iter))
50    }
51
52    /// Delete an fallible iterator of paths.
53    ///
54    /// Also see:
55    ///
56    /// - [`BlockingDeleter::delete_iter`]: delete an infallible iterator of paths.
57    pub fn delete_try_iter<I, D>(&mut self, try_iter: I) -> Result<()>
58    where
59        I: IntoIterator<Item = Result<D>>,
60        D: IntoDeleteInput,
61    {
62        self.handle.block_on(self.inner.delete_try_iter(try_iter))
63    }
64
65    /// Flush the deleter, returns the number of deleted paths.
66    pub fn flush(&mut self) -> Result<usize> {
67        self.handle.block_on(self.inner.flush())
68    }
69
70    /// Close the deleter, this will flush the deleter and wait until all paths are deleted.
71    pub fn close(&mut self) -> Result<()> {
72        self.handle.block_on(self.inner.close())
73    }
74}
```
