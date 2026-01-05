# 

opendal/layers/

async_backtrace.rs

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
18use crate::raw::*;
19use crate::*;
20
21/// Add Efficient, logical 'stack' traces of async functions for the underlying services.
22///
23/// # Async Backtrace
24///
25/// async-backtrace allows developers to get a stack trace of the async functions.
26/// Read more about [async-backtrace](https://docs.rs/async-backtrace/latest/async_backtrace/)
27///
28/// # Examples
29///
30/// ```no_run
31/// # use opendal::layers::AsyncBacktraceLayer;
32/// # use opendal::services;
33/// # use opendal::Operator;
34/// # use opendal::Result;
35/// # use opendal::Scheme;
36///
37/// # fn main() -> Result<()> {
38/// let _ = Operator::new(services::Memory::default())?
39///     .layer(AsyncBacktraceLayer::default())
40///     .finish();
41/// Ok(())
42/// # }
43/// ```
44#[derive(Clone, Default)]
45pub struct AsyncBacktraceLayer;
46
47impl<A: Access> Layer<A> for AsyncBacktraceLayer {
48    type LayeredAccess = AsyncBacktraceAccessor<A>;
49
50    fn layer(&self, accessor: A) -> Self::LayeredAccess {
51        AsyncBacktraceAccessor { inner: accessor }
52    }
53}
54
55#[derive(Debug, Clone)]
56pub struct AsyncBacktraceAccessor<A: Access> {
57    inner: A,
58}
59
60impl<A: Access> LayeredAccess for AsyncBacktraceAccessor<A> {
61    type Inner = A;
62    type Reader = AsyncBacktraceWrapper<A::Reader>;
63    type Writer = AsyncBacktraceWrapper<A::Writer>;
64    type Lister = AsyncBacktraceWrapper<A::Lister>;
65    type Deleter = AsyncBacktraceWrapper<A::Deleter>;
66
67    fn inner(&self) -> &Self::Inner {
68        &self.inner
69    }
70
71    #[async_backtrace::framed]
72    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
73        self.inner
74            .read(path, args)
75            .await
76            .map(|(rp, r)| (rp, AsyncBacktraceWrapper::new(r)))
77    }
78
79    #[async_backtrace::framed]
80    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
81        self.inner
82            .write(path, args)
83            .await
84            .map(|(rp, r)| (rp, AsyncBacktraceWrapper::new(r)))
85    }
86
87    #[async_backtrace::framed]
88    async fn copy(&self, from: &str, to: &str, args: OpCopy) -> Result<RpCopy> {
89        self.inner.copy(from, to, args).await
90    }
91
92    #[async_backtrace::framed]
93    async fn rename(&self, from: &str, to: &str, args: OpRename) -> Result<RpRename> {
94        self.inner.rename(from, to, args).await
95    }
96
97    #[async_backtrace::framed]
98    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
99        self.inner.stat(path, args).await
100    }
101
102    #[async_backtrace::framed]
103    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
104        self.inner
105            .delete()
106            .await
107            .map(|(rp, r)| (rp, AsyncBacktraceWrapper::new(r)))
108    }
109
110    #[async_backtrace::framed]
111    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
112        self.inner
113            .list(path, args)
114            .await
115            .map(|(rp, r)| (rp, AsyncBacktraceWrapper::new(r)))
116    }
117
118    #[async_backtrace::framed]
119    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
120        self.inner.presign(path, args).await
121    }
122}
123
124pub struct AsyncBacktraceWrapper<R> {
125    inner: R,
126}
127
128impl<R> AsyncBacktraceWrapper<R> {
129    fn new(inner: R) -> Self {
130        Self { inner }
131    }
132}
133
134impl<R: oio::Read> oio::Read for AsyncBacktraceWrapper<R> {
135    #[async_backtrace::framed]
136    async fn read(&mut self) -> Result<Buffer> {
137        self.inner.read().await
138    }
139}
140
141impl<R: oio::Write> oio::Write for AsyncBacktraceWrapper<R> {
142    #[async_backtrace::framed]
143    async fn write(&mut self, bs: Buffer) -> Result<()> {
144        self.inner.write(bs).await
145    }
146
147    #[async_backtrace::framed]
148    async fn close(&mut self) -> Result<Metadata> {
149        self.inner.close().await
150    }
151
152    #[async_backtrace::framed]
153    async fn abort(&mut self) -> Result<()> {
154        self.inner.abort().await
155    }
156}
157
158impl<R: oio::List> oio::List for AsyncBacktraceWrapper<R> {
159    #[async_backtrace::framed]
160    async fn next(&mut self) -> Result<Option<oio::Entry>> {
161        self.inner.next().await
162    }
163}
164
165impl<R: oio::Delete> oio::Delete for AsyncBacktraceWrapper<R> {
166    fn delete(&mut self, path: &str, args: OpDelete) -> Result<()> {
167        self.inner.delete(path, args)
168    }
169
170    #[async_backtrace::framed]
171    async fn flush(&mut self) -> Result<usize> {
172        self.inner.flush().await
173    }
174}
```
