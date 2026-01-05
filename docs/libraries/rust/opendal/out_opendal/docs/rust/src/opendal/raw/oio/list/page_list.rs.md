# 

opendal/raw/oio/list/

page_list.rs

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
18use std::collections::VecDeque;
19use std::future::Future;
20
21use crate::raw::*;
22use crate::*;
23
24/// PageList is used to implement [`oio::List`] based on API supporting pagination. By implementing
25/// PageList, services don't need to care about the details of page list.
26///
27/// # Architecture
28///
29/// The architecture after adopting [`PageList`]:
30///
31/// - Services impl `PageList`
32/// - `PageLister` impl `List`
33/// - Expose `PageLister` as `Accessor::Lister`
34pub trait PageList: Send + Sync + Unpin + 'static {
35    /// next_page is used to fetch next page of entries from underlying storage.
36    fn next_page(&self, ctx: &mut PageContext) -> impl Future<Output = Result<()>> + MaybeSend;
37}
38
39/// PageContext is the context passing between `PageList`.
40///
41/// [`PageLister`] will init the PageContext, and implementer of [`PageList`] should fill the `PageContext`
42/// based on their needs.
43///
44/// - Set `done` to `true` if all page have been fetched.
45/// - Update `token` if there is more page to fetch. `token` is not exposed to users, it's internal used only.
46/// - Push back into the entries for each entry fetched from underlying storage.
47///
48/// NOTE: `entries` is a `VecDeque` to avoid unnecessary memory allocation. Only `push_back` is allowed.
49pub struct PageContext {
50    /// done is used to indicate whether the list operation is done.
51    pub done: bool,
52    /// token is used by underlying storage services to fetch next page.
53    pub token: String,
54    /// entries are used to store entries fetched from underlying storage.
55    ///
56    /// Please always reuse the same `VecDeque` to avoid unnecessary memory allocation.
57    /// PageLister makes sure that entries is reset before calling `next_page`. Implementer
58    /// can call `push_back` on `entries` directly.
59    pub entries: VecDeque<oio::Entry>,
60}
61
62/// PageLister implements [`oio::List`] based on [`PageList`].
63pub struct PageLister<L: PageList> {
64    inner: L,
65    ctx: PageContext,
66}
67
68impl<L> PageLister<L>
69where
70    L: PageList,
71{
72    /// Create a new PageLister.
73    pub fn new(l: L) -> Self {
74        Self {
75            inner: l,
76            ctx: PageContext {
77                done: false,
78                token: "".to_string(),
79                entries: VecDeque::new(),
80            },
81        }
82    }
83}
84
85impl<L> oio::List for PageLister<L>
86where
87    L: PageList,
88{
89    async fn next(&mut self) -> Result<Option<oio::Entry>> {
90        loop {
91            if let Some(entry) = self.ctx.entries.pop_front() {
92                return Ok(Some(entry));
93            }
94            if self.ctx.done {
95                return Ok(None);
96            }
97
98            self.inner.next_page(&mut self.ctx).await?;
99        }
100    }
101}
```
