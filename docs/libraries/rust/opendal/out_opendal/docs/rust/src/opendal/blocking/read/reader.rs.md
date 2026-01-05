# 

opendal/blocking/read/

reader.rs

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
18use std::ops::RangeBounds;
19
20use bytes::BufMut;
21
22use super::BufferIterator;
23use super::StdBytesIterator;
24use super::StdReader;
25use crate::Reader as AsyncReader;
26use crate::*;
27
28/// BlockingReader is designed to read data from given path in an blocking
29/// manner.
30#[derive(Clone)]
31pub struct Reader {
32    handle: tokio::runtime::Handle,
33    inner: Option<AsyncReader>,
34}
35
36impl Reader {
37    /// Create a new blocking reader.
38    ///
39    /// We don't want to expose those details to users so keep this function
40    /// in crate only.
41    pub(crate) fn new(handle: tokio::runtime::Handle, inner: AsyncReader) -> Self {
42        Reader {
43            handle,
44            inner: Some(inner),
45        }
46    }
47
48    /// Read give range from reader into [`Buffer`].
49    ///
50    /// This operation is zero-copy, which means it keeps the [`bytes::Bytes`] returned by underlying
51    /// storage services without any extra copy or intensive memory allocations.
52    ///
53    /// # Notes
54    ///
55    /// - Buffer length smaller than range means we have reached the end of file.
56    pub fn read(&self, range: impl RangeBounds<u64>) -> Result<Buffer> {
57        let inner = self
58            .inner
59            .as_ref()
60            .ok_or_else(|| Error::new(ErrorKind::Unexpected, "reader has been dropped"))?;
61        self.handle.block_on(inner.read(range))
62    }
63
64    ///
65    /// This operation will copy and write bytes into given [`BufMut`]. Allocation happens while
66    /// [`BufMut`] doesn't have enough space.
67    ///
68    /// # Notes
69    ///
70    /// - Returning length smaller than range means we have reached the end of file.
71    pub fn read_into(&self, buf: &mut impl BufMut, range: impl RangeBounds<u64>) -> Result<usize> {
72        let inner = self
73            .inner
74            .as_ref()
75            .ok_or_else(|| Error::new(ErrorKind::Unexpected, "reader has been dropped"))?;
76        self.handle.block_on(inner.read_into(buf, range))
77    }
78
79    /// Create a buffer iterator to read specific range from given reader.
80    pub fn into_iterator(mut self, range: impl RangeBounds<u64>) -> Result<BufferIterator> {
81        let inner = self
82            .inner
83            .take()
84            .ok_or_else(|| Error::new(ErrorKind::Unexpected, "reader has been dropped"))?;
85        let iter = self.handle.block_on(inner.into_stream(range))?;
86
87        Ok(BufferIterator::new(self.handle.clone(), iter))
88    }
89
90    /// Convert reader into [`StdReader`] which implements [`futures::AsyncRead`],
91    /// [`futures::AsyncSeek`] and [`futures::AsyncBufRead`].
92    #[inline]
93    pub fn into_std_read(mut self, range: impl RangeBounds<u64>) -> Result<StdReader> {
94        let inner = self
95            .inner
96            .take()
97            .ok_or_else(|| Error::new(ErrorKind::Unexpected, "reader has been dropped"))?;
98
99        let r = self.handle.block_on(inner.into_futures_async_read(range))?;
100
101        Ok(StdReader::new(self.handle.clone(), r))
102    }
103
104    /// Convert reader into [`StdBytesIterator`] which implements [`Iterator`].
105    #[inline]
106    pub fn into_bytes_iterator(mut self, range: impl RangeBounds<u64>) -> Result<StdBytesIterator> {
107        let inner = self
108            .inner
109            .take()
110            .ok_or_else(|| Error::new(ErrorKind::Unexpected, "reader has been dropped"))?;
111
112        let iter = self.handle.block_on(inner.into_bytes_stream(range))?;
113        Ok(StdBytesIterator::new(self.handle.clone(), iter))
114    }
115}
116
117/// Make sure the inner reader is dropped in async context.
118impl Drop for Reader {
119    fn drop(&mut self) {
120        if let Some(v) = self.inner.take() {
121            self.handle.block_on(async move { drop(v) });
122        }
123    }
124}
```
