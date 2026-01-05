# 

opendal/layers/

concurrent_limit.rs

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
20use std::sync::Arc;
21use std::task::Context;
22use std::task::Poll;
23
24use futures::Stream;
25use futures::StreamExt;
26use tokio::sync::OwnedSemaphorePermit;
27use tokio::sync::Semaphore;
28
29use crate::raw::*;
30use crate::*;
31
32/// Add concurrent request limit.
33///
34/// # Notes
35///
36/// Users can control how many concurrent connections could be established
37/// between OpenDAL and underlying storage services.
38///
39/// All operators wrapped by this layer will share a common semaphore. This
40/// allows you to reuse the same layer across multiple operators, ensuring
41/// that the total number of concurrent requests across the entire
42/// application does not exceed the limit.
43///
44/// # Examples
45///
46/// Add a concurrent limit layer to the operator:
47///
48/// ```no_run
49/// # use opendal::layers::ConcurrentLimitLayer;
50/// # use opendal::services;
51/// # use opendal::Operator;
52/// # use opendal::Result;
53/// # use opendal::Scheme;
54///
55/// # fn main() -> Result<()> {
56/// let _ = Operator::new(services::Memory::default())?
57///     .layer(ConcurrentLimitLayer::new(1024))
58///     .finish();
59/// Ok(())
60/// # }
61/// ```
62///
63/// Share a concurrent limit layer between the operators:
64///
65/// ```no_run
66/// # use opendal::layers::ConcurrentLimitLayer;
67/// # use opendal::services;
68/// # use opendal::Operator;
69/// # use opendal::Result;
70/// # use opendal::Scheme;
71///
72/// # fn main() -> Result<()> {
73/// let limit = ConcurrentLimitLayer::new(1024);
74///
75/// let _operator_a = Operator::new(services::Memory::default())?
76///     .layer(limit.clone())
77///     .finish();
78/// let _operator_b = Operator::new(services::Memory::default())?
79///     .layer(limit.clone())
80///     .finish();
81///
82/// Ok(())
83/// # }
84/// ```
85#[derive(Clone)]
86pub struct ConcurrentLimitLayer {
87    operation_semaphore: Arc<Semaphore>,
88    http_semaphore: Option<Arc<Semaphore>>,
89}
90
91impl ConcurrentLimitLayer {
92    /// Create a new ConcurrentLimitLayer will specify permits.
93    ///
94    /// This permits will applied to all operations.
95    pub fn new(permits: usize) -> Self {
96        Self {
97            operation_semaphore: Arc::new(Semaphore::new(permits)),
98            http_semaphore: None,
99        }
100    }
101
102    /// Set a concurrent limit for HTTP requests.
103    ///
104    /// This will limit the number of concurrent HTTP requests made by the
105    /// operator.
106    pub fn with_http_concurrent_limit(mut self, permits: usize) -> Self {
107        self.http_semaphore = Some(Arc::new(Semaphore::new(permits)));
108        self
109    }
110}
111
112impl<A: Access> Layer<A> for ConcurrentLimitLayer {
113    type LayeredAccess = ConcurrentLimitAccessor<A>;
114
115    fn layer(&self, inner: A) -> Self::LayeredAccess {
116        let info = inner.info();
117
118        // Update http client with metrics http fetcher.
119        info.update_http_client(|client| {
120            HttpClient::with(ConcurrentLimitHttpFetcher {
121                inner: client.into_inner(),
122                http_semaphore: self.http_semaphore.clone(),
123            })
124        });
125
126        ConcurrentLimitAccessor {
127            inner,
128            semaphore: self.operation_semaphore.clone(),
129        }
130    }
131}
132
133pub struct ConcurrentLimitHttpFetcher {
134    inner: HttpFetcher,
135    http_semaphore: Option<Arc<Semaphore>>,
136}
137
138impl HttpFetch for ConcurrentLimitHttpFetcher {
139    async fn fetch(&self, req: http::Request<Buffer>) -> Result<http::Response<HttpBody>> {
140        let Some(semaphore) = self.http_semaphore.clone() else {
141            return self.inner.fetch(req).await;
142        };
143
144        let permit = semaphore
145            .acquire_owned()
146            .await
147            .expect("semaphore must be valid");
148
149        let resp = self.inner.fetch(req).await?;
150        let (parts, body) = resp.into_parts();
151        let body = body.map_inner(|s| {
152            Box::new(ConcurrentLimitStream {
153                inner: s,
154                _permit: permit,
155            })
156        });
157        Ok(http::Response::from_parts(parts, body))
158    }
159}
160
161pub struct ConcurrentLimitStream<S> {
162    inner: S,
163    // Hold on this permit until this reader has been dropped.
164    _permit: OwnedSemaphorePermit,
165}
166
167impl<S> Stream for ConcurrentLimitStream<S>
168where
169    S: Stream<Item = Result<Buffer>> + Unpin + 'static,
170{
171    type Item = Result<Buffer>;
172
173    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
174        self.inner.poll_next_unpin(cx)
175    }
176}
177
178#[derive(Debug, Clone)]
179pub struct ConcurrentLimitAccessor<A: Access> {
180    inner: A,
181    semaphore: Arc<Semaphore>,
182}
183
184impl<A: Access> LayeredAccess for ConcurrentLimitAccessor<A> {
185    type Inner = A;
186    type Reader = ConcurrentLimitWrapper<A::Reader>;
187    type Writer = ConcurrentLimitWrapper<A::Writer>;
188    type Lister = ConcurrentLimitWrapper<A::Lister>;
189    type Deleter = ConcurrentLimitWrapper<A::Deleter>;
190
191    fn inner(&self) -> &Self::Inner {
192        &self.inner
193    }
194
195    async fn create_dir(&self, path: &str, args: OpCreateDir) -> Result<RpCreateDir> {
196        let _permit = self
197            .semaphore
198            .acquire()
199            .await
200            .expect("semaphore must be valid");
201
202        self.inner.create_dir(path, args).await
203    }
204
205    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
206        let permit = self
207            .semaphore
208            .clone()
209            .acquire_owned()
210            .await
211            .expect("semaphore must be valid");
212
213        self.inner
214            .read(path, args)
215            .await
216            .map(|(rp, r)| (rp, ConcurrentLimitWrapper::new(r, permit)))
217    }
218
219    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
220        let permit = self
221            .semaphore
222            .clone()
223            .acquire_owned()
224            .await
225            .expect("semaphore must be valid");
226
227        self.inner
228            .write(path, args)
229            .await
230            .map(|(rp, w)| (rp, ConcurrentLimitWrapper::new(w, permit)))
231    }
232
233    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
234        let _permit = self
235            .semaphore
236            .acquire()
237            .await
238            .expect("semaphore must be valid");
239
240        self.inner.stat(path, args).await
241    }
242
243    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
244        let permit = self
245            .semaphore
246            .clone()
247            .acquire_owned()
248            .await
249            .expect("semaphore must be valid");
250
251        self.inner
252            .delete()
253            .await
254            .map(|(rp, w)| (rp, ConcurrentLimitWrapper::new(w, permit)))
255    }
256
257    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
258        let permit = self
259            .semaphore
260            .clone()
261            .acquire_owned()
262            .await
263            .expect("semaphore must be valid");
264
265        self.inner
266            .list(path, args)
267            .await
268            .map(|(rp, s)| (rp, ConcurrentLimitWrapper::new(s, permit)))
269    }
270}
271
272pub struct ConcurrentLimitWrapper<R> {
273    inner: R,
274
275    // Hold on this permit until this reader has been dropped.
276    _permit: OwnedSemaphorePermit,
277}
278
279impl<R> ConcurrentLimitWrapper<R> {
280    fn new(inner: R, permit: OwnedSemaphorePermit) -> Self {
281        Self {
282            inner,
283            _permit: permit,
284        }
285    }
286}
287
288impl<R: oio::Read> oio::Read for ConcurrentLimitWrapper<R> {
289    async fn read(&mut self) -> Result<Buffer> {
290        self.inner.read().await
291    }
292}
293
294impl<R: oio::Write> oio::Write for ConcurrentLimitWrapper<R> {
295    async fn write(&mut self, bs: Buffer) -> Result<()> {
296        self.inner.write(bs).await
297    }
298
299    async fn close(&mut self) -> Result<Metadata> {
300        self.inner.close().await
301    }
302
303    async fn abort(&mut self) -> Result<()> {
304        self.inner.abort().await
305    }
306}
307
308impl<R: oio::List> oio::List for ConcurrentLimitWrapper<R> {
309    async fn next(&mut self) -> Result<Option<oio::Entry>> {
310        self.inner.next().await
311    }
312}
313
314impl<R: oio::Delete> oio::Delete for ConcurrentLimitWrapper<R> {
315    fn delete(&mut self, path: &str, args: OpDelete) -> Result<()> {
316        self.inner.delete(path, args)
317    }
318
319    async fn flush(&mut self) -> Result<usize> {
320        self.inner.flush().await
321    }
322}
```
