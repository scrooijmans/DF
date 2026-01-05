# 

opendal/layers/

timeout.rs

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
20use std::time::Duration;
21
22use crate::raw::*;
23use crate::*;
24
25/// Add timeout for every operation to avoid slow or unexpected hang operations.
26///
27/// For example, a dead connection could hang a databases sql query. TimeoutLayer
28/// will break this connection and returns an error so users can handle it by
29/// retrying or print to users.
30///
31/// # Notes
32///
33/// `TimeoutLayer` treats all operations in two kinds:
34///
35/// - Non IO Operation like `stat`, `delete` they operate on a single file. We control
36///   them by setting `timeout`.
37/// - IO Operation like `read`, `Reader::read` and `Writer::write`, they operate on data directly, we
38///   control them by setting `io_timeout`.
39///
40/// # Default
41///
42/// - timeout: 60 seconds
43/// - io_timeout: 10 seconds
44///
45/// # Panics
46///
47/// TimeoutLayer will drop the future if the timeout is reached. This might cause the internal state
48/// of the future to be broken. If underlying future moves ownership into the future, it will be
49/// dropped and will neven return back.
50///
51/// For example, while using `TimeoutLayer` with `RetryLayer` at the same time, please make sure
52/// timeout layer showed up before retry layer.
53///
54/// ```no_run
55/// # use std::time::Duration;
56///
57/// # use opendal::layers::RetryLayer;
58/// # use opendal::layers::TimeoutLayer;
59/// # use opendal::services;
60/// # use opendal::Operator;
61/// # use opendal::Result;
62///
63/// # fn main() -> Result<()> {
64/// let op = Operator::new(services::Memory::default())?
65///     // This is fine, since timeout happen during retry.
66///     .layer(TimeoutLayer::new().with_io_timeout(Duration::from_nanos(1)))
67///     .layer(RetryLayer::new())
68///     // This is wrong. Since timeout layer will drop future, leaving retry layer in a bad state.
69///     .layer(TimeoutLayer::new().with_io_timeout(Duration::from_nanos(1)))
70///     .finish();
71/// Ok(())
72/// # }
73/// ```
74///
75/// # Examples
76///
77/// The following examples will create a timeout layer with 10 seconds timeout for all non-io
78/// operations, 3 seconds timeout for all io operations.
79///
80/// ```no_run
81/// # use std::time::Duration;
82///
83/// # use opendal::layers::TimeoutLayer;
84/// # use opendal::services;
85/// # use opendal::Operator;
86/// # use opendal::Result;
87/// # use opendal::Scheme;
88///
89/// # fn main() -> Result<()> {
90/// let _ = Operator::new(services::Memory::default())?
91///     .layer(
92///         TimeoutLayer::default()
93///             .with_timeout(Duration::from_secs(10))
94///             .with_io_timeout(Duration::from_secs(3)),
95///     )
96///     .finish();
97/// Ok(())
98/// # }
99/// ```
100///
101/// # Implementation Notes
102///
103/// TimeoutLayer is using [`tokio::time::timeout`] to implement timeout for operations. And IO
104/// Operations insides `reader`, `writer` will use `Pin<Box<tokio::time::Sleep>>` to track the
105/// timeout.
106///
107/// This might introduce a bit overhead for IO operations, but it's the only way to implement
108/// timeout correctly. We used to implement timeout layer in zero cost way that only stores
109/// a [`std::time::Instant`] and check the timeout by comparing the instant with current time.
110/// However, it doesn't work for all cases.
111///
112/// For examples, users TCP connection could be in [Busy ESTAB](https://blog.cloudflare.com/when-tcp-sockets-refuse-to-die) state. In this state, no IO event will be emitted. The runtime
113/// will never poll our future again. From the application side, this future is hanging forever
114/// until this TCP connection is closed for reaching the linux [net.ipv4.tcp_retries2](https://man7.org/linux/man-pages/man7/tcp.7.html) times.
115#[derive(Clone)]
116pub struct TimeoutLayer {
117    timeout: Duration,
118    io_timeout: Duration,
119}
120
121impl Default for TimeoutLayer {
122    fn default() -> Self {
123        Self {
124            timeout: Duration::from_secs(60),
125            io_timeout: Duration::from_secs(10),
126        }
127    }
128}
129
130impl TimeoutLayer {
131    /// Create a new `TimeoutLayer` with default settings.
132    pub fn new() -> Self {
133        Self::default()
134    }
135
136    /// Set timeout for TimeoutLayer with given value.
137    ///
138    /// This timeout is for all non-io operations like `stat`, `delete`.
139    pub fn with_timeout(mut self, timeout: Duration) -> Self {
140        self.timeout = timeout;
141        self
142    }
143
144    /// Set io timeout for TimeoutLayer with given value.
145    ///
146    /// This timeout is for all io operations like `read`, `Reader::read` and `Writer::write`.
147    pub fn with_io_timeout(mut self, timeout: Duration) -> Self {
148        self.io_timeout = timeout;
149        self
150    }
151
152    /// Set speed for TimeoutLayer with given value.
153    ///
154    /// # Notes
155    ///
156    /// The speed should be the lower bound of the IO speed. Set this value too
157    /// large could result in all write operations failing.
158    ///
159    /// # Panics
160    ///
161    /// This function will panic if speed is 0.
162    #[deprecated(note = "with speed is not supported anymore, please use with_io_timeout instead")]
163    pub fn with_speed(self, _: u64) -> Self {
164        self
165    }
166}
167
168impl<A: Access> Layer<A> for TimeoutLayer {
169    type LayeredAccess = TimeoutAccessor<A>;
170
171    fn layer(&self, inner: A) -> Self::LayeredAccess {
172        let info = inner.info();
173        info.update_executor(|exec| {
174            Executor::with(TimeoutExecutor::new(exec.into_inner(), self.io_timeout))
175        });
176
177        TimeoutAccessor {
178            inner,
179
180            timeout: self.timeout,
181            io_timeout: self.io_timeout,
182        }
183    }
184}
185
186#[derive(Debug, Clone)]
187pub struct TimeoutAccessor<A: Access> {
188    inner: A,
189
190    timeout: Duration,
191    io_timeout: Duration,
192}
193
194impl<A: Access> TimeoutAccessor<A> {
195    async fn timeout<F: Future<Output = Result<T>>, T>(&self, op: Operation, fut: F) -> Result<T> {
196        tokio::time::timeout(self.timeout, fut).await.map_err(|_| {
197            Error::new(ErrorKind::Unexpected, "operation timeout reached")
198                .with_operation(op)
199                .with_context("timeout", self.timeout.as_secs_f64().to_string())
200                .set_temporary()
201        })?
202    }
203
204    async fn io_timeout<F: Future<Output = Result<T>>, T>(
205        &self,
206        op: Operation,
207        fut: F,
208    ) -> Result<T> {
209        tokio::time::timeout(self.io_timeout, fut)
210            .await
211            .map_err(|_| {
212                Error::new(ErrorKind::Unexpected, "io timeout reached")
213                    .with_operation(op)
214                    .with_context("timeout", self.io_timeout.as_secs_f64().to_string())
215                    .set_temporary()
216            })?
217    }
218}
219
220impl<A: Access> LayeredAccess for TimeoutAccessor<A> {
221    type Inner = A;
222    type Reader = TimeoutWrapper<A::Reader>;
223    type Writer = TimeoutWrapper<A::Writer>;
224    type Lister = TimeoutWrapper<A::Lister>;
225    type Deleter = TimeoutWrapper<A::Deleter>;
226
227    fn inner(&self) -> &Self::Inner {
228        &self.inner
229    }
230
231    async fn create_dir(&self, path: &str, args: OpCreateDir) -> Result<RpCreateDir> {
232        self.timeout(Operation::CreateDir, self.inner.create_dir(path, args))
233            .await
234    }
235
236    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
237        self.io_timeout(Operation::Read, self.inner.read(path, args))
238            .await
239            .map(|(rp, r)| (rp, TimeoutWrapper::new(r, self.io_timeout)))
240    }
241
242    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
243        self.io_timeout(Operation::Write, self.inner.write(path, args))
244            .await
245            .map(|(rp, r)| (rp, TimeoutWrapper::new(r, self.io_timeout)))
246    }
247
248    async fn copy(&self, from: &str, to: &str, args: OpCopy) -> Result<RpCopy> {
249        self.timeout(Operation::Copy, self.inner.copy(from, to, args))
250            .await
251    }
252
253    async fn rename(&self, from: &str, to: &str, args: OpRename) -> Result<RpRename> {
254        self.timeout(Operation::Rename, self.inner.rename(from, to, args))
255            .await
256    }
257
258    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
259        self.timeout(Operation::Stat, self.inner.stat(path, args))
260            .await
261    }
262
263    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
264        self.timeout(Operation::Delete, self.inner.delete())
265            .await
266            .map(|(rp, r)| (rp, TimeoutWrapper::new(r, self.io_timeout)))
267    }
268
269    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
270        self.io_timeout(Operation::List, self.inner.list(path, args))
271            .await
272            .map(|(rp, r)| (rp, TimeoutWrapper::new(r, self.io_timeout)))
273    }
274
275    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
276        self.timeout(Operation::Presign, self.inner.presign(path, args))
277            .await
278    }
279}
280
281pub struct TimeoutExecutor {
282    exec: Arc<dyn Execute>,
283    timeout: Duration,
284}
285
286impl TimeoutExecutor {
287    pub fn new(exec: Arc<dyn Execute>, timeout: Duration) -> Self {
288        Self { exec, timeout }
289    }
290}
291
292impl Execute for TimeoutExecutor {
293    fn execute(&self, f: BoxedStaticFuture<()>) {
294        self.exec.execute(f)
295    }
296
297    fn timeout(&self) -> Option<BoxedStaticFuture<()>> {
298        Some(Box::pin(tokio::time::sleep(self.timeout)))
299    }
300}
301
302pub struct TimeoutWrapper<R> {
303    inner: R,
304
305    timeout: Duration,
306}
307
308impl<R> TimeoutWrapper<R> {
309    fn new(inner: R, timeout: Duration) -> Self {
310        Self { inner, timeout }
311    }
312
313    #[inline]
314    async fn io_timeout<F: Future<Output = Result<T>>, T>(
315        timeout: Duration,
316        op: &'static str,
317        fut: F,
318    ) -> Result<T> {
319        tokio::time::timeout(timeout, fut).await.map_err(|_| {
320            Error::new(ErrorKind::Unexpected, "io operation timeout reached")
321                .with_operation(op)
322                .with_context("timeout", timeout.as_secs_f64().to_string())
323                .set_temporary()
324        })?
325    }
326}
327
328impl<R: oio::Read> oio::Read for TimeoutWrapper<R> {
329    async fn read(&mut self) -> Result<Buffer> {
330        let fut = self.inner.read();
331        Self::io_timeout(self.timeout, Operation::Read.into_static(), fut).await
332    }
333}
334
335impl<R: oio::Write> oio::Write for TimeoutWrapper<R> {
336    async fn write(&mut self, bs: Buffer) -> Result<()> {
337        let fut = self.inner.write(bs);
338        Self::io_timeout(self.timeout, Operation::Write.into_static(), fut).await
339    }
340
341    async fn close(&mut self) -> Result<Metadata> {
342        let fut = self.inner.close();
343        Self::io_timeout(self.timeout, Operation::Write.into_static(), fut).await
344    }
345
346    async fn abort(&mut self) -> Result<()> {
347        let fut = self.inner.abort();
348        Self::io_timeout(self.timeout, Operation::Write.into_static(), fut).await
349    }
350}
351
352impl<R: oio::List> oio::List for TimeoutWrapper<R> {
353    async fn next(&mut self) -> Result<Option<oio::Entry>> {
354        let fut = self.inner.next();
355        Self::io_timeout(self.timeout, Operation::List.into_static(), fut).await
356    }
357}
358
359impl<R: oio::Delete> oio::Delete for TimeoutWrapper<R> {
360    fn delete(&mut self, path: &str, args: OpDelete) -> Result<()> {
361        self.inner.delete(path, args)
362    }
363
364    async fn flush(&mut self) -> Result<usize> {
365        let fut = self.inner.flush();
366        Self::io_timeout(self.timeout, Operation::Delete.into_static(), fut).await
367    }
368}
369
370#[cfg(test)]
371mod tests {
372    use std::future::Future;
373    use std::future::pending;
374    use std::sync::Arc;
375    use std::time::Duration;
376
377    use futures::StreamExt;
378    use tokio::time::sleep;
379    use tokio::time::timeout;
380
381    use crate::layers::TimeoutLayer;
382    use crate::layers::TypeEraseLayer;
383    use crate::raw::*;
384    use crate::*;
385
386    #[derive(Debug, Clone, Default)]
387    struct MockService;
388
389    impl Access for MockService {
390        type Reader = MockReader;
391        type Writer = ();
392        type Lister = MockLister;
393        type Deleter = ();
394
395        fn info(&self) -> Arc<AccessorInfo> {
396            let am = AccessorInfo::default();
397            am.set_native_capability(Capability {
398                read: true,
399                delete: true,
400                ..Default::default()
401            });
402
403            am.into()
404        }
405
406        /// This function will build a reader that always return pending.
407        async fn read(&self, _: &str, _: OpRead) -> Result<(RpRead, Self::Reader)> {
408            Ok((RpRead::new(), MockReader))
409        }
410
411        /// This function will never return.
412        async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
413            sleep(Duration::from_secs(u64::MAX)).await;
414
415            Ok((RpDelete::default(), ()))
416        }
417
418        async fn list(&self, _: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
419            Ok((RpList::default(), MockLister))
420        }
421    }
422
423    #[derive(Debug, Clone, Default)]
424    struct MockReader;
425
426    impl oio::Read for MockReader {
427        fn read(&mut self) -> impl Future<Output = Result<Buffer>> {
428            pending()
429        }
430    }
431
432    #[derive(Debug, Clone, Default)]
433    struct MockLister;
434
435    impl oio::List for MockLister {
436        fn next(&mut self) -> impl Future<Output = Result<Option<oio::Entry>>> {
437            pending()
438        }
439    }
440
441    #[tokio::test]
442    async fn test_operation_timeout() {
443        let acc = Arc::new(TypeEraseLayer.layer(MockService)) as Accessor;
444        let op = Operator::from_inner(acc)
445            .layer(TimeoutLayer::new().with_timeout(Duration::from_secs(1)));
446
447        let fut = async {
448            let res = op.delete("test").await;
449            assert!(res.is_err());
450            let err = res.unwrap_err();
451            assert_eq!(err.kind(), ErrorKind::Unexpected);
452            assert!(err.to_string().contains("timeout"))
453        };
454
455        timeout(Duration::from_secs(2), fut)
456            .await
457            .expect("this test should not exceed 2 seconds")
458    }
459
460    #[tokio::test]
461    async fn test_io_timeout() {
462        let acc = Arc::new(TypeEraseLayer.layer(MockService)) as Accessor;
463        let op = Operator::from_inner(acc)
464            .layer(TimeoutLayer::new().with_io_timeout(Duration::from_secs(1)));
465
466        let reader = op.reader("test").await.unwrap();
467
468        let res = reader.read(0..4).await;
469        assert!(res.is_err());
470        let err = res.unwrap_err();
471        assert_eq!(err.kind(), ErrorKind::Unexpected);
472        assert!(err.to_string().contains("timeout"))
473    }
474
475    #[tokio::test]
476    async fn test_list_timeout() {
477        let acc = Arc::new(TypeEraseLayer.layer(MockService)) as Accessor;
478        let op = Operator::from_inner(acc).layer(
479            TimeoutLayer::new()
480                .with_timeout(Duration::from_secs(1))
481                .with_io_timeout(Duration::from_secs(1)),
482        );
483
484        let mut lister = op.lister("test").await.unwrap();
485
486        let res = lister.next().await.unwrap();
487        assert!(res.is_err());
488        let err = res.unwrap_err();
489        assert_eq!(err.kind(), ErrorKind::Unexpected);
490        assert!(err.to_string().contains("timeout"))
491    }
492
493    #[tokio::test]
494    async fn test_list_timeout_raw() {
495        use oio::List;
496
497        let acc = MockService;
498        let timeout_layer = TimeoutLayer::new()
499            .with_timeout(Duration::from_secs(1))
500            .with_io_timeout(Duration::from_secs(1));
501        let timeout_acc = timeout_layer.layer(acc);
502
503        let (_, mut lister) = Access::list(&timeout_acc, "test", OpList::default())
504            .await
505            .unwrap();
506
507        let res = lister.next().await;
508        assert!(res.is_err());
509        let err = res.unwrap_err();
510        assert_eq!(err.kind(), ErrorKind::Unexpected);
511        assert!(err.to_string().contains("timeout"));
512    }
513}
```
