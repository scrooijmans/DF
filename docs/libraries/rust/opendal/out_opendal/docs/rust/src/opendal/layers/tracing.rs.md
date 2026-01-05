# 

opendal/layers/

tracing.rs

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
19use std::pin::Pin;
20use std::task::Context;
21use std::task::Poll;
22
23use futures::Stream;
24use futures::StreamExt;
25use tracing::Level;
26use tracing::Span;
27use tracing::span;
28
29use crate::raw::*;
30use crate::*;
31
32/// Add [tracing](https://docs.rs/tracing/) for every operation.
33///
34/// # Examples
35///
36/// ## Basic Setup
37///
38/// ```no_run
39/// # use opendal::layers::TracingLayer;
40/// # use opendal::services;
41/// # use opendal::Operator;
42/// # use opendal::Result;
43///
44/// # fn main() -> Result<()> {
45/// let _ = Operator::new(services::Memory::default())?
46///     .layer(TracingLayer)
47///     .finish();
48/// Ok(())
49/// # }
50/// ```
51///
52/// ## Real usage
53///
54/// ```no_run
55/// # use anyhow::Result;
56/// # use opendal::layers::TracingLayer;
57/// # use opendal::services;
58/// # use opendal::Operator;
59/// # use opentelemetry::KeyValue;
60/// # use opentelemetry_sdk::trace;
61/// # use opentelemetry_sdk::Resource;
62/// # use tracing_subscriber::prelude::*;
63/// # use tracing_subscriber::EnvFilter;
64///
65/// # fn main() -> Result<()> {
66/// use opentelemetry::trace::TracerProvider;
67/// let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
68///     .with_simple_exporter(
69///         opentelemetry_otlp::SpanExporter::builder()
70///             .with_tonic()
71///             .build()?,
72///     )
73///     .with_resource(
74///         Resource::builder()
75///             .with_attributes(vec![KeyValue::new("service.name", "opendal_example")])
76///             .build(),
77///     )
78///     .build();
79/// let tracer = tracer_provider.tracer("opendal_tracer");
80/// let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
81///
82/// tracing_subscriber::registry()
83///     .with(EnvFilter::from_default_env())
84///     .with(opentelemetry)
85///     .try_init()?;
86///
87/// {
88///     let runtime = tokio::runtime::Runtime::new()?;
89///     runtime.block_on(async {
90///         let root = tracing::span!(tracing::Level::INFO, "app_start", work_units = 2);
91///         let _enter = root.enter();
92///
93///         let _ = dotenvy::dotenv();
94///         let op = Operator::new(services::Memory::default())?
95///             .layer(TracingLayer)
96///             .finish();
97///
98///         op.write("test", "0".repeat(16 * 1024 * 1024).into_bytes())
99///             .await?;
100///         op.stat("test").await?;
101///         op.read("test").await?;
102///         Ok::<(), opendal::Error>(())
103///     })?;
104/// }
105///
106/// // Shut down the current tracer provider.
107/// // This will invoke the shutdown method on all span processors.
108/// // span processors should export remaining spans before return.
109/// tracer_provider.shutdown()?;
110///
111/// Ok(())
112/// # }
113/// ```
114///
115/// # Output
116///
117/// OpenDAL is using [`tracing`](https://docs.rs/tracing/latest/tracing/) for tracing internally.
118///
119/// To enable tracing output, please init one of the subscribers that `tracing` supports.
120///
121/// For example:
122///
123/// ```no_run
124/// # use tracing::dispatcher;
125/// # use tracing::Event;
126/// # use tracing::Metadata;
127/// # use tracing::span::Attributes;
128/// # use tracing::span::Id;
129/// # use tracing::span::Record;
130/// # use tracing::subscriber::Subscriber;
131///
132/// # pub struct FooSubscriber;
133/// # impl Subscriber for FooSubscriber {
134/// #   fn enabled(&self, _: &Metadata) -> bool { false }
135/// #   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
136/// #   fn record(&self, _: &Id, _: &Record) {}
137/// #   fn record_follows_from(&self, _: &Id, _: &Id) {}
138/// #   fn event(&self, _: &Event) {}
139/// #   fn enter(&self, _: &Id) {}
140/// #   fn exit(&self, _: &Id) {}
141/// # }
142/// # impl FooSubscriber { fn new() -> Self { FooSubscriber } }
143///
144/// let my_subscriber = FooSubscriber::new();
145/// tracing::subscriber::set_global_default(my_subscriber).expect("setting tracing default failed");
146/// ```
147///
148/// For real-world usage, please take a look at [`tracing-opentelemetry`](https://crates.io/crates/tracing-opentelemetry).
149pub struct TracingLayer;
150
151impl<A: Access> Layer<A> for TracingLayer {
152    type LayeredAccess = TracingAccessor<A>;
153
154    fn layer(&self, inner: A) -> Self::LayeredAccess {
155        let info = inner.info();
156
157        // Update http client with metrics http fetcher.
158        info.update_http_client(|client| {
159            HttpClient::with(TracingHttpFetcher {
160                inner: client.into_inner(),
161            })
162        });
163
164        TracingAccessor { inner }
165    }
166}
167
168pub struct TracingHttpFetcher {
169    inner: HttpFetcher,
170}
171
172impl HttpFetch for TracingHttpFetcher {
173    async fn fetch(&self, req: http::Request<Buffer>) -> Result<http::Response<HttpBody>> {
174        let span = span!(Level::DEBUG, "http::fetch", ?req);
175
176        let resp = {
177            let _enter = span.enter();
178            self.inner.fetch(req).await?
179        };
180
181        let (parts, body) = resp.into_parts();
182        let body = body.map_inner(|s| Box::new(TracingStream { inner: s, span }));
183        Ok(http::Response::from_parts(parts, body))
184    }
185}
186
187pub struct TracingStream<S> {
188    inner: S,
189    span: Span,
190}
191
192impl<S> Stream for TracingStream<S>
193where
194    S: Stream<Item = Result<Buffer>> + Unpin + 'static,
195{
196    type Item = Result<Buffer>;
197
198    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
199        let _enter = self.span.clone().entered();
200        self.inner.poll_next_unpin(cx)
201    }
202}
203
204#[derive(Debug)]
205pub struct TracingAccessor<A> {
206    inner: A,
207}
208
209impl<A: Access> LayeredAccess for TracingAccessor<A> {
210    type Inner = A;
211    type Reader = TracingWrapper<A::Reader>;
212    type Writer = TracingWrapper<A::Writer>;
213    type Lister = TracingWrapper<A::Lister>;
214    type Deleter = TracingWrapper<A::Deleter>;
215
216    fn inner(&self) -> &Self::Inner {
217        &self.inner
218    }
219
220    #[tracing::instrument(level = "debug", skip(self))]
221    async fn create_dir(&self, path: &str, args: OpCreateDir) -> Result<RpCreateDir> {
222        self.inner.create_dir(path, args).await
223    }
224
225    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
226        let span = span!(Level::DEBUG, "read", path, ?args);
227
228        let (rp, r) = {
229            let _enter = span.enter();
230            self.inner.read(path, args).await?
231        };
232
233        Ok((rp, TracingWrapper::new(span, r)))
234    }
235
236    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
237        let span = span!(Level::DEBUG, "write", path, ?args);
238
239        let (rp, r) = {
240            let _enter = span.enter();
241            self.inner.write(path, args).await?
242        };
243
244        Ok((rp, TracingWrapper::new(span, r)))
245    }
246
247    #[tracing::instrument(level = "debug", skip(self))]
248    async fn copy(&self, from: &str, to: &str, args: OpCopy) -> Result<RpCopy> {
249        self.inner().copy(from, to, args).await
250    }
251
252    #[tracing::instrument(level = "debug", skip(self))]
253    async fn rename(&self, from: &str, to: &str, args: OpRename) -> Result<RpRename> {
254        self.inner().rename(from, to, args).await
255    }
256
257    #[tracing::instrument(level = "debug", skip(self))]
258    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
259        self.inner.stat(path, args).await
260    }
261
262    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
263        let span = span!(Level::DEBUG, "delete");
264
265        let (rp, r) = {
266            let _enter = span.enter();
267            self.inner.delete().await?
268        };
269
270        Ok((rp, TracingWrapper::new(span, r)))
271    }
272
273    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
274        let span = span!(Level::DEBUG, "list", path, ?args);
275
276        let (rp, r) = {
277            let _enter = span.enter();
278            self.inner.list(path, args).await?
279        };
280
281        Ok((rp, TracingWrapper::new(span, r)))
282    }
283
284    #[tracing::instrument(level = "debug", skip(self))]
285    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
286        self.inner.presign(path, args).await
287    }
288}
289
290pub struct TracingWrapper<R> {
291    span: Span,
292    inner: R,
293}
294
295impl<R> TracingWrapper<R> {
296    fn new(span: Span, inner: R) -> Self {
297        Self { span, inner }
298    }
299}
300
301impl<R: oio::Read> oio::Read for TracingWrapper<R> {
302    async fn read(&mut self) -> Result<Buffer> {
303        let _enter = self.span.enter();
304
305        self.inner.read().await
306    }
307}
308
309impl<R: oio::Write> oio::Write for TracingWrapper<R> {
310    async fn write(&mut self, bs: Buffer) -> Result<()> {
311        let _enter = self.span.enter();
312
313        self.inner.write(bs).await
314    }
315
316    async fn abort(&mut self) -> Result<()> {
317        let _enter = self.span.enter();
318
319        self.inner.abort().await
320    }
321
322    async fn close(&mut self) -> Result<Metadata> {
323        let _enter = self.span.enter();
324
325        self.inner.close().await
326    }
327}
328
329impl<R: oio::List> oio::List for TracingWrapper<R> {
330    async fn next(&mut self) -> Result<Option<oio::Entry>> {
331        let _enter = self.span.enter();
332
333        self.inner.next().await
334    }
335}
336
337impl<R: oio::Delete> oio::Delete for TracingWrapper<R> {
338    fn delete(&mut self, path: &str, args: OpDelete) -> Result<()> {
339        let _enter = self.span.enter();
340
341        self.inner.delete(path, args)
342    }
343
344    async fn flush(&mut self) -> Result<usize> {
345        let _enter = self.span.enter();
346
347        self.inner.flush().await
348    }
349}
```
