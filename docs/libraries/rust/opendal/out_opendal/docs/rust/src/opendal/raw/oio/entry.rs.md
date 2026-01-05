# 

opendal/raw/oio/

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
18use crate::*;
19
20/// Entry is returned by `Page` or `BlockingPage` during list operations.
21///
22/// # Notes
23///
24/// Differences between `crate::Entry` and `oio::Entry`:
25///
26/// - `crate::Entry` is the user's public API and have less public methods.
27/// - `oio::Entry` is the raw API and doesn't expose to users.
28#[derive(Debug, Clone, PartialEq, Eq)]
29pub struct Entry {
30    path: String,
31    meta: Metadata,
32}
33
34impl Entry {
35    /// Create a new entry by its corresponding underlying storage.
36    pub fn new(path: &str, meta: Metadata) -> Entry {
37        Self::with(path.to_string(), meta)
38    }
39
40    /// Create a new entry with given value.
41    pub fn with(mut path: String, meta: Metadata) -> Entry {
42        // Normalize path as `/` if it's empty.
43        if path.is_empty() {
44            path = "/".to_string();
45        }
46
47        debug_assert!(
48            meta.mode().is_dir() == path.ends_with('/'),
49            "mode {:?} not match with path {}",
50            meta.mode(),
51            path
52        );
53
54        Entry { path, meta }
55    }
56
57    /// Set path for entry.
58    pub fn set_path(&mut self, path: &str) -> &mut Self {
59        self.path = path.to_string();
60        self
61    }
62
63    /// Get the path of entry.
64    pub fn path(&self) -> &str {
65        &self.path
66    }
67
68    /// Set mode for entry.
69    ///
70    /// # Note
71    ///
72    /// Please use this function carefully.
73    pub fn set_mode(&mut self, mode: EntryMode) -> &mut Self {
74        self.meta.set_mode(mode);
75        self
76    }
77
78    /// Get entry's mode.
79    pub fn mode(&self) -> EntryMode {
80        self.meta.mode()
81    }
82
83    /// Consume self to convert into an Entry.
84    ///
85    /// NOTE: implement this by hand to avoid leaking raw entry to end-users.
86    pub(crate) fn into_entry(self) -> crate::Entry {
87        crate::Entry::new(self.path, self.meta)
88    }
89}
```
