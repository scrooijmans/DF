# 

opendal/raw/oio/read/

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
18use std::mem;
19use std::ops::DerefMut;
20
21use bytes::Bytes;
22use futures::Future;
23
24use crate::raw::*;
25use crate::*;
26
27/// Reader is a type erased [`Read`].
28pub type Reader = Box<dyn ReadDyn>;
29
30/// Read is the internal trait used by OpenDAL to read data from storage.
31///
32/// Users should not use or import this trait unless they are implementing an `Accessor`.
33///
34/// # Notes
35///
36/// ## Object Safety
37///
38/// `Read` uses `async in trait`, making it not object safe, preventing the use of `Box<dyn Read>`.
39/// To address this, we've introduced [`ReadDyn`] and its compatible type `Box<dyn ReadDyn>`.
40///
41/// `ReadDyn` uses `Box::pin()` to transform the returned future into a [`BoxedFuture`], introducing
42/// an additional layer of indirection and an extra allocation. Ideally, `ReadDyn` should occur only
43/// once, at the outermost level of our API.
44pub trait Read: Unpin + Send + Sync {
45    /// Read at the given offset with the given size.
46    fn read(&mut self) -> impl Future<Output = Result<Buffer>> + MaybeSend;
47
48    /// Read all data from the reader.
49    fn read_all(&mut self) -> impl Future<Output = Result<Buffer>> + MaybeSend {
50        async {
51            let mut bufs = vec![];
52            loop {
53                match self.read().await {
54                    Ok(buf) if buf.is_empty() => break,
55                    Ok(buf) => bufs.push(buf),
56                    Err(err) => return Err(err),
57                }
58            }
59            Ok(bufs.into_iter().flatten().collect())
60        }
61    }
62}
63
64impl Read for () {
65    async fn read(&mut self) -> Result<Buffer> {
66        Err(Error::new(
67            ErrorKind::Unsupported,
68            "output reader doesn't support read",
69        ))
70    }
71}
72
73impl Read for Bytes {
74    async fn read(&mut self) -> Result<Buffer> {
75        Ok(Buffer::from(self.split_off(0)))
76    }
77}
78
79impl Read for Buffer {
80    async fn read(&mut self) -> Result<Buffer> {
81        Ok(mem::take(self))
82    }
83}
84
85/// ReadDyn is the dyn version of [`Read`] make it possible to use as
86/// `Box<dyn ReadDyn>`.
87pub trait ReadDyn: Unpin + Send + Sync {
88    /// The dyn version of [`Read::read`].
89    ///
90    /// This function returns a boxed future to make it object safe.
91    fn read_dyn(&mut self) -> BoxedFuture<'_, Result<Buffer>>;
92
93    /// The dyn version of [`Read::read_all`]
94    fn read_all_dyn(&mut self) -> BoxedFuture<'_, Result<Buffer>>;
95}
96
97impl<T: Read + ?Sized> ReadDyn for T {
98    fn read_dyn(&mut self) -> BoxedFuture<'_, Result<Buffer>> {
99        Box::pin(self.read())
100    }
101
102    fn read_all_dyn(&mut self) -> BoxedFuture<'_, Result<Buffer>> {
103        Box::pin(self.read_all())
104    }
105}
106
107/// # NOTE
108///
109/// Take care about the `deref_mut()` here. This makes sure that we are calling functions
110/// upon `&mut T` instead of `&mut Box<T>`. The later could result in infinite recursion.
111impl<T: ReadDyn + ?Sized> Read for Box<T> {
112    async fn read(&mut self) -> Result<Buffer> {
113        self.deref_mut().read_dyn().await
114    }
115
116    async fn read_all(&mut self) -> Result<Buffer> {
117        self.deref_mut().read_all_dyn().await
118    }
119}
```
