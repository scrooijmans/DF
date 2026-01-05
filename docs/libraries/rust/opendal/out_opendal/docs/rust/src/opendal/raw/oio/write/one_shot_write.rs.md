# 

opendal/raw/oio/write/

one_shot_write.rs

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
23/// OneShotWrite is used to implement [`oio::Write`] based on one shot operation.
24/// By implementing OneShotWrite, services don't need to care about the details.
25///
26/// For example, S3 `PUT Object` and fs `write_all`.
27///
28/// The layout after adopting [`OneShotWrite`]:
29pub trait OneShotWrite: Send + Sync + Unpin + 'static {
30    /// write_once write all data at once.
31    ///
32    /// Implementations should make sure that the data is written correctly at once.
33    fn write_once(&self, bs: Buffer) -> impl Future<Output = Result<Metadata>> + MaybeSend;
34}
35
36/// OneShotWrite is used to implement [`oio::Write`] based on one shot.
37pub struct OneShotWriter<W: OneShotWrite> {
38    inner: W,
39    buffer: Option<Buffer>,
40}
41
42impl<W: OneShotWrite> OneShotWriter<W> {
43    /// Create a new one shot writer.
44    pub fn new(inner: W) -> Self {
45        Self {
46            inner,
47            buffer: None,
48        }
49    }
50}
51
52impl<W: OneShotWrite> oio::Write for OneShotWriter<W> {
53    async fn write(&mut self, bs: Buffer) -> Result<()> {
54        match &self.buffer {
55            Some(_) => Err(Error::new(
56                ErrorKind::Unsupported,
57                "OneShotWriter doesn't support multiple write",
58            )),
59            None => {
60                self.buffer = Some(bs);
61                Ok(())
62            }
63        }
64    }
65
66    async fn close(&mut self) -> Result<Metadata> {
67        match self.buffer.clone() {
68            Some(bs) => self.inner.write_once(bs).await,
69            None => self.inner.write_once(Buffer::new()).await,
70        }
71    }
72
73    async fn abort(&mut self) -> Result<()> {
74        self.buffer = None;
75        Ok(())
76    }
77}
```
