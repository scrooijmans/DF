# 

opendal/blocking/

list.rs

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
18use futures::StreamExt;
19
20use crate::Lister as AsyncLister;
21use crate::*;
22
23/// BlockingLister is designed to list entries at given path in a blocking
24/// manner.
25///
26/// Users can construct Lister by [`blocking::Operator::lister`] or [`blocking::Operator::lister_with`].
27///
28/// - Lister implements `Iterator<Item = Result<Entry>>`.
29/// - Lister will return `None` if there is no more entries or error has been returned.
30pub struct Lister {
31    handle: tokio::runtime::Handle,
32    lister: Option<AsyncLister>,
33}
34
35/// # Safety
36///
37/// BlockingLister will only be accessed by `&mut Self`
38unsafe impl Sync for Lister {}
39
40impl Lister {
41    /// Create a new lister.
42    pub(crate) fn new(handle: tokio::runtime::Handle, lister: AsyncLister) -> Self {
43        Self {
44            handle,
45            lister: Some(lister),
46        }
47    }
48}
49
50impl Iterator for Lister {
51    type Item = Result<Entry>;
52
53    fn next(&mut self) -> Option<Self::Item> {
54        let Some(lister) = self.lister.as_mut() else {
55            return Some(Err(Error::new(
56                ErrorKind::Unexpected,
57                "lister has been dropped",
58            )));
59        };
60
61        self.handle.block_on(lister.next())
62    }
63}
64
65impl Drop for Lister {
66    fn drop(&mut self) {
67        if let Some(v) = self.lister.take() {
68            self.handle.block_on(async move { drop(v) });
69        }
70    }
71}
```
