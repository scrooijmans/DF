# 

opendal/blocking/write/

std_writer.rs

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
18use std::io::Write;
19
20use futures::AsyncWriteExt;
21
22use crate::*;
23
24/// StdWriter is the adapter of [`std::io::Write`] for [`BlockingWriter`].
25///
26/// Users can use this adapter in cases where they need to use [`std::io::Write`] related trait.
27///
28/// # Notes
29///
30/// Files are automatically closed when they go out of scope. Errors detected on closing are ignored
31/// by the implementation of Drop. Use the method `close` if these errors must be manually handled.
32pub struct StdWriter {
33    handle: tokio::runtime::Handle,
34    w: Option<FuturesAsyncWriter>,
35}
36
37impl StdWriter {
38    /// NOTE: don't allow users to create directly.
39    #[inline]
40    pub(crate) fn new(handle: tokio::runtime::Handle, w: Writer) -> Self {
41        StdWriter {
42            handle,
43            w: Some(w.into_futures_async_write()),
44        }
45    }
46
47    /// Close the internal writer and make sure all data have been stored.
48    pub fn close(&mut self) -> std::io::Result<()> {
49        let Some(w) = self.w.as_mut() else {
50            return Err(Error::new(ErrorKind::Unexpected, "writer has been dropped").into());
51        };
52
53        self.handle.block_on(w.close())
54    }
55}
56
57impl Write for StdWriter {
58    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
59        let Some(w) = self.w.as_mut() else {
60            return Err(Error::new(ErrorKind::Unexpected, "writer has been dropped").into());
61        };
62
63        self.handle.block_on(w.write(buf))
64    }
65
66    fn flush(&mut self) -> std::io::Result<()> {
67        let Some(w) = self.w.as_mut() else {
68            return Err(Error::new(ErrorKind::Unexpected, "writer has been dropped").into());
69        };
70
71        self.handle.block_on(w.flush())
72    }
73}
74
75/// Make sure the inner writer is dropped in async context.
76impl Drop for StdWriter {
77    fn drop(&mut self) {
78        if let Some(v) = self.w.take() {
79            self.handle.block_on(async move { drop(v) });
80        }
81    }
82}
```
