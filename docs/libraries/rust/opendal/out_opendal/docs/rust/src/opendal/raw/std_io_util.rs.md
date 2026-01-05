# 

opendal/raw/

std_io_util.rs

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
18use std::io;
19
20use crate::*;
21
22/// Parse std io error into opendal::Error.
23///
24/// # TODO
25///
26/// Add `NotADirectory` and `IsADirectory` once they are stable.
27///
28/// ref: <https://github.com/rust-lang/rust/issues/86442>
29pub fn new_std_io_error(err: io::Error) -> Error {
30    use std::io::ErrorKind::*;
31
32    let (kind, retryable) = match err.kind() {
33        NotFound => (ErrorKind::NotFound, false),
34        PermissionDenied => (ErrorKind::PermissionDenied, false),
35        AlreadyExists => (ErrorKind::AlreadyExists, false),
36        Unsupported => (ErrorKind::Unsupported, false),
37
38        Interrupted | UnexpectedEof | TimedOut | WouldBlock => (ErrorKind::Unexpected, true),
39        _ => (ErrorKind::Unexpected, true),
40    };
41
42    let mut err = Error::new(kind, err.kind().to_string()).set_source(err);
43
44    if retryable {
45        err = err.set_temporary();
46    }
47
48    err
49}
50
51/// helper functions to format `Error` into `io::Error`.
52///
53/// This function is added privately by design and only valid in current
54/// context (i.e. `raw` mod). We don't want to expose this function to
55/// users.
56#[inline]
57pub(crate) fn format_std_io_error(err: Error) -> io::Error {
58    let kind = match err.kind() {
59        ErrorKind::NotFound => io::ErrorKind::NotFound,
60        ErrorKind::PermissionDenied => io::ErrorKind::PermissionDenied,
61        _ => io::ErrorKind::Interrupted,
62    };
63
64    io::Error::new(kind, err)
65}
```
