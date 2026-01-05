# 

opendal/raw/

operation.rs

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
21/// Operation is the name of the operation that is being performed.
22///
23/// Most operations can be mapped to the methods of the `Access` trait,
24/// but we modify the names to make them more readable and clear for users.
25///
26/// The same operation might have different meanings and costs in different
27/// storage services.
28#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
29#[non_exhaustive]
30pub enum Operation {
31    /// Operation to retrieve information about the specified storage services.
32    #[default]
33    Info,
34    /// Operation to create a directory.
35    CreateDir,
36    /// Operation to read a file.
37    Read,
38    /// Operation to write to a file.
39    Write,
40    /// Operation to copy a file.
41    Copy,
42    /// Operation to rename a file.
43    Rename,
44    /// Operation to stat a file or a directory.
45    Stat,
46    /// Operation to delete files.
47    Delete,
48    /// Operation to get the next file from the list.
49    List,
50    /// Operation to generate a presigned URL.
51    Presign,
52}
53
54impl Operation {
55    /// Convert self into static str.
56    pub fn into_static(self) -> &'static str {
57        self.into()
58    }
59}
60
61impl Display for Operation {
62    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
63        write!(f, "{}", self.into_static())
64    }
65}
66
67impl From<Operation> for &'static str {
68    fn from(v: Operation) -> &'static str {
69        match v {
70            Operation::Info => "info",
71            Operation::CreateDir => "create_dir",
72            Operation::Read => "read",
73            Operation::Write => "write",
74            Operation::Copy => "copy",
75            Operation::Rename => "rename",
76            Operation::Stat => "stat",
77            Operation::Delete => "delete",
78            Operation::List => "list",
79            Operation::Presign => "presign",
80        }
81    }
82}
83
84impl From<Operation> for String {
85    fn from(v: Operation) -> Self {
86        v.into_static().to_string()
87    }
88}
```
