# 

opendal/blocking/read/

std_reader.rs

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
19use std::io::BufRead;
20use std::io::Read;
21use std::io::Seek;
22use std::io::SeekFrom;
23
24use futures::AsyncBufReadExt;
25use futures::AsyncReadExt;
26use futures::AsyncSeekExt;
27
28use crate::*;
29
30/// StdReader is the adapter of [`Read`], [`Seek`] and [`BufRead`] for [`BlockingReader`][crate::BlockingReader].
31///
32/// Users can use this adapter in cases where they need to use [`Read`] or [`BufRead`] trait.
33///
34/// StdReader also implements [`Send`] and [`Sync`].
35pub struct StdReader {
36    handle: tokio::runtime::Handle,
37    r: Option<FuturesAsyncReader>,
38}
39
40impl StdReader {
41    /// NOTE: don't allow users to create StdReader directly.
42    #[inline]
43    pub(super) fn new(handle: tokio::runtime::Handle, r: FuturesAsyncReader) -> Self {
44        Self { handle, r: Some(r) }
45    }
46}
47
48impl BufRead for StdReader {
49    fn fill_buf(&mut self) -> io::Result<&[u8]> {
50        let Some(r) = self.r.as_mut() else {
51            return Err(Error::new(ErrorKind::Unexpected, "reader has been dropped").into());
52        };
53
54        self.handle.block_on(r.fill_buf())
55    }
56
57    fn consume(&mut self, amt: usize) {
58        let Some(r) = self.r.as_mut() else {
59            return;
60        };
61
62        r.consume_unpin(amt);
63    }
64}
65
66impl Read for StdReader {
67    #[inline]
68    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
69        let Some(r) = self.r.as_mut() else {
70            return Err(Error::new(ErrorKind::Unexpected, "reader has been dropped").into());
71        };
72
73        self.handle.block_on(r.read(buf))
74    }
75}
76
77impl Seek for StdReader {
78    #[inline]
79    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
80        let Some(r) = self.r.as_mut() else {
81            return Err(Error::new(ErrorKind::Unexpected, "reader has been dropped").into());
82        };
83
84        self.handle.block_on(r.seek(pos))
85    }
86}
87
88/// Make sure the inner reader is dropped in async context.
89impl Drop for StdReader {
90    fn drop(&mut self) {
91        if let Some(v) = self.r.take() {
92            self.handle.block_on(async move { drop(v) });
93        }
94    }
95}
```
