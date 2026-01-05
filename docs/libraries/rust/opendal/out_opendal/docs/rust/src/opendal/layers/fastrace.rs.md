# 

opendal/layers/

fastrace.rs

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
18use std::fmt::Debug;
19use std::future::Future;
20use std::sync::Arc;
21
22use fastrace::prelude::*;
23
24use crate::raw::*;
25use crate::*;
26
27/// Add [fastrace](https://docs.rs/fastrace/) for every operation.
28///
29/// # Examples
30///
31/// ## Basic Setup
32///
33/// ```no_run
34/// # use opendal::layers::FastraceLayer;
35/// # use opendal::services;
36/// # use opendal::Operator;
37/// # use opendal::Result;
38///
39/// # fn main() -> Result<()> {
40/// let _ = Operator::new(services::Memory::default())?
41///     .layer(FastraceLayer)
42///     .finish();
43/// Ok(())
44/// # }
45/// ```
46///
47/// ## Real usage
48///
49/// ```no_run
50/// # use anyhow::Result;
51/// # use fastrace::prelude::*;
52/// # use opendal::layers::FastraceLayer;
53/// # use opendal::services;
54/// # use opendal::Operator;
55///
56/// # fn main() -> Result<()> {
57/// let reporter =
58///     fastrace_jaeger::JaegerReporter::new("127.0.0.1:6831".parse()?, "opendal").unwrap();
59/// fastrace::set_reporter(reporter, fastrace::collector::Config::default());
60///
61/// {
62///     let root = Span::root("op", SpanContext::random());
63///     let runtime = tokio::runtime::Runtime::new()?;
64///     runtime.block_on(
65///         async {
66///             let _ = dotenvy::dotenv();
67///             let op = Operator::new(services::Memory::default())?
68///                 .layer(FastraceLayer)
69///                 .finish();
70///             op.write("test", "0".repeat(16 * 1024 * 1024).into_bytes())
71///                 .await?;
72///             op.stat("test").await?;
73///             op.read("test").await?;
74///             Ok::<(), opendal::Error>(())
75///         }
76///         .in_span(Span::enter_with_parent("test", &root)),
77///     )?;
78/// }
79///
80/// fastrace::flush();
81///
82/// Ok(())
83/// # }
84/// ```
85///
86/// # Output
87///
88/// OpenDAL is using [`fastrace`](https://docs.rs/fastrace/latest/fastrace/) for tracing internally.
89///
90/// To enable fastrace output, please init one of the reporter that `fastrace` supports.
91///
92/// For example:
93///
94/// ```no_run
95/// # use anyhow::Result;
96///
97/// # fn main() -> Result<()> {
98/// let reporter =
99///     fastrace_jaeger::JaegerReporter::new("127.0.0.1:6831".parse()?, "opendal").unwrap();
100/// fastrace::set_reporter(reporter, fastrace::collector::Config::default());
101/// Ok(())
102/// # }
103/// ```
104///
105/// For real-world usage, please take a look at [`fastrace-datadog`](https://crates.io/crates/fastrace-datadog) or [`fastrace-jaeger`](https://crates.io/crates/fastrace-jaeger) .
106pub struct FastraceLayer;
107
108impl<A: Access> Layer<A> for FastraceLayer {
109    type LayeredAccess = FastraceAccessor<A>;
110
111    fn layer(&self, inner: A) -> Self::LayeredAccess {
112        FastraceAccessor { inner }
113    }
114}
115
116#[derive(Debug)]
117pub struct FastraceAccessor<A> {
118    inner: A,
119}
120
121impl<A: Access> LayeredAccess for FastraceAccessor<A> {
122    type Inner = A;
123    type Reader = FastraceWrapper<A::Reader>;
124    type Writer = FastraceWrapper<A::Writer>;
125    type Lister = FastraceWrapper<A::Lister>;
126    type Deleter = FastraceWrapper<A::Deleter>;
127
128    fn inner(&self) -> &Self::Inner {
129        &self.inner
130    }
131
132    #[trace]
133    fn info(&self) -> Arc<AccessorInfo> {
134        self.inner.info()
135    }
136
137    #[trace(enter_on_poll = true)]
138    async fn create_dir(&self, path: &str, args: OpCreateDir) -> Result<RpCreateDir> {
139        self.inner.create_dir(path, args).await
140    }
141
142    #[trace(enter_on_poll = true)]
143    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
144        self.inner.read(path, args).await.map(|(rp, r)| {
145            (
146                rp,
147                FastraceWrapper::new(
148                    Span::enter_with_local_parent(Operation::Read.into_static()),
149                    r,
150                ),
151            )
152        })
153    }
154
155    #[trace(enter_on_poll = true)]
156    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
157        self.inner.write(path, args).await.map(|(rp, r)| {
158            (
159                rp,
160                FastraceWrapper::new(
161                    Span::enter_with_local_parent(Operation::Write.into_static()),
162                    r,
163                ),
164            )
165        })
166    }
167
168    #[trace(enter_on_poll = true)]
169    async fn copy(&self, from: &str, to: &str, args: OpCopy) -> Result<RpCopy> {
170        self.inner().copy(from, to, args).await
171    }
172
173    #[trace(enter_on_poll = true)]
174    async fn rename(&self, from: &str, to: &str, args: OpRename) -> Result<RpRename> {
175        self.inner().rename(from, to, args).await
176    }
177
178    #[trace(enter_on_poll = true)]
179    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
180        self.inner.stat(path, args).await
181    }
182
183    #[trace(enter_on_poll = true)]
184    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
185        self.inner.delete().await.map(|(rp, r)| {
186            (
187                rp,
188                FastraceWrapper::new(
189                    Span::enter_with_local_parent(Operation::Delete.into_static()),
190                    r,
191                ),
192            )
193        })
194    }
195
196    #[trace(enter_on_poll = true)]
197    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
198        self.inner.list(path, args).await.map(|(rp, s)| {
199            (
200                rp,
201                FastraceWrapper::new(
202                    Span::enter_with_local_parent(Operation::List.into_static()),
203                    s,
204                ),
205            )
206        })
207    }
208
209    #[trace(enter_on_poll = true)]
210    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
211        self.inner.presign(path, args).await
212    }
213}
214
215pub struct FastraceWrapper<R> {
216    span: Span,
217    inner: R,
218}
219
220impl<R> FastraceWrapper<R> {
221    fn new(span: Span, inner: R) -> Self {
222        Self { span, inner }
223    }
224}
225
226impl<R: oio::Read> oio::Read for FastraceWrapper<R> {
227    #[trace(enter_on_poll = true)]
228    async fn read(&mut self) -> Result<Buffer> {
229        self.inner.read().await
230    }
231}
232
233impl<R: oio::Write> oio::Write for FastraceWrapper<R> {
234    fn write(&mut self, bs: Buffer) -> impl Future<Output = Result<()>> + MaybeSend {
235        let _g = self.span.set_local_parent();
236        let _span = LocalSpan::enter_with_local_parent(Operation::Write.into_static());
237        self.inner.write(bs)
238    }
239
240    fn abort(&mut self) -> impl Future<Output = Result<()>> + MaybeSend {
241        let _g = self.span.set_local_parent();
242        let _span = LocalSpan::enter_with_local_parent(Operation::Write.into_static());
243        self.inner.abort()
244    }
245
246    fn close(&mut self) -> impl Future<Output = Result<Metadata>> + MaybeSend {
247        let _g = self.span.set_local_parent();
248        let _span = LocalSpan::enter_with_local_parent(Operation::Write.into_static());
249        self.inner.close()
250    }
251}
252
253impl<R: oio::List> oio::List for FastraceWrapper<R> {
254    #[trace(enter_on_poll = true)]
255    async fn next(&mut self) -> Result<Option<oio::Entry>> {
256        self.inner.next().await
257    }
258}
259
260impl<R: oio::Delete> oio::Delete for FastraceWrapper<R> {
261    fn delete(&mut self, path: &str, args: OpDelete) -> Result<()> {
262        let _g = self.span.set_local_parent();
263        let _span = LocalSpan::enter_with_local_parent(Operation::Delete.into_static());
264        self.inner.delete(path, args)
265    }
266
267    #[trace(enter_on_poll = true)]
268    async fn flush(&mut self) -> Result<usize> {
269        self.inner.flush().await
270    }
271}
```
