# 

opendal/blocking/read/

buffer_iterator.rs

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
18use futures::StreamExt;
19
20use crate::Buffer;
21use crate::*;
22
23/// BufferIterator is an iterator of buffers.
24///
25/// # TODO
26///
27/// We can support chunked reader for concurrent read in the future.
28pub struct BufferIterator {
29    handle: tokio::runtime::Handle,
30    inner: Option<BufferStream>,
31}
32
33impl BufferIterator {
34    /// Create a new buffer iterator.
35    #[inline]
36    pub(crate) fn new(handle: tokio::runtime::Handle, inner: BufferStream) -> Self {
37        Self {
38            handle,
39            inner: Some(inner),
40        }
41    }
42}
43
44impl Iterator for BufferIterator {
45    type Item = Result<Buffer>;
46
47    fn next(&mut self) -> Option<Self::Item> {
48        let Some(inner) = self.inner.as_mut() else {
49            return Some(Err(Error::new(
50                ErrorKind::Unexpected,
51                "reader has been dropped",
52            )));
53        };
54
55        self.handle.block_on(inner.next())
56    }
57}
58
59/// Make sure the inner reader is dropped in async context.
60impl Drop for BufferIterator {
61    fn drop(&mut self) {
62        if let Some(v) = self.inner.take() {
63            self.handle.block_on(async move { drop(v) });
64        }
65    }
66}
```
