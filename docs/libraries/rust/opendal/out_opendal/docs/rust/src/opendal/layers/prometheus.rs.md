# 

opendal/layers/

prometheus.rs

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
18use std::time::Duration;
19
20use prometheus::HistogramVec;
21use prometheus::Registry;
22use prometheus::core::AtomicI64;
23use prometheus::core::AtomicU64;
24use prometheus::core::GenericCounterVec;
25use prometheus::core::GenericGaugeVec;
26use prometheus::register_histogram_vec_with_registry;
27use prometheus::register_int_counter_vec_with_registry;
28use prometheus::register_int_gauge_vec_with_registry;
29
30use crate::layers::observe;
31use crate::raw::Access;
32use crate::raw::*;
33use crate::*;
34
35/// Add [prometheus](https://docs.rs/prometheus) for every operation.
36///
37/// # Prometheus Metrics
38///
39/// We provide several metrics, please see the documentation of [`observe`] module.
40/// For a more detailed explanation of these metrics and how they are used, please refer to the [Prometheus documentation](https://prometheus.io/docs/introduction/overview/).
41///
42/// # Examples
43///
44/// ## Basic Usage
45///
46/// ```no_run
47/// # use log::info;
48/// # use opendal::layers::PrometheusLayer;
49/// # use opendal::services;
50/// # use opendal::Operator;
51/// # use opendal::Result;
52/// # use prometheus::Encoder;
53///
54/// # #[tokio::main]
55/// # async fn main() -> Result<()> {
56/// let registry = prometheus::default_registry();
57///
58/// let op = Operator::new(services::Memory::default())?
59///     .layer(
60///         PrometheusLayer::builder()
61///             .register(registry)
62///             .expect("register metrics successfully"),
63///     )
64///     .finish();
65///
66/// // Write data into object test.
67/// op.write("test", "Hello, World!").await?;
68/// // Read data from the object.
69/// let bs = op.read("test").await?;
70/// info!("content: {}", String::from_utf8_lossy(&bs.to_bytes()));
71///
72/// // Get object metadata.
73/// let meta = op.stat("test").await?;
74/// info!("meta: {:?}", meta);
75///
76/// // Export prometheus metrics.
77/// let mut buffer = Vec::<u8>::new();
78/// let encoder = prometheus::TextEncoder::new();
79/// encoder.encode(&prometheus::gather(), &mut buffer).unwrap();
80/// println!("## Prometheus Metrics");
81/// println!("{}", String::from_utf8(buffer.clone()).unwrap());
82/// # Ok(())
83/// # }
84/// ```
85///
86/// ## Global Instance
87///
88/// `PrometheusLayer` needs to be registered before instantiation.
89///
90/// If there are multiple operators in an application that need the `PrometheusLayer`, we could
91/// instantiate it once and pass it to multiple operators. But we cannot directly call
92/// `.layer(PrometheusLayer::builder().register(&registry)?)` for different services, because
93/// registering the same metrics multiple times to the same registry will cause register errors.
94/// Therefore, we can provide a global instance for the `PrometheusLayer`.
95///
96/// ```no_run
97/// # use std::sync::OnceLock;
98/// # use log::info;
99/// # use opendal::layers::PrometheusLayer;
100/// # use opendal::services;
101/// # use opendal::Operator;
102/// # use opendal::Result;
103/// # use prometheus::Encoder;
104///
105/// fn global_prometheus_layer() -> &'static PrometheusLayer {
106///     static GLOBAL: OnceLock<PrometheusLayer> = OnceLock::new();
107///     GLOBAL.get_or_init(|| {
108///         PrometheusLayer::builder()
109///             .register_default()
110///             .expect("Failed to register with the global registry")
111///     })
112/// }
113///
114/// # #[tokio::main]
115/// # async fn main() -> Result<()> {
116/// let op = Operator::new(services::Memory::default())?
117///     .layer(global_prometheus_layer().clone())
118///     .finish();
119///
120/// // Write data into object test.
121/// op.write("test", "Hello, World!").await?;
122///
123/// // Read data from the object.
124/// let bs = op.read("test").await?;
125/// info!("content: {}", String::from_utf8_lossy(&bs.to_bytes()));
126///
127/// // Get object metadata.
128/// let meta = op.stat("test").await?;
129/// info!("meta: {:?}", meta);
130///
131/// // Export prometheus metrics.
132/// let mut buffer = Vec::<u8>::new();
133/// let encoder = prometheus::TextEncoder::new();
134/// encoder.encode(&prometheus::gather(), &mut buffer).unwrap();
135/// println!("## Prometheus Metrics");
136/// println!("{}", String::from_utf8(buffer.clone()).unwrap());
137/// # Ok(())
138/// # }
139/// ```
140#[derive(Clone, Debug)]
141pub struct PrometheusLayer {
142    interceptor: PrometheusInterceptor,
143}
144
145impl PrometheusLayer {
146    /// Create a [`PrometheusLayerBuilder`] to set the configuration of metrics.
147    ///
148    /// # Example
149    ///
150    /// ```no_run
151    /// # use opendal::layers::PrometheusLayer;
152    /// # use opendal::services;
153    /// # use opendal::Operator;
154    /// # use opendal::Result;
155    /// #
156    /// # #[tokio::main]
157    /// # async fn main() -> Result<()> {
158    /// // Pick a builder and configure it.
159    /// let builder = services::Memory::default();
160    /// let registry = prometheus::default_registry();
161    ///
162    /// let duration_seconds_buckets = prometheus::exponential_buckets(0.01, 2.0, 16).unwrap();
163    /// let bytes_buckets = prometheus::exponential_buckets(1.0, 2.0, 16).unwrap();
164    /// let _ = Operator::new(builder)?
165    ///     .layer(
166    ///         PrometheusLayer::builder()
167    ///             .duration_seconds_buckets(duration_seconds_buckets)
168    ///             .bytes_buckets(bytes_buckets)
169    ///             .register(registry)
170    ///             .expect("register metrics successfully"),
171    ///     )
172    ///     .finish();
173    /// # Ok(())
174    /// # }
175    /// ```
176    pub fn builder() -> PrometheusLayerBuilder {
177        PrometheusLayerBuilder::default()
178    }
179}
180
181impl<A: Access> Layer<A> for PrometheusLayer {
182    type LayeredAccess = observe::MetricsAccessor<A, PrometheusInterceptor>;
183
184    fn layer(&self, inner: A) -> Self::LayeredAccess {
185        observe::MetricsLayer::new(self.interceptor.clone()).layer(inner)
186    }
187}
188
189/// [`PrometheusLayerBuilder`] is a config builder to build a [`PrometheusLayer`].
190pub struct PrometheusLayerBuilder {
191    bytes_buckets: Vec<f64>,
192    bytes_rate_buckets: Vec<f64>,
193    entries_buckets: Vec<f64>,
194    entries_rate_buckets: Vec<f64>,
195    duration_seconds_buckets: Vec<f64>,
196    ttfb_buckets: Vec<f64>,
197}
198
199impl Default for PrometheusLayerBuilder {
200    fn default() -> Self {
201        Self {
202            bytes_buckets: observe::DEFAULT_BYTES_BUCKETS.to_vec(),
203            bytes_rate_buckets: observe::DEFAULT_BYTES_RATE_BUCKETS.to_vec(),
204            entries_buckets: observe::DEFAULT_ENTRIES_BUCKETS.to_vec(),
205            entries_rate_buckets: observe::DEFAULT_ENTRIES_RATE_BUCKETS.to_vec(),
206            duration_seconds_buckets: observe::DEFAULT_DURATION_SECONDS_BUCKETS.to_vec(),
207            ttfb_buckets: observe::DEFAULT_TTFB_BUCKETS.to_vec(),
208        }
209    }
210}
211
212impl PrometheusLayerBuilder {
213    /// Set buckets for bytes related histogram like `operation_bytes`.
214    pub fn bytes_buckets(mut self, buckets: Vec<f64>) -> Self {
215        if !buckets.is_empty() {
216            self.bytes_buckets = buckets;
217        }
218        self
219    }
220
221    /// Set buckets for bytes rate related histogram like `operation_bytes_rate`.
222    pub fn bytes_rate_buckets(mut self, buckets: Vec<f64>) -> Self {
223        if !buckets.is_empty() {
224            self.bytes_rate_buckets = buckets;
225        }
226        self
227    }
228
229    /// Set buckets for entries related histogram like `operation_entries`.
230    pub fn entries_buckets(mut self, buckets: Vec<f64>) -> Self {
231        if !buckets.is_empty() {
232            self.entries_buckets = buckets;
233        }
234        self
235    }
236
237    /// Set buckets for entries rate related histogram like `operation_entries_rate`.
238    pub fn entries_rate_buckets(mut self, buckets: Vec<f64>) -> Self {
239        if !buckets.is_empty() {
240            self.entries_rate_buckets = buckets;
241        }
242        self
243    }
244
245    /// Set buckets for duration seconds related histogram like `operation_duration_seconds`.
246    pub fn duration_seconds_buckets(mut self, buckets: Vec<f64>) -> Self {
247        if !buckets.is_empty() {
248            self.duration_seconds_buckets = buckets;
249        }
250        self
251    }
252
253    /// Set buckets for ttfb related histogram like `operation_ttfb_seconds`.
254    pub fn ttfb_buckets(mut self, buckets: Vec<f64>) -> Self {
255        if !buckets.is_empty() {
256            self.ttfb_buckets = buckets;
257        }
258        self
259    }
260
261    /// Register the metrics into the given registry and return a [`PrometheusLayer`].
262    ///
263    /// # Example
264    ///
265    /// ```no_run
266    /// # use opendal::layers::PrometheusLayer;
267    /// # use opendal::services;
268    /// # use opendal::Operator;
269    /// # use opendal::Result;
270    /// #
271    /// # #[tokio::main]
272    /// # async fn main() -> Result<()> {
273    /// // Pick a builder and configure it.
274    /// let builder = services::Memory::default();
275    /// let _ = Operator::new(builder)?
276    ///     .layer(
277    ///         PrometheusLayer::builder()
278    ///             .register(prometheus::default_registry())
279    ///             .expect("register metrics successfully"),
280    ///     )
281    ///     .finish();
282    /// # Ok(())
283    /// # }
284    /// ```
285    pub fn register(self, registry: &Registry) -> Result<PrometheusLayer> {
286        let labels = OperationLabels::names();
287        let operation_bytes = {
288            let metric = observe::MetricValue::OperationBytes(0);
289            register_histogram_vec_with_registry!(
290                metric.name(),
291                metric.help(),
292                labels.as_ref(),
293                self.bytes_buckets.clone(),
294                registry
295            )
296            .map_err(parse_prometheus_error)?
297        };
298        let operation_bytes_rate = {
299            let metric = observe::MetricValue::OperationBytesRate(0.0);
300            register_histogram_vec_with_registry!(
301                metric.name(),
302                metric.help(),
303                labels.as_ref(),
304                self.bytes_rate_buckets.clone(),
305                registry
306            )
307            .map_err(parse_prometheus_error)?
308        };
309        let operation_entries = {
310            let metric = observe::MetricValue::OperationEntries(0);
311            register_histogram_vec_with_registry!(
312                metric.name(),
313                metric.help(),
314                labels.as_ref(),
315                self.entries_buckets,
316                registry
317            )
318            .map_err(parse_prometheus_error)?
319        };
320        let operation_entries_rate = {
321            let metric = observe::MetricValue::OperationEntriesRate(0.0);
322            register_histogram_vec_with_registry!(
323                metric.name(),
324                metric.help(),
325                labels.as_ref(),
326                self.entries_rate_buckets,
327                registry
328            )
329            .map_err(parse_prometheus_error)?
330        };
331        let operation_duration_seconds = {
332            let metric = observe::MetricValue::OperationDurationSeconds(Duration::default());
333            register_histogram_vec_with_registry!(
334                metric.name(),
335                metric.help(),
336                labels.as_ref(),
337                self.duration_seconds_buckets.clone(),
338                registry
339            )
340            .map_err(parse_prometheus_error)?
341        };
342        let operation_executing = {
343            let metric = observe::MetricValue::OperationExecuting(0);
344            register_int_gauge_vec_with_registry!(
345                metric.name(),
346                metric.help(),
347                labels.as_ref(),
348                registry
349            )
350            .map_err(parse_prometheus_error)?
351        };
352        let operation_ttfb_seconds = {
353            let metric = observe::MetricValue::OperationTtfbSeconds(Duration::default());
354            register_histogram_vec_with_registry!(
355                metric.name(),
356                metric.help(),
357                labels.as_ref(),
358                self.ttfb_buckets.clone(),
359                registry
360            )
361            .map_err(parse_prometheus_error)?
362        };
363
364        let labels_with_error = OperationLabels::names().with_error();
365        let operation_errors_total = {
366            let metric = observe::MetricValue::OperationErrorsTotal;
367            register_int_counter_vec_with_registry!(
368                metric.name(),
369                metric.help(),
370                labels_with_error.as_ref(),
371                registry
372            )
373            .map_err(parse_prometheus_error)?
374        };
375
376        let http_executing = {
377            let metric = observe::MetricValue::HttpExecuting(0);
378            register_int_gauge_vec_with_registry!(
379                metric.name(),
380                metric.help(),
381                labels.as_ref(),
382                registry
383            )
384            .map_err(parse_prometheus_error)?
385        };
386        let http_request_bytes = {
387            let metric = observe::MetricValue::HttpRequestBytes(0);
388            register_histogram_vec_with_registry!(
389                metric.name(),
390                metric.help(),
391                labels.as_ref(),
392                self.bytes_buckets.clone(),
393                registry
394            )
395            .map_err(parse_prometheus_error)?
396        };
397        let http_request_bytes_rate = {
398            let metric = observe::MetricValue::HttpRequestBytesRate(0.0);
399            register_histogram_vec_with_registry!(
400                metric.name(),
401                metric.help(),
402                labels.as_ref(),
403                self.bytes_rate_buckets.clone(),
404                registry
405            )
406            .map_err(parse_prometheus_error)?
407        };
408        let http_request_duration_seconds = {
409            let metric = observe::MetricValue::HttpRequestDurationSeconds(Duration::default());
410            register_histogram_vec_with_registry!(
411                metric.name(),
412                metric.help(),
413                labels.as_ref(),
414                self.duration_seconds_buckets.clone(),
415                registry
416            )
417            .map_err(parse_prometheus_error)?
418        };
419        let http_response_bytes = {
420            let metric = observe::MetricValue::HttpResponseBytes(0);
421            register_histogram_vec_with_registry!(
422                metric.name(),
423                metric.help(),
424                labels.as_ref(),
425                self.bytes_buckets,
426                registry
427            )
428            .map_err(parse_prometheus_error)?
429        };
430        let http_response_bytes_rate = {
431            let metric = observe::MetricValue::HttpResponseBytesRate(0.0);
432            register_histogram_vec_with_registry!(
433                metric.name(),
434                metric.help(),
435                labels.as_ref(),
436                self.bytes_rate_buckets,
437                registry
438            )
439            .map_err(parse_prometheus_error)?
440        };
441        let http_response_duration_seconds = {
442            let metric = observe::MetricValue::HttpResponseDurationSeconds(Duration::default());
443            register_histogram_vec_with_registry!(
444                metric.name(),
445                metric.help(),
446                labels.as_ref(),
447                self.duration_seconds_buckets,
448                registry
449            )
450            .map_err(parse_prometheus_error)?
451        };
452        let http_connection_errors_total = {
453            let metric = observe::MetricValue::HttpConnectionErrorsTotal;
454            register_int_counter_vec_with_registry!(
455                metric.name(),
456                metric.help(),
457                labels.as_ref(),
458                registry
459            )
460            .map_err(parse_prometheus_error)?
461        };
462
463        let labels_with_status_code = OperationLabels::names().with_status_code();
464        let http_status_errors_total = {
465            let metric = observe::MetricValue::HttpStatusErrorsTotal;
466            register_int_counter_vec_with_registry!(
467                metric.name(),
468                metric.help(),
469                labels_with_status_code.as_ref(),
470                registry
471            )
472            .map_err(parse_prometheus_error)?
473        };
474
475        Ok(PrometheusLayer {
476            interceptor: PrometheusInterceptor {
477                operation_bytes,
478                operation_bytes_rate,
479                operation_entries,
480                operation_entries_rate,
481                operation_duration_seconds,
482                operation_errors_total,
483                operation_executing,
484                operation_ttfb_seconds,
485
486                http_executing,
487                http_request_bytes,
488                http_request_bytes_rate,
489                http_request_duration_seconds,
490                http_response_bytes,
491                http_response_bytes_rate,
492                http_response_duration_seconds,
493                http_connection_errors_total,
494                http_status_errors_total,
495            },
496        })
497    }
498
499    /// Register the metrics into the default registry and return a [`PrometheusLayer`].
500    ///
501    /// # Example
502    ///
503    /// ```no_run
504    /// # use opendal::layers::PrometheusLayer;
505    /// # use opendal::services;
506    /// # use opendal::Operator;
507    /// # use opendal::Result;
508    /// #
509    /// # #[tokio::main]
510    /// # async fn main() -> Result<()> {
511    /// // Pick a builder and configure it.
512    /// let builder = services::Memory::default();
513    /// let _ = Operator::new(builder)?
514    ///     .layer(
515    ///         PrometheusLayer::builder()
516    ///             .register_default()
517    ///             .expect("register metrics successfully"),
518    ///     )
519    ///     .finish();
520    /// # Ok(())
521    /// # }
522    /// ```
523    pub fn register_default(self) -> Result<PrometheusLayer> {
524        let registry = prometheus::default_registry();
525        self.register(registry)
526    }
527}
528
529/// Convert the [`prometheus::Error`] to [`Error`].
530fn parse_prometheus_error(err: prometheus::Error) -> Error {
531    Error::new(ErrorKind::Unexpected, err.to_string()).set_source(err)
532}
533
534#[derive(Clone, Debug)]
535pub struct PrometheusInterceptor {
536    operation_bytes: HistogramVec,
537    operation_bytes_rate: HistogramVec,
538    operation_entries: HistogramVec,
539    operation_entries_rate: HistogramVec,
540    operation_duration_seconds: HistogramVec,
541    operation_errors_total: GenericCounterVec<AtomicU64>,
542    operation_executing: GenericGaugeVec<AtomicI64>,
543    operation_ttfb_seconds: HistogramVec,
544
545    http_executing: GenericGaugeVec<AtomicI64>,
546    http_request_bytes: HistogramVec,
547    http_request_bytes_rate: HistogramVec,
548    http_request_duration_seconds: HistogramVec,
549    http_response_bytes: HistogramVec,
550    http_response_bytes_rate: HistogramVec,
551    http_response_duration_seconds: HistogramVec,
552    http_connection_errors_total: GenericCounterVec<AtomicU64>,
553    http_status_errors_total: GenericCounterVec<AtomicU64>,
554}
555
556impl observe::MetricsIntercept for PrometheusInterceptor {
557    fn observe(&self, labels: observe::MetricLabels, value: observe::MetricValue) {
558        let labels = OperationLabels(labels);
559        match value {
560            observe::MetricValue::OperationBytes(v) => self
561                .operation_bytes
562                .with_label_values(&labels.values())
563                .observe(v as f64),
564            observe::MetricValue::OperationBytesRate(v) => self
565                .operation_bytes_rate
566                .with_label_values(&labels.values())
567                .observe(v),
568            observe::MetricValue::OperationEntries(v) => self
569                .operation_entries
570                .with_label_values(&labels.values())
571                .observe(v as f64),
572            observe::MetricValue::OperationEntriesRate(v) => self
573                .operation_entries_rate
574                .with_label_values(&labels.values())
575                .observe(v),
576            observe::MetricValue::OperationDurationSeconds(v) => self
577                .operation_duration_seconds
578                .with_label_values(&labels.values())
579                .observe(v.as_secs_f64()),
580            observe::MetricValue::OperationErrorsTotal => self
581                .operation_errors_total
582                .with_label_values(&labels.values())
583                .inc(),
584            observe::MetricValue::OperationExecuting(v) => self
585                .operation_executing
586                .with_label_values(&labels.values())
587                .add(v as i64),
588            observe::MetricValue::OperationTtfbSeconds(v) => self
589                .operation_ttfb_seconds
590                .with_label_values(&labels.values())
591                .observe(v.as_secs_f64()),
592
593            observe::MetricValue::HttpExecuting(v) => self
594                .http_executing
595                .with_label_values(&labels.values())
596                .add(v as i64),
597            observe::MetricValue::HttpRequestBytes(v) => self
598                .http_request_bytes
599                .with_label_values(&labels.values())
600                .observe(v as f64),
601            observe::MetricValue::HttpRequestBytesRate(v) => self
602                .http_request_bytes_rate
603                .with_label_values(&labels.values())
604                .observe(v),
605            observe::MetricValue::HttpRequestDurationSeconds(v) => self
606                .http_request_duration_seconds
607                .with_label_values(&labels.values())
608                .observe(v.as_secs_f64()),
609            observe::MetricValue::HttpResponseBytes(v) => self
610                .http_response_bytes
611                .with_label_values(&labels.values())
612                .observe(v as f64),
613            observe::MetricValue::HttpResponseBytesRate(v) => self
614                .http_response_bytes_rate
615                .with_label_values(&labels.values())
616                .observe(v),
617            observe::MetricValue::HttpResponseDurationSeconds(v) => self
618                .http_response_duration_seconds
619                .with_label_values(&labels.values())
620                .observe(v.as_secs_f64()),
621            observe::MetricValue::HttpConnectionErrorsTotal => self
622                .http_connection_errors_total
623                .with_label_values(&labels.values())
624                .inc(),
625            observe::MetricValue::HttpStatusErrorsTotal => self
626                .http_status_errors_total
627                .with_label_values(&labels.values())
628                .inc(),
629        }
630    }
631}
632
633struct OperationLabelNames(Vec<&'static str>);
634
635impl AsRef<[&'static str]> for OperationLabelNames {
636    fn as_ref(&self) -> &[&'static str] {
637        &self.0
638    }
639}
640
641impl OperationLabelNames {
642    fn with_error(mut self) -> Self {
643        self.0.push(observe::LABEL_ERROR);
644        self
645    }
646
647    fn with_status_code(mut self) -> Self {
648        self.0.push(observe::LABEL_STATUS_CODE);
649        self
650    }
651}
652
653#[derive(Clone, Debug, PartialEq, Eq, Hash)]
654struct OperationLabels(observe::MetricLabels);
655
656impl OperationLabels {
657    fn names() -> OperationLabelNames {
658        OperationLabelNames(vec![
659            observe::LABEL_SCHEME,
660            observe::LABEL_NAMESPACE,
661            observe::LABEL_ROOT,
662            observe::LABEL_OPERATION,
663        ])
664    }
665
666    fn values(&self) -> Vec<&str> {
667        let mut labels = Vec::with_capacity(6);
668
669        labels.extend([
670            self.0.scheme,
671            self.0.namespace.as_ref(),
672            self.0.root.as_ref(),
673            self.0.operation,
674        ]);
675
676        if let Some(error) = self.0.error {
677            labels.push(error.into_static());
678        }
679
680        if let Some(status_code) = &self.0.status_code {
681            labels.push(status_code.as_str());
682        }
683
684        labels
685    }
686}
```
