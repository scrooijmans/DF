# 

opendal/raw/oio/list/

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
21use crate::raw::oio::Entry;
22use crate::raw::*;
23use crate::*;
24
25/// The boxed version of [`List`]
26pub type Lister = Box<dyn ListDyn>;
27
28/// Page trait is used by [`raw::Accessor`] to implement `list` operation.
29pub trait List: Unpin + Send + Sync {
30    /// Fetch a new page of [`Entry`]
31    ///
32    /// `Ok(None)` means all pages have been returned. Any following call
33    /// to `next` will always get the same result.
34    fn next(&mut self) -> impl Future<Output = Result<Option<Entry>>> + MaybeSend;
35}
36
37impl List for () {
38    async fn next(&mut self) -> Result<Option<Entry>> {
39        Ok(None)
40    }
41}
42
43impl<P: List> List for Option<P> {
44    async fn next(&mut self) -> Result<Option<Entry>> {
45        match self {
46            Some(p) => p.next().await,
47            None => Ok(None),
48        }
49    }
50}
51
52/// ListDyn is the dyn version of [`List`]. Makes it possible to use as
53/// `Box<dyn ListDyn>`.
54pub trait ListDyn: Unpin + Send + Sync {
55    /// The dyn version of [`List::next`].
56    fn next_dyn(&mut self) -> BoxedFuture<'_, Result<Option<Entry>>>;
57}
58
59impl<T: List + ?Sized> ListDyn for T {
60    fn next_dyn(&mut self) -> BoxedFuture<'_, Result<Option<Entry>>> {
61        Box::pin(self.next())
62    }
63}
64
65impl<T: ListDyn + ?Sized> List for Box<T> {
66    async fn next(&mut self) -> Result<Option<Entry>> {
67        self.deref_mut().next_dyn().await
68    }
69}
```
