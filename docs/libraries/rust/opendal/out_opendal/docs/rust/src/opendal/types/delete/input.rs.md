# 

opendal/types/delete/

input.rs

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
18use crate::Entry;
19use crate::raw::OpDelete;
20
21/// DeleteInput is the input for delete operations.
22#[non_exhaustive]
23#[derive(Default, Debug)]
24pub struct DeleteInput {
25    /// The path of the path to delete.
26    pub path: String,
27    /// The version of the path to delete.
28    pub version: Option<String>,
29}
30
31/// IntoDeleteInput is a helper trait that makes it easier for users to play with `Deleter`.
32pub trait IntoDeleteInput: Send + Sync + Unpin {
33    /// Convert `self` into a `DeleteInput`.
34    fn into_delete_input(self) -> DeleteInput;
35}
36
37/// Implement `IntoDeleteInput` for `DeleteInput` self.
38impl IntoDeleteInput for DeleteInput {
39    fn into_delete_input(self) -> DeleteInput {
40        self
41    }
42}
43
44/// Implement `IntoDeleteInput` for `&str` so we can use `&str` as a DeleteInput.
45impl IntoDeleteInput for &str {
46    fn into_delete_input(self) -> DeleteInput {
47        DeleteInput {
48            path: self.to_string(),
49            ..Default::default()
50        }
51    }
52}
53
54/// Implement `IntoDeleteInput` for `String` so we can use `Vec<String>` as a DeleteInput stream.
55impl IntoDeleteInput for String {
56    fn into_delete_input(self) -> DeleteInput {
57        DeleteInput {
58            path: self,
59            ..Default::default()
60        }
61    }
62}
63
64/// Implement `IntoDeleteInput` for `(String, OpDelete)` so we can use `(String, OpDelete)`
65/// as a DeleteInput stream.
66impl IntoDeleteInput for (String, OpDelete) {
67    fn into_delete_input(self) -> DeleteInput {
68        let (path, args) = self;
69
70        let mut input = DeleteInput {
71            path,
72            ..Default::default()
73        };
74
75        if let Some(version) = args.version() {
76            input.version = Some(version.to_string());
77        }
78        input
79    }
80}
81
82/// Implement `IntoDeleteInput` for `Entry` so we can use `Lister` as a DeleteInput stream.
83impl IntoDeleteInput for Entry {
84    fn into_delete_input(self) -> DeleteInput {
85        let (path, meta) = self.into_parts();
86
87        let mut input = DeleteInput {
88            path,
89            ..Default::default()
90        };
91
92        if let Some(version) = meta.version() {
93            input.version = Some(version.to_string());
94        }
95        input
96    }
97}
```
