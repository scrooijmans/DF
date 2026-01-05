# 

opendal/raw/oio/write/

append_write.rs

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
18use std::future::Future;
19
20use crate::raw::*;
21use crate::*;
22
23/// AppendWrite is used to implement [`oio::Write`] based on append
24/// object. By implementing AppendWrite, services don't need to
25/// care about the details of buffering and uploading parts.
26///
27/// The layout after adopting [`AppendWrite`]:
28///
29/// - Services impl `AppendWrite`
30/// - `AppendWriter` impl `Write`
31/// - Expose `AppendWriter` as `Accessor::Writer`
32///
33/// ## Requirements
34///
35/// Services that implement `AppendWrite` must fulfill the following requirements:
36///
37/// - Must be a http service that could accept `AsyncBody`.
38/// - Provide a way to get the current offset of the append object.
39pub trait AppendWrite: Send + Sync + Unpin + 'static {
40    /// Get the current offset of the append object.
41    ///
42    /// Returns `0` if the object is not exist.
43    fn offset(&self) -> impl Future<Output = Result<u64>> + MaybeSend;
44
45    /// Append the data to the end of this object.
46    fn append(
47        &self,
48        offset: u64,
49        size: u64,
50        body: Buffer,
51    ) -> impl Future<Output = Result<Metadata>> + MaybeSend;
52}
53
54/// AppendWriter will implements [`oio::Write`] based on append object.
55///
56/// ## TODO
57///
58/// - Allow users to switch to un-buffered mode if users write 16MiB every time.
59pub struct AppendWriter<W: AppendWrite> {
60    inner: W,
61
62    offset: Option<u64>,
63
64    meta: Metadata,
65}
66
67/// # Safety
68///
69/// wasm32 is a special target that we only have one event-loop for this state.
70impl<W: AppendWrite> AppendWriter<W> {
71    /// Create a new AppendWriter.
72    pub fn new(inner: W) -> Self {
73        Self {
74            inner,
75            offset: None,
76            meta: Metadata::default(),
77        }
78    }
79}
80
81impl<W> oio::Write for AppendWriter<W>
82where
83    W: AppendWrite,
84{
85    async fn write(&mut self, bs: Buffer) -> Result<()> {
86        let offset = match self.offset {
87            Some(offset) => offset,
88            None => {
89                let offset = self.inner.offset().await?;
90                self.offset = Some(offset);
91                offset
92            }
93        };
94
95        let size = bs.len();
96        self.meta = self.inner.append(offset, size as u64, bs).await?;
97        // Update offset after succeed.
98        self.offset = Some(offset + size as u64);
99        Ok(())
100    }
101
102    async fn close(&mut self) -> Result<Metadata> {
103        self.meta
104            .set_content_length(self.offset.unwrap_or_default());
105        Ok(self.meta.clone())
106    }
107
108    async fn abort(&mut self) -> Result<()> {
109        Ok(())
110    }
111}
```
