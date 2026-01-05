# 

opendal/types/

mode.rs

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
18use std::fmt::Display;
19use std::fmt::Formatter;
20
21/// EntryMode represents the mode.
22#[derive(Copy, Clone, Debug, Eq, PartialEq)]
23pub enum EntryMode {
24    /// FILE means the path has data to read.
25    FILE,
26    /// DIR means the path can be listed.
27    DIR,
28    /// Unknown means we don't know what we can do on this path.
29    Unknown,
30}
31
32impl EntryMode {
33    /// Check if this mode is FILE.
34    pub fn is_file(self) -> bool {
35        self == EntryMode::FILE
36    }
37
38    /// Check if this mode is DIR.
39    pub fn is_dir(self) -> bool {
40        self == EntryMode::DIR
41    }
42
43    /// Create entry mode from given path.
44    #[allow(dead_code)]
45    pub(crate) fn from_path(path: &str) -> Self {
46        if path.ends_with('/') {
47            EntryMode::DIR
48        } else {
49            EntryMode::FILE
50        }
51    }
52}
53
54impl Default for EntryMode {
55    fn default() -> Self {
56        Self::Unknown
57    }
58}
59
60impl Display for EntryMode {
61    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
62        match self {
63            EntryMode::FILE => write!(f, "file"),
64            EntryMode::DIR => write!(f, "dir"),
65            EntryMode::Unknown => write!(f, "unknown"),
66        }
67    }
68}
```
