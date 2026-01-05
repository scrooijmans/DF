# 

opendal/raw/oio/write/

api.rs

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
19use std::ops::DerefMut;
20
21use crate::raw::*;
22use crate::*;
23
24/// Writer is a type erased [`Write`]
25pub type Writer = Box<dyn WriteDyn>;
26
27/// Write is the trait that OpenDAL returns to callers.
28pub trait Write: Unpin + Send + Sync {
29    /// Write given bytes into writer.
30    ///
31    /// # Behavior
32    ///
33    /// - `Ok(())` means all bytes has been written successfully.
34    /// - `Err(err)` means error happens and no bytes has been written.
35    fn write(&mut self, bs: Buffer) -> impl Future<Output = Result<()>> + MaybeSend;
36
37    /// Close the writer and make sure all data has been flushed.
38    fn close(&mut self) -> impl Future<Output = Result<Metadata>> + MaybeSend;
39
40    /// Abort the pending writer.
41    fn abort(&mut self) -> impl Future<Output = Result<()>> + MaybeSend;
42}
43
44impl Write for () {
45    async fn write(&mut self, _: Buffer) -> Result<()> {
46        unimplemented!("write is required to be implemented for oio::Write")
47    }
48
49    async fn close(&mut self) -> Result<Metadata> {
50        Err(Error::new(
51            ErrorKind::Unsupported,
52            "output writer doesn't support close",
53        ))
54    }
55
56    async fn abort(&mut self) -> Result<()> {
57        Err(Error::new(
58            ErrorKind::Unsupported,
59            "output writer doesn't support abort",
60        ))
61    }
62}
63
64/// WriteDyn is the dyn version of [`Write`] make it possible to use as
65/// `Box<dyn WriteDyn>`.
66pub trait WriteDyn: Unpin + Send + Sync {
67    /// The dyn version of [`Write::write`].
68    fn write_dyn(&mut self, bs: Buffer) -> BoxedFuture<'_, Result<()>>;
69
70    /// The dyn version of [`Write::close`].
71    fn close_dyn(&mut self) -> BoxedFuture<'_, Result<Metadata>>;
72
73    /// The dyn version of [`Write::abort`].
74    fn abort_dyn(&mut self) -> BoxedFuture<'_, Result<()>>;
75}
76
77impl<T: Write + ?Sized> WriteDyn for T {
78    fn write_dyn(&mut self, bs: Buffer) -> BoxedFuture<'_, Result<()>> {
79        Box::pin(self.write(bs))
80    }
81
82    fn close_dyn(&mut self) -> BoxedFuture<'_, Result<Metadata>> {
83        Box::pin(self.close())
84    }
85
86    fn abort_dyn(&mut self) -> BoxedFuture<'_, Result<()>> {
87        Box::pin(self.abort())
88    }
89}
90
91impl<T: WriteDyn + ?Sized> Write for Box<T> {
92    async fn write(&mut self, bs: Buffer) -> Result<()> {
93        self.deref_mut().write_dyn(bs).await
94    }
95
96    async fn close(&mut self) -> Result<Metadata> {
97        self.deref_mut().close_dyn().await
98    }
99
100    async fn abort(&mut self) -> Result<()> {
101        self.deref_mut().abort_dyn().await
102    }
103}
```
