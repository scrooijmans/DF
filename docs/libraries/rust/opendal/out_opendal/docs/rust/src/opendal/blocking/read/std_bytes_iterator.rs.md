# 

opendal/blocking/read/

std_bytes_iterator.rs

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
20use bytes::Bytes;
21use futures::StreamExt;
22
23use crate::*;
24
25/// StdIterator is the adapter of [`Iterator`] for [`BlockingReader`][crate::BlockingReader].
26///
27/// Users can use this adapter in cases where they need to use [`Iterator`] trait.
28///
29/// StdIterator also implements [`Send`] and [`Sync`].
30pub struct StdBytesIterator {
31    handle: tokio::runtime::Handle,
32    inner: Option<FuturesBytesStream>,
33}
34
35impl StdBytesIterator {
36    /// NOTE: don't allow users to create StdIterator directly.
37    #[inline]
38    pub(crate) fn new(handle: tokio::runtime::Handle, inner: FuturesBytesStream) -> Self {
39        StdBytesIterator {
40            handle,
41            inner: Some(inner),
42        }
43    }
44}
45
46impl Iterator for StdBytesIterator {
47    type Item = io::Result<Bytes>;
48
49    fn next(&mut self) -> Option<Self::Item> {
50        let Some(inner) = self.inner.as_mut() else {
51            return Some(Err(Error::new(
52                ErrorKind::Unexpected,
53                "reader has been dropped",
54            )
55            .into()));
56        };
57
58        self.handle.block_on(inner.next())
59    }
60}
61
62/// Make sure the inner reader is dropped in async context.
63impl Drop for StdBytesIterator {
64    fn drop(&mut self) {
65        if let Some(v) = self.inner.take() {
66            self.handle.block_on(async move { drop(v) });
67        }
68    }
69}
```
