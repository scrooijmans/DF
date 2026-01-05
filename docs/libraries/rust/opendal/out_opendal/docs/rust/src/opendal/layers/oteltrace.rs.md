# 

opendal/layers/

oteltrace.rs

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
19use std::sync::Arc;
20
21use opentelemetry::Context as TraceContext;
22use opentelemetry::KeyValue;
23use opentelemetry::global;
24use opentelemetry::global::BoxedSpan;
25use opentelemetry::trace::FutureExt as TraceFutureExt;
26use opentelemetry::trace::Span;
27use opentelemetry::trace::TraceContextExt;
28use opentelemetry::trace::Tracer;
29
30use crate::raw::*;
31use crate::*;
32
33/// Add [opentelemetry::trace](https://docs.rs/opentelemetry/latest/opentelemetry/trace/index.html) for every operation.
34///
35/// Examples
36///
37/// ## Basic Setup
38///
39/// ```no_run
40/// # use opendal::layers::OtelTraceLayer;
41/// # use opendal::services;
42/// # use opendal::Operator;
43/// # use opendal::Result;
44///
45/// # fn main() -> Result<()> {
46/// let _ = Operator::new(services::Memory::default())?
47///     .layer(OtelTraceLayer)
48///     .finish();
49/// Ok(())
50/// # }
51/// ```
52pub struct OtelTraceLayer;
53
54impl<A: Access> Layer<A> for OtelTraceLayer {
55    type LayeredAccess = OtelTraceAccessor<A>;
56
57    fn layer(&self, inner: A) -> Self::LayeredAccess {
58        OtelTraceAccessor { inner }
59    }
60}
61
62#[derive(Debug)]
63pub struct OtelTraceAccessor<A> {
64    inner: A,
65}
66
67impl<A: Access> LayeredAccess for OtelTraceAccessor<A> {
68    type Inner = A;
69    type Reader = OtelTraceWrapper<A::Reader>;
70    type Writer = OtelTraceWrapper<A::Writer>;
71    type Lister = OtelTraceWrapper<A::Lister>;
72    type Deleter = A::Deleter;
73
74    fn inner(&self) -> &Self::Inner {
75        &self.inner
76    }
77
78    fn info(&self) -> Arc<AccessorInfo> {
79        let tracer = global::tracer("opendal");
80        tracer.in_span("info", |_cx| self.inner.info())
81    }
82
83    async fn create_dir(&self, path: &str, args: OpCreateDir) -> Result<RpCreateDir> {
84        let tracer = global::tracer("opendal");
85        let mut span = tracer.start("create");
86        span.set_attribute(KeyValue::new("path", path.to_string()));
87        span.set_attribute(KeyValue::new("args", format!("{args:?}")));
88        let cx = TraceContext::current_with_span(span);
89        self.inner.create_dir(path, args).with_context(cx).await
90    }
91
92    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
93        let tracer = global::tracer("opendal");
94        let mut span = tracer.start("read");
95        span.set_attribute(KeyValue::new("path", path.to_string()));
96        span.set_attribute(KeyValue::new("args", format!("{args:?}")));
97        self.inner
98            .read(path, args)
99            .await
100            .map(|(rp, r)| (rp, OtelTraceWrapper::new(span, r)))
101    }
102
103    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
104        let tracer = global::tracer("opendal");
105        let mut span = tracer.start("write");
106        span.set_attribute(KeyValue::new("path", path.to_string()));
107        span.set_attribute(KeyValue::new("args", format!("{args:?}")));
108        self.inner
109            .write(path, args)
110            .await
111            .map(|(rp, r)| (rp, OtelTraceWrapper::new(span, r)))
112    }
113
114    async fn copy(&self, from: &str, to: &str, args: OpCopy) -> Result<RpCopy> {
115        let tracer = global::tracer("opendal");
116        let mut span = tracer.start("copy");
117        span.set_attribute(KeyValue::new("from", from.to_string()));
118        span.set_attribute(KeyValue::new("to", to.to_string()));
119        span.set_attribute(KeyValue::new("args", format!("{args:?}")));
120        let cx = TraceContext::current_with_span(span);
121        self.inner().copy(from, to, args).with_context(cx).await
122    }
123
124    async fn rename(&self, from: &str, to: &str, args: OpRename) -> Result<RpRename> {
125        let tracer = global::tracer("opendal");
126        let mut span = tracer.start("rename");
127        span.set_attribute(KeyValue::new("from", from.to_string()));
128        span.set_attribute(KeyValue::new("to", to.to_string()));
129        span.set_attribute(KeyValue::new("args", format!("{args:?}")));
130        let cx = TraceContext::current_with_span(span);
131        self.inner().rename(from, to, args).with_context(cx).await
132    }
133
134    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
135        let tracer = global::tracer("opendal");
136        let mut span = tracer.start("stat");
137        span.set_attribute(KeyValue::new("path", path.to_string()));
138        span.set_attribute(KeyValue::new("args", format!("{args:?}")));
139        let cx = TraceContext::current_with_span(span);
140        self.inner().stat(path, args).with_context(cx).await
141    }
142
143    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
144        self.inner().delete().await
145    }
146
147    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
148        let tracer = global::tracer("opendal");
149        let mut span = tracer.start("list");
150        span.set_attribute(KeyValue::new("path", path.to_string()));
151        span.set_attribute(KeyValue::new("args", format!("{args:?}")));
152        self.inner
153            .list(path, args)
154            .await
155            .map(|(rp, s)| (rp, OtelTraceWrapper::new(span, s)))
156    }
157
158    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
159        let tracer = global::tracer("opendal");
160        let mut span = tracer.start("presign");
161        span.set_attribute(KeyValue::new("path", path.to_string()));
162        span.set_attribute(KeyValue::new("args", format!("{args:?}")));
163        let cx = TraceContext::current_with_span(span);
164        self.inner().presign(path, args).with_context(cx).await
165    }
166}
167
168pub struct OtelTraceWrapper<R> {
169    _span: BoxedSpan,
170    inner: R,
171}
172
173impl<R> OtelTraceWrapper<R> {
174    fn new(_span: BoxedSpan, inner: R) -> Self {
175        Self { _span, inner }
176    }
177}
178
179impl<R: oio::Read> oio::Read for OtelTraceWrapper<R> {
180    async fn read(&mut self) -> Result<Buffer> {
181        self.inner.read().await
182    }
183}
184
185impl<R: oio::Write> oio::Write for OtelTraceWrapper<R> {
186    fn write(&mut self, bs: Buffer) -> impl Future<Output = Result<()>> + MaybeSend {
187        self.inner.write(bs)
188    }
189
190    fn abort(&mut self) -> impl Future<Output = Result<()>> + MaybeSend {
191        self.inner.abort()
192    }
193
194    fn close(&mut self) -> impl Future<Output = Result<Metadata>> + MaybeSend {
195        self.inner.close()
196    }
197}
198
199impl<R: oio::List> oio::List for OtelTraceWrapper<R> {
200    async fn next(&mut self) -> Result<Option<oio::Entry>> {
201        self.inner.next().await
202    }
203}
```
