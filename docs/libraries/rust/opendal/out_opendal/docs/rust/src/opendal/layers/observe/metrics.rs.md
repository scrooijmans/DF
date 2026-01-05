# 

opendal/layers/observe/

metrics.rs

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
19use std::fmt::Formatter;
20use std::pin::Pin;
21use std::sync::Arc;
22use std::task::Context;
23use std::task::Poll;
24use std::task::ready;
25use std::time::Duration;
26use std::time::Instant;
27
28use futures::Stream;
29use futures::StreamExt;
30use http::StatusCode;
31
32use crate::raw::*;
33use crate::*;
34
35const KIB: f64 = 1024.0;
36const MIB: f64 = 1024.0 * KIB;
37const GIB: f64 = 1024.0 * MIB;
38
39/// Buckets for data size metrics like OperationBytes
40/// Covers typical file and object sizes from small files to large objects
41pub const DEFAULT_BYTES_BUCKETS: &[f64] = &[
42    4.0 * KIB,   // Small files
43    64.0 * KIB,  // File system block size
44    256.0 * KIB, //
45    1.0 * MIB,   // Common size threshold for many systems
46    4.0 * MIB,   // Best size for most http based systems
47    16.0 * MIB,  //
48    64.0 * MIB,  // Widely used threshold for multipart uploads
49    256.0 * MIB, //
50    1.0 * GIB,   // Considered large for many systems
51    5.0 * GIB,   // Maximum size in single upload for many cloud storage services
52];
53
54/// Buckets for data transfer rate metrics like OperationBytesRate
55///
56/// Covers various network speeds from slow connections to high-speed transfers
57///
58/// Note: this is for single operation rate, not for total bandwidth.
59pub const DEFAULT_BYTES_RATE_BUCKETS: &[f64] = &[
60    // Low-speed network range (mobile/weak connections)
61    8.0 * KIB,   // ~64Kbps - 2G networks
62    32.0 * KIB,  // ~256Kbps - 3G networks
63    128.0 * KIB, // ~1Mbps - Basic broadband
64    // Standard broadband range
65    1.0 * MIB,  // ~8Mbps - Entry-level broadband
66    8.0 * MIB,  // ~64Mbps - Fast broadband
67    32.0 * MIB, // ~256Mbps - Gigabit broadband
68    // High-performance network range
69    128.0 * MIB, // ~1Gbps - Standard datacenter
70    512.0 * MIB, // ~4Gbps - Fast datacenter
71    2.0 * GIB,   // ~16Gbps - High-end interconnects
72    // Ultra-high-speed range
73    8.0 * GIB,  // ~64Gbps - InfiniBand/RDMA
74    32.0 * GIB, // ~256Gbps - Top-tier datacenters
75];
76
77/// Buckets for batch operation entry counts (OperationEntriesCount)
78/// Covers scenarios from single entry operations to large batch operations
79pub const DEFAULT_ENTRIES_BUCKETS: &[f64] = &[
80    1.0,     // Single item operations
81    5.0,     // Very small batches
82    10.0,    // Small batches
83    50.0,    // Medium batches
84    100.0,   // Standard batch size
85    500.0,   // Large batches
86    1000.0,  // Very large batches, API limits for some services
87    5000.0,  // Huge batches, multi-page operations
88    10000.0, // Extremely large operations, multi-request batches
89];
90
91/// Buckets for batch operation processing rates (OperationEntriesRate)
92/// Measures how many entries can be processed per second
93pub const DEFAULT_ENTRIES_RATE_BUCKETS: &[f64] = &[
94    1.0,     // Slowest processing, heavy operations per entry
95    10.0,    // Slow processing, complex operations
96    50.0,    // Moderate processing speed
97    100.0,   // Good processing speed, efficient operations
98    500.0,   // Fast processing, optimized operations
99    1000.0,  // Very fast processing, simple operations
100    5000.0,  // Extremely fast processing, bulk operations
101    10000.0, // Maximum speed, listing operations, local systems
102];
103
104/// Buckets for operation duration metrics like OperationDurationSeconds
105/// Covers timeframes from fast metadata operations to long-running transfers
106pub const DEFAULT_DURATION_SECONDS_BUCKETS: &[f64] = &[
107    0.001, // 1ms - Fastest operations, cached responses
108    0.01,  // 10ms - Fast metadata operations, local operations
109    0.05,  // 50ms - Quick operations, nearby cloud resources
110    0.1,   // 100ms - Standard API response times, typical cloud latency
111    0.25,  // 250ms - Medium operations, small data transfers
112    0.5,   // 500ms - Medium-long operations, larger metadata operations
113    1.0,   // 1s - Long operations, small file transfers
114    2.5,   // 2.5s - Extended operations, medium file transfers
115    5.0,   // 5s - Long-running operations, large transfers
116    10.0,  // 10s - Very long operations, very large transfers
117    30.0,  // 30s - Extended operations, complex batch processes
118    60.0,  // 1min - Near timeout operations, extremely large transfers
119];
120
121/// Buckets for time to first byte metrics like OperationTtfbSeconds
122/// Focuses on initial response times, which are typically shorter than full operations
123pub const DEFAULT_TTFB_BUCKETS: &[f64] = &[
124    0.001, // 1ms - Cached or local resources
125    0.01,  // 10ms - Very fast responses, same region
126    0.025, // 25ms - Fast responses, optimized configurations
127    0.05,  // 50ms - Good response times, standard configurations
128    0.1,   // 100ms - Average response times for cloud storage
129    0.2,   // 200ms - Slower responses, cross-region or throttled
130    0.4,   // 400ms - Poor response times, network congestion
131    0.8,   // 800ms - Very slow responses, potential issues
132    1.6,   // 1.6s - Problematic responses, retry territory
133    3.2,   // 3.2s - Critical latency issues, close to timeouts
134];
135
136/// The metric label for the scheme like s3, fs, cos.
137pub static LABEL_SCHEME: &str = "scheme";
138/// The metric label for the namespace like bucket name in s3.
139pub static LABEL_NAMESPACE: &str = "namespace";
140/// The metric label for the root path.
141pub static LABEL_ROOT: &str = "root";
142/// The metric label for the operation like read, write, list.
143pub static LABEL_OPERATION: &str = "operation";
144/// The metric label for the error.
145pub static LABEL_ERROR: &str = "error";
146/// The metric label for the http code.
147pub static LABEL_STATUS_CODE: &str = "status_code";
148
149/// MetricLabels are the labels for the metrics.
150#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
151pub struct MetricLabels {
152    /// The storage scheme identifier (e.g., "s3", "gcs", "azblob", "fs").
153    /// Used to differentiate between different storage backends.
154    pub scheme: &'static str,
155    /// The storage namespace (e.g., bucket name, container name).
156    /// Identifies the specific storage container being accessed.
157    pub namespace: Arc<str>,
158    /// The root path within the namespace that was configured.
159    /// Used to track operations within a specific path prefix.
160    pub root: Arc<str>,
161    /// The operation being performed (e.g., "read", "write", "list").
162    /// Identifies which API operation generated this metric.
163    pub operation: &'static str,
164    /// The specific error kind that occurred during an operation.
165    /// Only populated for `OperationErrorsTotal` metric.
166    /// Used to track frequency of specific error types.
167    pub error: Option<ErrorKind>,
168    /// The HTTP status code received in an error response.
169    /// Only populated for `HttpStatusErrorsTotal` metric.
170    /// Used to track frequency of specific HTTP error status codes.
171    pub status_code: Option<StatusCode>,
172}
173
174impl MetricLabels {
175    /// Create a new set of MetricLabels.
176    fn new(info: Arc<AccessorInfo>, op: &'static str) -> Self {
177        MetricLabels {
178            scheme: info.scheme(),
179            namespace: info.name(),
180            root: info.root(),
181            operation: op,
182            ..MetricLabels::default()
183        }
184    }
185
186    /// Add error to the metric labels.
187    fn with_error(mut self, err: ErrorKind) -> Self {
188        self.error = Some(err);
189        self
190    }
191
192    /// Add status code to the metric labels.
193    fn with_status_code(mut self, code: StatusCode) -> Self {
194        self.status_code = Some(code);
195        self
196    }
197}
198
199/// MetricValue is the value the opendal sends to the metrics impls.
200///
201/// Metrics impls can be `prometheus_client`, `metrics` etc.
202///
203/// Every metrics impls SHOULD implement observe over the MetricValue to make
204/// sure they provide the consistent metrics for users.
205#[non_exhaustive]
206#[derive(Debug, Clone, Copy)]
207pub enum MetricValue {
208    /// Record the size of data processed in bytes.
209    /// Metrics impl: Update a Histogram with the given byte count.
210    OperationBytes(u64),
211    /// Record the rate of data processing in bytes/second.
212    /// Metrics impl: Update a Histogram with the calculated rate value.
213    OperationBytesRate(f64),
214    /// Record the number of entries (files, objects, keys) processed.
215    /// Metrics impl: Update a Histogram with the entry count.
216    OperationEntries(u64),
217    /// Record the rate of entries processing in entries/second.
218    /// Metrics impl: Update a Histogram with the calculated rate value.
219    OperationEntriesRate(f64),
220    /// Record the total duration of an operation.
221    /// Metrics impl: Update a Histogram with the duration converted to seconds (as f64).
222    OperationDurationSeconds(Duration),
223    /// Increment the counter for operation errors.
224    /// Metrics impl: Increment a Counter by 1.
225    OperationErrorsTotal,
226    /// Update the current number of executing operations.
227    /// Metrics impl: Add the value (positive or negative) to a Gauge.
228    OperationExecuting(isize),
229    /// Record the time to first byte duration.
230    /// Metrics impl: Update a Histogram with the duration converted to seconds (as f64).
231    OperationTtfbSeconds(Duration),
232    /// Update the current number of executing HTTP requests.
233    /// Metrics impl: Add the value (positive or negative) to a Gauge.
234    HttpExecuting(isize),
235    /// Record the size of HTTP request body in bytes.
236    /// Metrics impl: Update a Histogram with the given byte count.
237    HttpRequestBytes(u64),
238    /// Record the rate of HTTP request data in bytes/second.
239    /// Metrics impl: Update a Histogram with the calculated rate value.
240    HttpRequestBytesRate(f64),
241    /// Record the duration of sending an HTTP request (until first byte received).
242    /// Metrics impl: Update a Histogram with the duration converted to seconds (as f64).
243    HttpRequestDurationSeconds(Duration),
244    /// Record the size of HTTP response body in bytes.
245    /// Metrics impl: Update a Histogram with the given byte count.
246    HttpResponseBytes(u64),
247    /// Record the rate of HTTP response data in bytes/second.
248    /// Metrics impl: Update a Histogram with the calculated rate value.
249    HttpResponseBytesRate(f64),
250    /// Record the duration of receiving an HTTP response (from first byte to last).
251    /// Metrics impl: Update a Histogram with the duration converted to seconds (as f64).
252    HttpResponseDurationSeconds(Duration),
253    /// Increment the counter for HTTP connection errors.
254    /// Metrics impl: Increment a Counter by 1.
255    HttpConnectionErrorsTotal,
256    /// Increment the counter for HTTP status errors (non-2xx responses).
257    /// Metrics impl: Increment a Counter by 1.
258    HttpStatusErrorsTotal,
259}
260
261impl MetricValue {
262    /// Returns the full metric name for this metric value.
263    pub fn name(&self) -> &'static str {
264        match self {
265            MetricValue::OperationBytes(_) => "opendal_operation_bytes",
266            MetricValue::OperationBytesRate(_) => "opendal_operation_bytes_rate",
267            MetricValue::OperationEntries(_) => "opendal_operation_entries",
268            MetricValue::OperationEntriesRate(_) => "opendal_operation_entries_rate",
269            MetricValue::OperationDurationSeconds(_) => "opendal_operation_duration_seconds",
270            MetricValue::OperationErrorsTotal => "opendal_operation_errors_total",
271            MetricValue::OperationExecuting(_) => "opendal_operation_executing",
272            MetricValue::OperationTtfbSeconds(_) => "opendal_operation_ttfb_seconds",
273
274            MetricValue::HttpConnectionErrorsTotal => "opendal_http_connection_errors_total",
275            MetricValue::HttpStatusErrorsTotal => "opendal_http_status_errors_total",
276            MetricValue::HttpExecuting(_) => "opendal_http_executing",
277            MetricValue::HttpRequestBytes(_) => "opendal_http_request_bytes",
278            MetricValue::HttpRequestBytesRate(_) => "opendal_http_request_bytes_rate",
279            MetricValue::HttpRequestDurationSeconds(_) => "opendal_http_request_duration_seconds",
280            MetricValue::HttpResponseBytes(_) => "opendal_http_response_bytes",
281            MetricValue::HttpResponseBytesRate(_) => "opendal_http_response_bytes_rate",
282            MetricValue::HttpResponseDurationSeconds(_) => "opendal_http_response_duration_seconds",
283        }
284    }
285
286    /// Returns the metric name along with unit for this metric value.
287    ///
288    /// # Notes
289    ///
290    /// This API is designed for the metrics impls that unit aware. They will handle the names by themselves like append `_total` for counters.
291    pub fn name_with_unit(&self) -> (&'static str, Option<&'static str>) {
292        match self {
293            MetricValue::OperationBytes(_) => ("opendal_operation", Some("bytes")),
294            MetricValue::OperationBytesRate(_) => ("opendal_operation_bytes_rate", None),
295            MetricValue::OperationEntries(_) => ("opendal_operation_entries", None),
296            MetricValue::OperationEntriesRate(_) => ("opendal_operation_entries_rate", None),
297            MetricValue::OperationDurationSeconds(_) => {
298                ("opendal_operation_duration", Some("seconds"))
299            }
300            MetricValue::OperationErrorsTotal => ("opendal_operation_errors", None),
301            MetricValue::OperationExecuting(_) => ("opendal_operation_executing", None),
302            MetricValue::OperationTtfbSeconds(_) => ("opendal_operation_ttfb", Some("seconds")),
303
304            MetricValue::HttpConnectionErrorsTotal => ("opendal_http_connection_errors", None),
305            MetricValue::HttpStatusErrorsTotal => ("opendal_http_status_errors", None),
306            MetricValue::HttpExecuting(_) => ("opendal_http_executing", None),
307            MetricValue::HttpRequestBytes(_) => ("opendal_http_request", Some("bytes")),
308            MetricValue::HttpRequestBytesRate(_) => ("opendal_http_request_bytes_rate", None),
309            MetricValue::HttpRequestDurationSeconds(_) => {
310                ("opendal_http_request_duration", Some("seconds"))
311            }
312            MetricValue::HttpResponseBytes(_) => ("opendal_http_response", Some("bytes")),
313            MetricValue::HttpResponseBytesRate(_) => ("opendal_http_response_bytes_rate", None),
314            MetricValue::HttpResponseDurationSeconds(_) => {
315                ("opendal_http_response_duration", Some("seconds"))
316            }
317        }
318    }
319
320    /// Returns the help text for this metric value.
321    pub fn help(&self) -> &'static str {
322        match self {
323            MetricValue::OperationBytes(_) => {
324                "Current operation size in bytes, represents the size of data being processed in the current operation"
325            }
326            MetricValue::OperationBytesRate(_) => {
327                "Histogram of data processing rates in bytes per second within individual operations"
328            }
329            MetricValue::OperationEntries(_) => {
330                "Current operation size in entries, represents the entries being processed in the current operation"
331            }
332            MetricValue::OperationEntriesRate(_) => {
333                "Histogram of entries processing rates in entries per second within individual operations"
334            }
335            MetricValue::OperationDurationSeconds(_) => {
336                "Duration of operations in seconds, measured from start to completion"
337            }
338            MetricValue::OperationErrorsTotal => "Total number of failed operations",
339            MetricValue::OperationExecuting(_) => "Number of operations currently being executed",
340            MetricValue::OperationTtfbSeconds(_) => "Time to first byte in seconds for operations",
341
342            MetricValue::HttpConnectionErrorsTotal => {
343                "Total number of HTTP requests that failed before receiving a response (DNS failures, connection refused, timeouts, TLS errors)"
344            }
345            MetricValue::HttpStatusErrorsTotal => {
346                "Total number of HTTP requests that received error status codes (non-2xx responses)"
347            }
348            MetricValue::HttpExecuting(_) => {
349                "Number of HTTP requests currently in flight from this client"
350            }
351            MetricValue::HttpRequestBytes(_) => "Histogram of HTTP request body sizes in bytes",
352            MetricValue::HttpRequestBytesRate(_) => {
353                "Histogram of HTTP request bytes per second rates"
354            }
355            MetricValue::HttpRequestDurationSeconds(_) => {
356                "Histogram of time durations in seconds spent sending HTTP requests, from first byte sent to receiving the first byte"
357            }
358            MetricValue::HttpResponseBytes(_) => "Histogram of HTTP response body sizes in bytes",
359            MetricValue::HttpResponseBytesRate(_) => {
360                "Histogram of HTTP response bytes per second rates"
361            }
362            MetricValue::HttpResponseDurationSeconds(_) => {
363                "Histogram of time durations in seconds spent receiving HTTP responses, from first byte received to last byte received"
364            }
365        }
366    }
367}
368
369/// The interceptor for metrics.
370///
371/// All metrics related libs should implement this trait to observe opendal's internal operations.
372pub trait MetricsIntercept: Debug + Clone + Send + Sync + Unpin + 'static {
373    /// Observe the metric value.
374    fn observe(&self, labels: MetricLabels, value: MetricValue) {
375        let _ = (labels, value);
376    }
377}
378
379/// The metrics layer for opendal.
380#[derive(Clone, Debug)]
381pub struct MetricsLayer<I: MetricsIntercept> {
382    interceptor: I,
383}
384
385impl<I: MetricsIntercept> MetricsLayer<I> {
386    /// Create a new metrics layer.
387    pub fn new(interceptor: I) -> Self {
388        Self { interceptor }
389    }
390}
391
392impl<A: Access, I: MetricsIntercept> Layer<A> for MetricsLayer<I> {
393    type LayeredAccess = MetricsAccessor<A, I>;
394
395    fn layer(&self, inner: A) -> Self::LayeredAccess {
396        let info = inner.info();
397
398        // Update http client with metrics http fetcher.
399        info.update_http_client(|client| {
400            HttpClient::with(MetricsHttpFetcher {
401                inner: client.into_inner(),
402                info: info.clone(),
403                interceptor: self.interceptor.clone(),
404            })
405        });
406
407        MetricsAccessor {
408            inner,
409            info,
410            interceptor: self.interceptor.clone(),
411        }
412    }
413}
414
415/// The metrics http fetcher for opendal.
416pub struct MetricsHttpFetcher<I: MetricsIntercept> {
417    inner: HttpFetcher,
418    info: Arc<AccessorInfo>,
419    interceptor: I,
420}
421
422impl<I: MetricsIntercept> HttpFetch for MetricsHttpFetcher<I> {
423    async fn fetch(&self, req: http::Request<Buffer>) -> Result<http::Response<HttpBody>> {
424        let labels = MetricLabels::new(
425            self.info.clone(),
426            req.extensions()
427                .get::<Operation>()
428                .copied()
429                .map(Operation::into_static)
430                .unwrap_or("unknown"),
431        );
432
433        let start = Instant::now();
434        let req_size = req.body().len();
435
436        self.interceptor
437            .observe(labels.clone(), MetricValue::HttpExecuting(1));
438
439        let res = self.inner.fetch(req).await;
440        let req_duration = start.elapsed();
441
442        match res {
443            Err(err) => {
444                self.interceptor
445                    .observe(labels.clone(), MetricValue::HttpExecuting(-1));
446                self.interceptor
447                    .observe(labels, MetricValue::HttpConnectionErrorsTotal);
448                Err(err)
449            }
450            Ok(resp) if resp.status().is_client_error() && resp.status().is_server_error() => {
451                self.interceptor
452                    .observe(labels.clone(), MetricValue::HttpExecuting(-1));
453                self.interceptor.observe(
454                    labels.with_status_code(resp.status()),
455                    MetricValue::HttpStatusErrorsTotal,
456                );
457                Ok(resp)
458            }
459            Ok(resp) => {
460                self.interceptor.observe(
461                    labels.clone(),
462                    MetricValue::HttpRequestBytes(req_size as u64),
463                );
464                self.interceptor.observe(
465                    labels.clone(),
466                    MetricValue::HttpRequestBytesRate(req_size as f64 / req_duration.as_secs_f64()),
467                );
468                self.interceptor.observe(
469                    labels.clone(),
470                    MetricValue::HttpRequestDurationSeconds(req_duration),
471                );
472
473                let (parts, body) = resp.into_parts();
474                let body = body.map_inner(|s| {
475                    Box::new(MetricsStream {
476                        inner: s,
477                        interceptor: self.interceptor.clone(),
478                        labels: labels.clone(),
479                        size: 0,
480                        start: Instant::now(),
481                    })
482                });
483
484                Ok(http::Response::from_parts(parts, body))
485            }
486        }
487    }
488}
489
490pub struct MetricsStream<S, I> {
491    inner: S,
492    interceptor: I,
493
494    labels: MetricLabels,
495    size: u64,
496    start: Instant,
497}
498
499impl<S, I> Stream for MetricsStream<S, I>
500where
501    S: Stream<Item = Result<Buffer>> + Unpin + 'static,
502    I: MetricsIntercept,
503{
504    type Item = Result<Buffer>;
505
506    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
507        match ready!(self.inner.poll_next_unpin(cx)) {
508            Some(Ok(bs)) => {
509                self.size += bs.len() as u64;
510                Poll::Ready(Some(Ok(bs)))
511            }
512            Some(Err(err)) => Poll::Ready(Some(Err(err))),
513            None => {
514                let resp_size = self.size;
515                let resp_duration = self.start.elapsed();
516
517                self.interceptor.observe(
518                    self.labels.clone(),
519                    MetricValue::HttpResponseBytes(resp_size),
520                );
521                self.interceptor.observe(
522                    self.labels.clone(),
523                    MetricValue::HttpResponseBytesRate(
524                        resp_size as f64 / resp_duration.as_secs_f64(),
525                    ),
526                );
527                self.interceptor.observe(
528                    self.labels.clone(),
529                    MetricValue::HttpResponseDurationSeconds(resp_duration),
530                );
531                self.interceptor
532                    .observe(self.labels.clone(), MetricValue::HttpExecuting(-1));
533
534                Poll::Ready(None)
535            }
536        }
537    }
538}
539
540/// The metrics accessor for opendal.
541pub struct MetricsAccessor<A: Access, I: MetricsIntercept> {
542    inner: A,
543    info: Arc<AccessorInfo>,
544    interceptor: I,
545}
546
547impl<A: Access, I: MetricsIntercept> Debug for MetricsAccessor<A, I> {
548    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
549        f.debug_struct("MetricsAccessor")
550            .field("inner", &self.inner)
551            .finish_non_exhaustive()
552    }
553}
554
555impl<A: Access, I: MetricsIntercept> LayeredAccess for MetricsAccessor<A, I> {
556    type Inner = A;
557    type Reader = MetricsWrapper<A::Reader, I>;
558    type Writer = MetricsWrapper<A::Writer, I>;
559    type Lister = MetricsWrapper<A::Lister, I>;
560    type Deleter = MetricsWrapper<A::Deleter, I>;
561
562    fn inner(&self) -> &Self::Inner {
563        &self.inner
564    }
565
566    async fn create_dir(&self, path: &str, args: OpCreateDir) -> Result<RpCreateDir> {
567        let labels = MetricLabels::new(self.info.clone(), Operation::CreateDir.into_static());
568
569        let start = Instant::now();
570
571        self.interceptor
572            .observe(labels.clone(), MetricValue::OperationExecuting(1));
573
574        let res = self
575            .inner()
576            .create_dir(path, args)
577            .await
578            .inspect(|_| {
579                self.interceptor.observe(
580                    labels.clone(),
581                    MetricValue::OperationDurationSeconds(start.elapsed()),
582                );
583            })
584            .inspect_err(|err| {
585                self.interceptor.observe(
586                    labels.clone().with_error(err.kind()),
587                    MetricValue::OperationErrorsTotal,
588                );
589            });
590
591        self.interceptor
592            .observe(labels, MetricValue::OperationExecuting(-1));
593        res
594    }
595
596    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
597        let labels = MetricLabels::new(self.info.clone(), Operation::Read.into_static());
598
599        let start = Instant::now();
600
601        self.interceptor
602            .observe(labels.clone(), MetricValue::OperationExecuting(1));
603
604        let (rp, reader) = self
605            .inner
606            .read(path, args)
607            .await
608            .inspect(|_| {
609                self.interceptor.observe(
610                    labels.clone(),
611                    MetricValue::OperationTtfbSeconds(start.elapsed()),
612                );
613            })
614            .inspect_err(|err| {
615                self.interceptor.observe(
616                    labels.clone().with_error(err.kind()),
617                    MetricValue::OperationErrorsTotal,
618                );
619            })?;
620
621        Ok((
622            rp,
623            MetricsWrapper::new(reader, self.interceptor.clone(), labels, start),
624        ))
625    }
626
627    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
628        let labels = MetricLabels::new(self.info.clone(), Operation::Write.into_static());
629
630        let start = Instant::now();
631
632        self.interceptor
633            .observe(labels.clone(), MetricValue::OperationExecuting(1));
634
635        let (rp, writer) = self.inner.write(path, args).await.inspect_err(|err| {
636            self.interceptor.observe(
637                labels.clone().with_error(err.kind()),
638                MetricValue::OperationErrorsTotal,
639            );
640        })?;
641
642        Ok((
643            rp,
644            MetricsWrapper::new(writer, self.interceptor.clone(), labels, start),
645        ))
646    }
647
648    async fn copy(&self, from: &str, to: &str, args: OpCopy) -> Result<RpCopy> {
649        let labels = MetricLabels::new(self.info.clone(), Operation::Copy.into_static());
650
651        let start = Instant::now();
652
653        self.interceptor
654            .observe(labels.clone(), MetricValue::OperationExecuting(1));
655
656        let res = self
657            .inner()
658            .copy(from, to, args)
659            .await
660            .inspect(|_| {
661                self.interceptor.observe(
662                    labels.clone(),
663                    MetricValue::OperationDurationSeconds(start.elapsed()),
664                );
665            })
666            .inspect_err(|err| {
667                self.interceptor.observe(
668                    labels.clone().with_error(err.kind()),
669                    MetricValue::OperationErrorsTotal,
670                );
671            });
672
673        self.interceptor
674            .observe(labels, MetricValue::OperationExecuting(-1));
675        res
676    }
677
678    async fn rename(&self, from: &str, to: &str, args: OpRename) -> Result<RpRename> {
679        let labels = MetricLabels::new(self.info.clone(), Operation::Rename.into_static());
680
681        let start = Instant::now();
682
683        self.interceptor
684            .observe(labels.clone(), MetricValue::OperationExecuting(1));
685
686        let res = self
687            .inner()
688            .rename(from, to, args)
689            .await
690            .inspect(|_| {
691                self.interceptor.observe(
692                    labels.clone(),
693                    MetricValue::OperationDurationSeconds(start.elapsed()),
694                );
695            })
696            .inspect_err(|err| {
697                self.interceptor.observe(
698                    labels.clone().with_error(err.kind()),
699                    MetricValue::OperationErrorsTotal,
700                );
701            });
702
703        self.interceptor
704            .observe(labels, MetricValue::OperationExecuting(-1));
705        res
706    }
707
708    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
709        let labels = MetricLabels::new(self.info.clone(), Operation::Stat.into_static());
710
711        let start = Instant::now();
712
713        self.interceptor
714            .observe(labels.clone(), MetricValue::OperationExecuting(1));
715
716        let res = self
717            .inner()
718            .stat(path, args)
719            .await
720            .inspect(|_| {
721                self.interceptor.observe(
722                    labels.clone(),
723                    MetricValue::OperationDurationSeconds(start.elapsed()),
724                );
725            })
726            .inspect_err(|err| {
727                self.interceptor.observe(
728                    labels.clone().with_error(err.kind()),
729                    MetricValue::OperationErrorsTotal,
730                );
731            });
732
733        self.interceptor
734            .observe(labels, MetricValue::OperationExecuting(-1));
735        res
736    }
737
738    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
739        let labels = MetricLabels::new(self.info.clone(), Operation::Delete.into_static());
740
741        let start = Instant::now();
742
743        self.interceptor
744            .observe(labels.clone(), MetricValue::OperationExecuting(1));
745
746        let (rp, deleter) = self.inner.delete().await.inspect_err(|err| {
747            self.interceptor.observe(
748                labels.clone().with_error(err.kind()),
749                MetricValue::OperationErrorsTotal,
750            );
751        })?;
752
753        Ok((
754            rp,
755            MetricsWrapper::new(deleter, self.interceptor.clone(), labels, start),
756        ))
757    }
758
759    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
760        let labels = MetricLabels::new(self.info.clone(), Operation::List.into_static());
761
762        let start = Instant::now();
763
764        self.interceptor
765            .observe(labels.clone(), MetricValue::OperationExecuting(1));
766
767        let (rp, lister) = self.inner.list(path, args).await.inspect_err(|err| {
768            self.interceptor.observe(
769                labels.clone().with_error(err.kind()),
770                MetricValue::OperationErrorsTotal,
771            );
772        })?;
773
774        Ok((
775            rp,
776            MetricsWrapper::new(lister, self.interceptor.clone(), labels, start),
777        ))
778    }
779
780    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
781        let labels = MetricLabels::new(self.info.clone(), Operation::Presign.into_static());
782
783        let start = Instant::now();
784
785        self.interceptor
786            .observe(labels.clone(), MetricValue::OperationExecuting(1));
787
788        let res = self
789            .inner()
790            .presign(path, args)
791            .await
792            .inspect(|_| {
793                self.interceptor.observe(
794                    labels.clone(),
795                    MetricValue::OperationDurationSeconds(start.elapsed()),
796                );
797            })
798            .inspect_err(|err| {
799                self.interceptor.observe(
800                    labels.clone().with_error(err.kind()),
801                    MetricValue::OperationErrorsTotal,
802                );
803            });
804
805        self.interceptor
806            .observe(labels, MetricValue::OperationExecuting(-1));
807        res
808    }
809}
810
811pub struct MetricsWrapper<R, I: MetricsIntercept> {
812    inner: R,
813    interceptor: I,
814    labels: MetricLabels,
815
816    start: Instant,
817    size: u64,
818}
819
820impl<R, I: MetricsIntercept> Drop for MetricsWrapper<R, I> {
821    fn drop(&mut self) {
822        let size = self.size;
823        let duration = self.start.elapsed();
824
825        if self.labels.operation == Operation::Read.into_static()
826            || self.labels.operation == Operation::Write.into_static()
827        {
828            self.interceptor
829                .observe(self.labels.clone(), MetricValue::OperationBytes(self.size));
830            self.interceptor.observe(
831                self.labels.clone(),
832                MetricValue::OperationBytesRate(size as f64 / duration.as_secs_f64()),
833            );
834        } else {
835            self.interceptor.observe(
836                self.labels.clone(),
837                MetricValue::OperationEntries(self.size),
838            );
839            self.interceptor.observe(
840                self.labels.clone(),
841                MetricValue::OperationEntriesRate(size as f64 / duration.as_secs_f64()),
842            );
843        }
844
845        self.interceptor.observe(
846            self.labels.clone(),
847            MetricValue::OperationDurationSeconds(duration),
848        );
849        self.interceptor
850            .observe(self.labels.clone(), MetricValue::OperationExecuting(-1));
851    }
852}
853
854impl<R, I: MetricsIntercept> MetricsWrapper<R, I> {
855    fn new(inner: R, interceptor: I, labels: MetricLabels, start: Instant) -> Self {
856        Self {
857            inner,
858            interceptor,
859            labels,
860            start,
861            size: 0,
862        }
863    }
864}
865
866impl<R: oio::Read, I: MetricsIntercept> oio::Read for MetricsWrapper<R, I> {
867    async fn read(&mut self) -> Result<Buffer> {
868        self.inner
869            .read()
870            .await
871            .inspect(|bs| {
872                self.size += bs.len() as u64;
873            })
874            .inspect_err(|err| {
875                self.interceptor.observe(
876                    self.labels.clone().with_error(err.kind()),
877                    MetricValue::OperationErrorsTotal,
878                );
879            })
880    }
881}
882
883impl<R: oio::Write, I: MetricsIntercept> oio::Write for MetricsWrapper<R, I> {
884    async fn write(&mut self, bs: Buffer) -> Result<()> {
885        let size = bs.len();
886
887        self.inner
888            .write(bs)
889            .await
890            .inspect(|_| {
891                self.size += size as u64;
892            })
893            .inspect_err(|err| {
894                self.interceptor.observe(
895                    self.labels.clone().with_error(err.kind()),
896                    MetricValue::OperationErrorsTotal,
897                );
898            })
899    }
900
901    async fn close(&mut self) -> Result<Metadata> {
902        self.inner.close().await.inspect_err(|err| {
903            self.interceptor.observe(
904                self.labels.clone().with_error(err.kind()),
905                MetricValue::OperationErrorsTotal,
906            );
907        })
908    }
909
910    async fn abort(&mut self) -> Result<()> {
911        self.inner.abort().await.inspect_err(|err| {
912            self.interceptor.observe(
913                self.labels.clone().with_error(err.kind()),
914                MetricValue::OperationErrorsTotal,
915            );
916        })
917    }
918}
919
920impl<R: oio::List, I: MetricsIntercept> oio::List for MetricsWrapper<R, I> {
921    async fn next(&mut self) -> Result<Option<oio::Entry>> {
922        self.inner
923            .next()
924            .await
925            .inspect(|_| {
926                self.size += 1;
927            })
928            .inspect_err(|err| {
929                self.interceptor.observe(
930                    self.labels.clone().with_error(err.kind()),
931                    MetricValue::OperationErrorsTotal,
932                );
933            })
934    }
935}
936
937impl<R: oio::Delete, I: MetricsIntercept> oio::Delete for MetricsWrapper<R, I> {
938    fn delete(&mut self, path: &str, args: OpDelete) -> Result<()> {
939        self.inner
940            .delete(path, args)
941            .inspect(|_| {
942                self.size += 1;
943            })
944            .inspect_err(|err| {
945                self.interceptor.observe(
946                    self.labels.clone().with_error(err.kind()),
947                    MetricValue::OperationErrorsTotal,
948                );
949            })
950    }
951
952    async fn flush(&mut self) -> Result<usize> {
953        self.inner.flush().await.inspect_err(|err| {
954            self.interceptor.observe(
955                self.labels.clone().with_error(err.kind()),
956                MetricValue::OperationErrorsTotal,
957            );
958        })
959    }
960}
```
