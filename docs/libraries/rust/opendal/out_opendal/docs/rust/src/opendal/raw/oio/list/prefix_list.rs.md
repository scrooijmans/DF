# 

opendal/raw/oio/list/

prefix_list.rs

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
18use crate::raw::*;
19use crate::*;
20
21/// PrefixLister is used to filter entries by prefix.
22///
23/// For example, if we have a lister that returns entries:
24///
25/// ```txt
26/// .
27/// âââ file_a
28/// âââ file_b
29/// ```
30///
31/// We can use `PrefixLister` to filter entries with prefix `file_`.
32pub struct PrefixLister<L> {
33    lister: L,
34    prefix: String,
35}
36
37/// # Safety
38///
39/// We will only take `&mut Self` reference for FsLister.
40unsafe impl<L> Sync for PrefixLister<L> {}
41
42impl<L> PrefixLister<L> {
43    /// Create a new flat lister
44    pub fn new(lister: L, prefix: &str) -> PrefixLister<L> {
45        PrefixLister {
46            lister,
47            prefix: prefix.to_string(),
48        }
49    }
50}
51
52impl<L> oio::List for PrefixLister<L>
53where
54    L: oio::List,
55{
56    async fn next(&mut self) -> Result<Option<oio::Entry>> {
57        loop {
58            match self.lister.next().await {
59                Ok(Some(e)) if !e.path().starts_with(&self.prefix) => continue,
60                v => return v,
61            }
62        }
63    }
64}
```
