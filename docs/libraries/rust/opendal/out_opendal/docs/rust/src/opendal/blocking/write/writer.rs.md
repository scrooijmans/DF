# 

opendal/blocking/write/

writer.rs

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
18use super::std_writer::StdWriter;
19use crate::Writer as AsyncWriter;
20use crate::*;
21
22/// BlockingWriter is designed to write data into given path in an blocking
23/// manner.
24pub struct Writer {
25    handle: tokio::runtime::Handle,
26    inner: Option<AsyncWriter>,
27}
28
29impl Writer {
30    /// Create a new writer.
31    ///
32    /// Create will use internal information to decide the most suitable
33    /// implementation for users.
34    ///
35    /// We don't want to expose those details to users so keep this function
36    /// in crate only.
37    pub(crate) fn new(handle: tokio::runtime::Handle, inner: AsyncWriter) -> Self {
38        Self {
39            handle,
40            inner: Some(inner),
41        }
42    }
43
44    /// Write [`Buffer`] into writer.
45    ///
46    /// This operation will write all data in given buffer into writer.
47    ///
48    /// ## Examples
49    ///
50    /// ```
51    /// use bytes::Bytes;
52    /// use opendal::blocking;
53    /// use opendal::blocking::Operator;
54    /// use opendal::Result;
55    ///
56    /// fn test(op: blocking::Operator) -> Result<()> {
57    ///     let mut w = op.writer("hello.txt")?;
58    ///     // Buffer can be created from continues bytes.
59    ///     w.write("hello, world")?;
60    ///     // Buffer can also be created from non-continues bytes.
61    ///     w.write(vec![Bytes::from("hello,"), Bytes::from("world!")])?;
62    ///
63    ///     // Make sure file has been written completely.
64    ///     w.close()?;
65    ///     Ok(())
66    /// }
67    /// ```
68    pub fn write(&mut self, bs: impl Into<Buffer>) -> Result<()> {
69        let Some(inner) = self.inner.as_mut() else {
70            return Err(Error::new(ErrorKind::Unexpected, "writer has been dropped"));
71        };
72
73        self.handle.block_on(inner.write(bs))
74    }
75
76    /// Close the writer and make sure all data have been committed.
77    ///
78    /// ## Notes
79    ///
80    /// Close should only be called when the writer is not closed or
81    /// aborted, otherwise an unexpected error could be returned.
82    pub fn close(&mut self) -> Result<Metadata> {
83        let Some(inner) = self.inner.as_mut() else {
84            return Err(Error::new(ErrorKind::Unexpected, "writer has been dropped"));
85        };
86
87        self.handle.block_on(inner.close())
88    }
89
90    /// Convert writer into [`StdWriter`] which implements [`std::io::Write`],
91    pub fn into_std_write(mut self) -> StdWriter {
92        let inner = self
93            .inner
94            .take()
95            .ok_or_else(|| Error::new(ErrorKind::Unexpected, "writer has been dropped"))
96            .expect("writer has been dropped");
97
98        StdWriter::new(self.handle.clone(), inner)
99    }
100}
101
102/// Make sure the inner writer is dropped in async context.
103impl Drop for Writer {
104    fn drop(&mut self) {
105        if let Some(v) = self.inner.take() {
106            self.handle.block_on(async move { drop(v) });
107        }
108    }
109}
```
