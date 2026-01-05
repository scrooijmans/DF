# 

opendal/types/

entry.rs

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
21/// Entry returned by [`Lister`] or [`BlockingLister`] to represent a path and it's relative metadata.
22#[derive(Clone, Debug)]
23pub struct Entry {
24    /// Path of this entry.
25    path: String,
26
27    /// Metadata of this entry.
28    metadata: Metadata,
29}
30
31impl Entry {
32    /// Create an entry with metadata.
33    ///
34    /// # Notes
35    ///
36    /// The only way to get an entry with associated cached metadata
37    /// is `Operator::list`.
38    pub(crate) fn new(path: String, metadata: Metadata) -> Self {
39        Self { path, metadata }
40    }
41
42    /// Path of entry. Path is relative to operator's root.
43    ///
44    /// Only valid in current operator.
45    ///
46    /// If this entry is a dir, `path` MUST end with `/`
47    /// Otherwise, `path` MUST NOT end with `/`.
48    pub fn path(&self) -> &str {
49        &self.path
50    }
51
52    /// Name of entry. Name is the last segment of path.
53    ///
54    /// If this entry is a dir, `name` MUST end with `/`
55    /// Otherwise, `name` MUST NOT end with `/`.
56    pub fn name(&self) -> &str {
57        get_basename(&self.path)
58    }
59
60    /// Fetch metadata of this entry.
61    pub fn metadata(&self) -> &Metadata {
62        &self.metadata
63    }
64
65    /// Consume this entry to get its path and metadata.
66    pub fn into_parts(self) -> (String, Metadata) {
67        (self.path, self.metadata)
68    }
69}
```
