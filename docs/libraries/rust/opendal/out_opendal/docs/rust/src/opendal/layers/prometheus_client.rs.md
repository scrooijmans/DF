# 

opendal/layers/

prometheus_client.rs

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
18use std::fmt;
19use std::time::Duration;
20
21use prometheus_client::encoding::EncodeLabel;
22use prometheus_client::encoding::EncodeLabelSet;
23use prometheus_client::encoding::LabelSetEncoder;
24use prometheus_client::metrics::counter::Counter;
25use prometheus_client::metrics::family::Family;
26use prometheus_client::metrics::family::MetricConstructor;
27use prometheus_client::metrics::gauge::Gauge;
28use prometheus_client::metrics::histogram::Histogram;
29use prometheus_client::registry::Metric;
30use prometheus_client::registry::Registry;
31use prometheus_client::registry::Unit;
32
33use crate::layers::observe;
34use crate::raw::*;
35use crate::*;
36
37/// Add [prometheus-client](https://docs.rs/prometheus-client) for every operation.
38///
39/// # Prometheus Metrics
40///
41/// We provide several metrics, please see the documentation of [`observe`] module.
42/// For a more detailed explanation of these metrics and how they are used, please refer to the [Prometheus documentation](https://prometheus.io/docs/introduction/overview/).
43///
44/// # Examples
45///
46/// ```no_run
47/// # use log::info;
48/// # use opendal::layers::PrometheusClientLayer;
49/// # use opendal::services;
50/// # use opendal::Operator;
51/// # use opendal::Result;
52///
53/// # #[tokio::main]
54/// # async fn main() -> Result<()> {
55/// let mut registry = prometheus_client::registry::Registry::default();
56///
57/// let op = Operator::new(services::Memory::default())?
58///     .layer(PrometheusClientLayer::builder().register(&mut registry))
59///     .finish();
60///
61/// // Write data into object test.
62/// op.write("test", "Hello, World!").await?;
63/// // Read data from the object.
64/// let bs = op.read("test").await?;
65/// info!("content: {}", String::from_utf8_lossy(&bs.to_bytes()));
66///
67/// // Get object metadata.
68/// let meta = op.stat("test").await?;
69/// info!("meta: {:?}", meta);
70///
71/// // Export prometheus metrics.
72/// let mut buf = String::new();
73/// prometheus_client::encoding::text::encode(&mut buf, &registry).unwrap();
74/// println!("## Prometheus Metrics");
75/// println!("{}", buf);
76/// # Ok(())
77/// # }
78/// ```
79#[derive(Clone, Debug)]
80pub struct PrometheusClientLayer {
81    interceptor: PrometheusClientInterceptor,
82}
83
84impl PrometheusClientLayer {
85    /// Create a [`PrometheusClientLayerBuilder`] to set the configuration of metrics.
86    pub fn builder() -> PrometheusClientLayerBuilder {
87        PrometheusClientLayerBuilder::default()
88    }
89}
90
91impl<A: Access> Layer<A> for PrometheusClientLayer {
92    type LayeredAccess = observe::MetricsAccessor<A, PrometheusClientInterceptor>;
93
94    fn layer(&self, inner: A) -> Self::LayeredAccess {
95        observe::MetricsLayer::new(self.interceptor.clone()).layer(inner)
96    }
97}
98
99/// [`PrometheusClientLayerBuilder`] is a config builder to build a [`PrometheusClientLayer`].
100pub struct PrometheusClientLayerBuilder {
101    bytes_buckets: Vec<f64>,
102    bytes_rate_buckets: Vec<f64>,
103    entries_buckets: Vec<f64>,
104    entries_rate_buckets: Vec<f64>,
105    duration_seconds_buckets: Vec<f64>,
106    ttfb_buckets: Vec<f64>,
107    disable_label_root: bool,
108}
109
110impl Default for PrometheusClientLayerBuilder {
111    fn default() -> Self {
112        Self {
113            bytes_buckets: observe::DEFAULT_BYTES_BUCKETS.to_vec(),
114            bytes_rate_buckets: observe::DEFAULT_BYTES_RATE_BUCKETS.to_vec(),
115            entries_buckets: observe::DEFAULT_ENTRIES_BUCKETS.to_vec(),
116            entries_rate_buckets: observe::DEFAULT_ENTRIES_RATE_BUCKETS.to_vec(),
117            duration_seconds_buckets: observe::DEFAULT_DURATION_SECONDS_BUCKETS.to_vec(),
118            ttfb_buckets: observe::DEFAULT_TTFB_BUCKETS.to_vec(),
119            disable_label_root: false,
120        }
121    }
122}
123
124impl PrometheusClientLayerBuilder {
125    /// Set buckets for bytes related histogram like `operation_bytes`.
126    pub fn bytes_buckets(mut self, buckets: Vec<f64>) -> Self {
127        if !buckets.is_empty() {
128            self.bytes_buckets = buckets;
129        }
130        self
131    }
132
133    /// Set buckets for bytes rate related histogram like `operation_bytes_rate`.
134    pub fn bytes_rate_buckets(mut self, buckets: Vec<f64>) -> Self {
135        if !buckets.is_empty() {
136            self.bytes_rate_buckets = buckets;
137        }
138        self
139    }
140
141    /// Set buckets for entries related histogram like `operation_entries`.
142    pub fn entries_buckets(mut self, buckets: Vec<f64>) -> Self {
143        if !buckets.is_empty() {
144            self.entries_buckets = buckets;
145        }
146        self
147    }
148
149    /// Set buckets for entries rate related histogram like `operation_entries_rate`.
150    pub fn entries_rate_buckets(mut self, buckets: Vec<f64>) -> Self {
151        if !buckets.is_empty() {
152            self.entries_rate_buckets = buckets;
153        }
154        self
155    }
156
157    /// Set buckets for duration seconds related histogram like `operation_duration_seconds`.
158    pub fn duration_seconds_buckets(mut self, buckets: Vec<f64>) -> Self {
159        if !buckets.is_empty() {
160            self.duration_seconds_buckets = buckets;
161        }
162        self
163    }
164
165    /// Set buckets for ttfb related histogram like `operation_ttfb_seconds`.
166    pub fn ttfb_buckets(mut self, buckets: Vec<f64>) -> Self {
167        if !buckets.is_empty() {
168            self.ttfb_buckets = buckets;
169        }
170        self
171    }
172
173    /// The 'root' label might have risks of being high cardinality, users can choose to disable it
174    /// when they found it's not useful for their metrics.
175    pub fn disable_label_root(mut self, disable: bool) -> Self {
176        self.disable_label_root = disable;
177        self
178    }
179
180    /// Register the metrics into the registry and return a [`PrometheusClientLayer`].
181    ///
182    /// # Example
183    ///
184    /// ```no_run
185    /// # use opendal::layers::PrometheusClientLayer;
186    /// # use opendal::services;
187    /// # use opendal::Operator;
188    /// # use opendal::Result;
189    ///
190    /// # #[tokio::main]
191    /// # async fn main() -> Result<()> {
192    /// // Pick a builder and configure it.
193    /// let builder = services::Memory::default();
194    /// let mut registry = prometheus_client::registry::Registry::default();
195    ///
196    /// let _ = Operator::new(builder)?
197    ///     .layer(PrometheusClientLayer::builder().register(&mut registry))
198    ///     .finish();
199    /// # Ok(())
200    /// # }
201    /// ```
202    pub fn register(self, registry: &mut Registry) -> PrometheusClientLayer {
203        let operation_bytes =
204            Family::<OperationLabels, Histogram, _>::new_with_constructor(HistogramConstructor {
205                buckets: self.bytes_buckets.clone(),
206            });
207        let operation_bytes_rate =
208            Family::<OperationLabels, Histogram, _>::new_with_constructor(HistogramConstructor {
209                buckets: self.bytes_rate_buckets.clone(),
210            });
211        let operation_entries =
212            Family::<OperationLabels, Histogram, _>::new_with_constructor(HistogramConstructor {
213                buckets: self.entries_buckets.clone(),
214            });
215        let operation_entries_rate =
216            Family::<OperationLabels, Histogram, _>::new_with_constructor(HistogramConstructor {
217                buckets: self.entries_rate_buckets.clone(),
218            });
219        let operation_duration_seconds =
220            Family::<OperationLabels, Histogram, _>::new_with_constructor(HistogramConstructor {
221                buckets: self.duration_seconds_buckets.clone(),
222            });
223        let operation_errors_total = Family::<OperationLabels, Counter>::default();
224        let operation_executing = Family::<OperationLabels, Gauge>::default();
225        let operation_ttfb_seconds =
226            Family::<OperationLabels, Histogram, _>::new_with_constructor(HistogramConstructor {
227                buckets: self.ttfb_buckets.clone(),
228            });
229
230        let http_executing = Family::<OperationLabels, Gauge>::default();
231        let http_request_bytes =
232            Family::<OperationLabels, Histogram, _>::new_with_constructor(HistogramConstructor {
233                buckets: self.bytes_buckets.clone(),
234            });
235        let http_request_bytes_rate =
236            Family::<OperationLabels, Histogram, _>::new_with_constructor(HistogramConstructor {
237                buckets: self.bytes_rate_buckets.clone(),
238            });
239        let http_request_duration_seconds =
240            Family::<OperationLabels, Histogram, _>::new_with_constructor(HistogramConstructor {
241                buckets: self.duration_seconds_buckets.clone(),
242            });
243        let http_response_bytes =
244            Family::<OperationLabels, Histogram, _>::new_with_constructor(HistogramConstructor {
245                buckets: self.bytes_buckets.clone(),
246            });
247        let http_response_bytes_rate =
248            Family::<OperationLabels, Histogram, _>::new_with_constructor(HistogramConstructor {
249                buckets: self.bytes_rate_buckets.clone(),
250            });
251        let http_response_duration_seconds =
252            Family::<OperationLabels, Histogram, _>::new_with_constructor(HistogramConstructor {
253                buckets: self.duration_seconds_buckets.clone(),
254            });
255        let http_connection_errors_total = Family::<OperationLabels, Counter>::default();
256        let http_status_errors_total = Family::<OperationLabels, Counter>::default();
257
258        register_metric(
259            registry,
260            operation_bytes.clone(),
261            observe::MetricValue::OperationBytes(0),
262        );
263        register_metric(
264            registry,
265            operation_bytes_rate.clone(),
266            observe::MetricValue::OperationBytesRate(0.0),
267        );
268        register_metric(
269            registry,
270            operation_entries.clone(),
271            observe::MetricValue::OperationEntries(0),
272        );
273        register_metric(
274            registry,
275            operation_entries_rate.clone(),
276            observe::MetricValue::OperationEntriesRate(0.0),
277        );
278        register_metric(
279            registry,
280            operation_duration_seconds.clone(),
281            observe::MetricValue::OperationDurationSeconds(Duration::default()),
282        );
283        register_metric(
284            registry,
285            operation_errors_total.clone(),
286            observe::MetricValue::OperationErrorsTotal,
287        );
288        register_metric(
289            registry,
290            operation_executing.clone(),
291            observe::MetricValue::OperationExecuting(0),
292        );
293        register_metric(
294            registry,
295            operation_ttfb_seconds.clone(),
296            observe::MetricValue::OperationTtfbSeconds(Duration::default()),
297        );
298
299        register_metric(
300            registry,
301            http_executing.clone(),
302            observe::MetricValue::HttpExecuting(0),
303        );
304        register_metric(
305            registry,
306            http_request_bytes.clone(),
307            observe::MetricValue::HttpRequestBytes(0),
308        );
309        register_metric(
310            registry,
311            http_request_bytes_rate.clone(),
312            observe::MetricValue::HttpRequestBytesRate(0.0),
313        );
314        register_metric(
315            registry,
316            http_request_duration_seconds.clone(),
317            observe::MetricValue::HttpRequestDurationSeconds(Duration::default()),
318        );
319        register_metric(
320            registry,
321            http_response_bytes.clone(),
322            observe::MetricValue::HttpResponseBytes(0),
323        );
324        register_metric(
325            registry,
326            http_response_bytes_rate.clone(),
327            observe::MetricValue::HttpResponseBytesRate(0.0),
328        );
329        register_metric(
330            registry,
331            http_response_duration_seconds.clone(),
332            observe::MetricValue::HttpResponseDurationSeconds(Duration::default()),
333        );
334        register_metric(
335            registry,
336            http_connection_errors_total.clone(),
337            observe::MetricValue::HttpConnectionErrorsTotal,
338        );
339        register_metric(
340            registry,
341            http_status_errors_total.clone(),
342            observe::MetricValue::HttpStatusErrorsTotal,
343        );
344
345        PrometheusClientLayer {
346            interceptor: PrometheusClientInterceptor {
347                operation_bytes,
348                operation_bytes_rate,
349                operation_entries,
350                operation_entries_rate,
351                operation_duration_seconds,
352                operation_errors_total,
353                operation_executing,
354                operation_ttfb_seconds,
355
356                http_executing,
357                http_request_bytes,
358                http_request_bytes_rate,
359                http_request_duration_seconds,
360                http_response_bytes,
361                http_response_bytes_rate,
362                http_response_duration_seconds,
363                http_connection_errors_total,
364                http_status_errors_total,
365
366                disable_label_root: self.disable_label_root,
367            },
368        }
369    }
370}
371
372#[derive(Clone)]
373struct HistogramConstructor {
374    buckets: Vec<f64>,
375}
376
377impl MetricConstructor<Histogram> for HistogramConstructor {
378    fn new_metric(&self) -> Histogram {
379        Histogram::new(self.buckets.iter().cloned())
380    }
381}
382
383#[derive(Clone, Debug)]
384pub struct PrometheusClientInterceptor {
385    operation_bytes: Family<OperationLabels, Histogram, HistogramConstructor>,
386    operation_bytes_rate: Family<OperationLabels, Histogram, HistogramConstructor>,
387    operation_entries: Family<OperationLabels, Histogram, HistogramConstructor>,
388    operation_entries_rate: Family<OperationLabels, Histogram, HistogramConstructor>,
389    operation_duration_seconds: Family<OperationLabels, Histogram, HistogramConstructor>,
390    operation_errors_total: Family<OperationLabels, Counter>,
391    operation_executing: Family<OperationLabels, Gauge>,
392    operation_ttfb_seconds: Family<OperationLabels, Histogram, HistogramConstructor>,
393
394    http_executing: Family<OperationLabels, Gauge>,
395    http_request_bytes: Family<OperationLabels, Histogram, HistogramConstructor>,
396    http_request_bytes_rate: Family<OperationLabels, Histogram, HistogramConstructor>,
397    http_request_duration_seconds: Family<OperationLabels, Histogram, HistogramConstructor>,
398    http_response_bytes: Family<OperationLabels, Histogram, HistogramConstructor>,
399    http_response_bytes_rate: Family<OperationLabels, Histogram, HistogramConstructor>,
400    http_response_duration_seconds: Family<OperationLabels, Histogram, HistogramConstructor>,
401    http_connection_errors_total: Family<OperationLabels, Counter>,
402    http_status_errors_total: Family<OperationLabels, Counter>,
403
404    disable_label_root: bool,
405}
406
407impl observe::MetricsIntercept for PrometheusClientInterceptor {
408    fn observe(&self, labels: observe::MetricLabels, value: observe::MetricValue) {
409        let labels = OperationLabels {
410            labels,
411            disable_label_root: self.disable_label_root,
412        };
413        match value {
414            observe::MetricValue::OperationBytes(v) => self
415                .operation_bytes
416                .get_or_create(&labels)
417                .observe(v as f64),
418            observe::MetricValue::OperationBytesRate(v) => {
419                self.operation_bytes_rate.get_or_create(&labels).observe(v)
420            }
421            observe::MetricValue::OperationEntries(v) => self
422                .operation_entries
423                .get_or_create(&labels)
424                .observe(v as f64),
425            observe::MetricValue::OperationEntriesRate(v) => self
426                .operation_entries_rate
427                .get_or_create(&labels)
428                .observe(v),
429            observe::MetricValue::OperationDurationSeconds(v) => self
430                .operation_duration_seconds
431                .get_or_create(&labels)
432                .observe(v.as_secs_f64()),
433            observe::MetricValue::OperationErrorsTotal => {
434                self.operation_errors_total.get_or_create(&labels).inc();
435            }
436            observe::MetricValue::OperationExecuting(v) => {
437                self.operation_executing
438                    .get_or_create(&labels)
439                    .inc_by(v as i64);
440            }
441            observe::MetricValue::OperationTtfbSeconds(v) => self
442                .operation_ttfb_seconds
443                .get_or_create(&labels)
444                .observe(v.as_secs_f64()),
445
446            observe::MetricValue::HttpExecuting(v) => {
447                self.http_executing.get_or_create(&labels).inc_by(v as i64);
448            }
449            observe::MetricValue::HttpRequestBytes(v) => self
450                .http_request_bytes
451                .get_or_create(&labels)
452                .observe(v as f64),
453            observe::MetricValue::HttpRequestBytesRate(v) => self
454                .http_request_bytes_rate
455                .get_or_create(&labels)
456                .observe(v),
457            observe::MetricValue::HttpRequestDurationSeconds(v) => self
458                .http_request_duration_seconds
459                .get_or_create(&labels)
460                .observe(v.as_secs_f64()),
461            observe::MetricValue::HttpResponseBytes(v) => self
462                .http_response_bytes
463                .get_or_create(&labels)
464                .observe(v as f64),
465            observe::MetricValue::HttpResponseBytesRate(v) => self
466                .http_response_bytes_rate
467                .get_or_create(&labels)
468                .observe(v),
469            observe::MetricValue::HttpResponseDurationSeconds(v) => self
470                .http_response_duration_seconds
471                .get_or_create(&labels)
472                .observe(v.as_secs_f64()),
473            observe::MetricValue::HttpConnectionErrorsTotal => {
474                self.http_connection_errors_total
475                    .get_or_create(&labels)
476                    .inc();
477            }
478            observe::MetricValue::HttpStatusErrorsTotal => {
479                self.http_status_errors_total.get_or_create(&labels).inc();
480            }
481        };
482    }
483}
484
485#[derive(Clone, Debug, PartialEq, Eq, Hash)]
486struct OperationLabels {
487    labels: observe::MetricLabels,
488    disable_label_root: bool,
489}
490
491impl EncodeLabelSet for OperationLabels {
492    fn encode(&self, encoder: &mut LabelSetEncoder<'_>) -> Result<(), fmt::Error> {
493        (observe::LABEL_SCHEME, self.labels.scheme).encode(encoder.encode_label())?;
494        (observe::LABEL_NAMESPACE, self.labels.namespace.as_ref())
495            .encode(encoder.encode_label())?;
496        if !self.disable_label_root {
497            (observe::LABEL_ROOT, self.labels.root.as_ref()).encode(encoder.encode_label())?;
498        }
499        (observe::LABEL_OPERATION, self.labels.operation).encode(encoder.encode_label())?;
500
501        if let Some(error) = &self.labels.error {
502            (observe::LABEL_ERROR, error.into_static()).encode(encoder.encode_label())?;
503        }
504        if let Some(code) = &self.labels.status_code {
505            (observe::LABEL_STATUS_CODE, code.as_str()).encode(encoder.encode_label())?;
506        }
507        Ok(())
508    }
509}
510
511fn register_metric(registry: &mut Registry, metric: impl Metric, value: observe::MetricValue) {
512    let ((name, unit), help) = (value.name_with_unit(), value.help());
513
514    if let Some(unit) = unit {
515        registry.register_with_unit(name, help, Unit::Other(unit.to_string()), metric);
516    } else {
517        registry.register(name, help, metric);
518    }
519}
```
