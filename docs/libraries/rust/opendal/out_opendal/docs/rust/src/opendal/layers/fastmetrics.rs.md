# 

opendal/layers/

fastmetrics.rs

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
21use fastmetrics::encoder::EncodeLabelSet;
22use fastmetrics::encoder::LabelSetEncoder;
23use fastmetrics::metrics::counter::Counter;
24use fastmetrics::metrics::family::Family;
25use fastmetrics::metrics::family::MetricFactory;
26use fastmetrics::metrics::gauge::Gauge;
27use fastmetrics::metrics::histogram::Histogram;
28use fastmetrics::registry::Register;
29use fastmetrics::registry::Registry;
30use fastmetrics::registry::RegistryError;
31use fastmetrics::registry::with_global_registry_mut;
32
33use crate::layers::observe;
34use crate::raw::*;
35use crate::*;
36
37/// Add [fastmetrics](https://docs.rs/fastmetrics/) for every operation.
38///
39/// # Examples
40///
41/// ## Basic Usage
42///
43/// ```no_run
44/// # use fastmetrics::format::text;
45/// # use log::info;
46/// # use opendal::layers::FastmetricsLayer;
47/// # use opendal::services;
48/// # use opendal::Operator;
49/// # use opendal::Result;
50///
51/// # #[tokio::main]
52/// # async fn main() -> Result<()> {
53/// let mut registry = fastmetrics::registry::Registry::default();
54/// let op = Operator::new(services::Memory::default())?
55///     .layer(FastmetricsLayer::builder().register(&mut registry)?)
56///     .finish();
57///
58/// // Write data into object test.
59/// op.write("test", "Hello, World!").await?;
60///
61/// // Read data from the object.
62/// let bs = op.read("test").await?;
63/// info!("content: {}", String::from_utf8_lossy(&bs.to_bytes()));
64///
65/// // Get object metadata.
66/// let meta = op.stat("test").await?;
67/// info!("meta: {:?}", meta);
68///
69/// // Export prometheus metrics.
70/// let mut output = String::new();
71/// text::encode(&mut output, &registry).unwrap();
72/// println!("{}", output);
73/// # Ok(())
74/// # }
75/// ```
76/// ## Global Instance
77///
78/// `FastmetricsLayer` needs to be registered before instantiation.
79///
80/// If there are multiple operators in an application that need the `FastmetricsLayer`, we could
81/// instantiate it once and pass it to multiple operators. But we cannot directly call
82/// `.layer(FastmetricsLayer::builder().register(&mut registry)?)` for different services, because
83/// registering the same metrics multiple times to the same registry will cause register errors.
84/// Therefore, we can provide a global instance for the `FastmetricsLayer`.
85///
86/// ```no_run
87/// # use std::sync::OnceLock;
88/// # use fastmetrics::format::text;
89/// # use fastmetrics::registry::with_global_registry;
90/// # use log::info;
91/// # use opendal::layers::FastmetricsLayer;
92/// # use opendal::services;
93/// # use opendal::Operator;
94/// # use opendal::Result;
95///
96/// fn global_fastmetrics_layer() -> &'static FastmetricsLayer {
97///     static GLOBAL: OnceLock<FastmetricsLayer> = OnceLock::new();
98///     GLOBAL.get_or_init(|| {
99///         FastmetricsLayer::builder()
100///             .register_global()
101///             .expect("Failed to register with the global registry")
102///     })
103/// }
104///
105/// # #[tokio::main]
106/// # async fn main() -> Result<()> {
107/// let op = Operator::new(services::Memory::default())?
108///     .layer(global_fastmetrics_layer().clone())
109///     .finish();
110///
111/// // Write data into object test.
112/// op.write("test", "Hello, World!").await?;
113///
114/// // Read data from the object.
115/// let bs = op.read("test").await?;
116/// info!("content: {}", String::from_utf8_lossy(&bs.to_bytes()));
117///
118/// // Get object metadata.
119/// let meta = op.stat("test").await?;
120/// info!("meta: {:?}", meta);
121///
122/// // Export prometheus metrics.
123/// let mut output = String::new();
124/// with_global_registry(|registry| text::encode(&mut output, &registry).unwrap());
125/// println!("{}", output);
126/// # Ok(())
127/// # }
128#[derive(Clone, Debug)]
129pub struct FastmetricsLayer {
130    interceptor: FastmetricsInterceptor,
131}
132
133impl FastmetricsLayer {
134    /// Create a [`FastmetricsLayerBuilder`] to set the configuration of metrics.
135    pub fn builder() -> FastmetricsLayerBuilder {
136        FastmetricsLayerBuilder::default()
137    }
138}
139
140impl<A: Access> Layer<A> for FastmetricsLayer {
141    type LayeredAccess = observe::MetricsAccessor<A, FastmetricsInterceptor>;
142
143    fn layer(&self, inner: A) -> Self::LayeredAccess {
144        observe::MetricsLayer::new(self.interceptor.clone()).layer(inner)
145    }
146}
147
148/// [`FastmetricsLayerBuilder`] is a config builder to build a [`FastmetricsLayer`].
149pub struct FastmetricsLayerBuilder {
150    bytes_buckets: Vec<f64>,
151    bytes_rate_buckets: Vec<f64>,
152    entries_buckets: Vec<f64>,
153    entries_rate_buckets: Vec<f64>,
154    duration_seconds_buckets: Vec<f64>,
155    ttfb_buckets: Vec<f64>,
156    disable_label_root: bool,
157}
158
159impl Default for FastmetricsLayerBuilder {
160    fn default() -> Self {
161        Self {
162            bytes_buckets: observe::DEFAULT_BYTES_BUCKETS.to_vec(),
163            bytes_rate_buckets: observe::DEFAULT_BYTES_RATE_BUCKETS.to_vec(),
164            entries_buckets: observe::DEFAULT_ENTRIES_BUCKETS.to_vec(),
165            entries_rate_buckets: observe::DEFAULT_ENTRIES_RATE_BUCKETS.to_vec(),
166            duration_seconds_buckets: observe::DEFAULT_DURATION_SECONDS_BUCKETS.to_vec(),
167            ttfb_buckets: observe::DEFAULT_TTFB_BUCKETS.to_vec(),
168            disable_label_root: false,
169        }
170    }
171}
172
173impl FastmetricsLayerBuilder {
174    /// Set buckets for bytes related histogram like `operation_bytes`.
175    pub fn bytes_buckets(mut self, buckets: Vec<f64>) -> Self {
176        if !buckets.is_empty() {
177            self.bytes_buckets = buckets;
178        }
179        self
180    }
181
182    /// Set buckets for bytes rate related histogram like `operation_bytes_rate`.
183    pub fn bytes_rate_buckets(mut self, buckets: Vec<f64>) -> Self {
184        if !buckets.is_empty() {
185            self.bytes_rate_buckets = buckets;
186        }
187        self
188    }
189
190    /// Set buckets for entries related histogram like `operation_entries`.
191    pub fn entries_buckets(mut self, buckets: Vec<f64>) -> Self {
192        if !buckets.is_empty() {
193            self.entries_buckets = buckets;
194        }
195        self
196    }
197
198    /// Set buckets for entries rate related histogram like `operation_entries_rate`.
199    pub fn entries_rate_buckets(mut self, buckets: Vec<f64>) -> Self {
200        if !buckets.is_empty() {
201            self.entries_rate_buckets = buckets;
202        }
203        self
204    }
205
206    /// Set buckets for duration seconds related histogram like `operation_duration_seconds`.
207    pub fn duration_seconds_buckets(mut self, buckets: Vec<f64>) -> Self {
208        if !buckets.is_empty() {
209            self.duration_seconds_buckets = buckets;
210        }
211        self
212    }
213
214    /// Set buckets for ttfb related histogram like `operation_ttfb_seconds`.
215    pub fn ttfb_buckets(mut self, buckets: Vec<f64>) -> Self {
216        if !buckets.is_empty() {
217            self.ttfb_buckets = buckets;
218        }
219        self
220    }
221
222    /// The 'root' label might have risks of being high cardinality; users can choose to disable it
223    /// when they found it's not useful for their metrics.
224    pub fn disable_label_root(mut self, disable: bool) -> Self {
225        self.disable_label_root = disable;
226        self
227    }
228
229    /// Register the metrics into the registry and return a [`FastmetricsLayer`].
230    ///
231    /// # Example
232    ///
233    /// ```no_run
234    /// # use opendal::layers::FastmetricsLayer;
235    /// # use opendal::services;
236    /// # use opendal::Operator;
237    /// # use opendal::Result;
238    ///
239    /// # #[tokio::main]
240    /// # async fn main() -> Result<()> {
241    /// let mut registry = fastmetrics::registry::Registry::default();
242    ///
243    /// // Pick a builder and configure it.
244    /// let builder = services::Memory::default();
245    /// let _ = Operator::new(builder)?
246    ///     .layer(FastmetricsLayer::builder().register(&mut registry)?)
247    ///     .finish();
248    /// # Ok(())
249    /// # }
250    /// ```
251    pub fn register(self, registry: &mut Registry) -> Result<FastmetricsLayer> {
252        let operation_bytes = Family::new(HistogramFactory {
253            buckets: self.bytes_buckets.clone(),
254        });
255        let operation_bytes_rate = Family::new(HistogramFactory {
256            buckets: self.bytes_rate_buckets.clone(),
257        });
258        let operation_entries = Family::new(HistogramFactory {
259            buckets: self.entries_buckets.clone(),
260        });
261        let operation_entries_rate = Family::new(HistogramFactory {
262            buckets: self.entries_rate_buckets.clone(),
263        });
264        let operation_duration_seconds = Family::new(HistogramFactory {
265            buckets: self.duration_seconds_buckets.clone(),
266        });
267        let operation_errors_total = Family::default();
268        let operation_executing = Family::default();
269        let operation_ttfb_seconds = Family::new(HistogramFactory {
270            buckets: self.ttfb_buckets.clone(),
271        });
272
273        let http_executing = Family::default();
274        let http_request_bytes = Family::new(HistogramFactory {
275            buckets: self.bytes_buckets.clone(),
276        });
277        let http_request_bytes_rate = Family::new(HistogramFactory {
278            buckets: self.bytes_rate_buckets.clone(),
279        });
280        let http_request_duration_seconds = Family::new(HistogramFactory {
281            buckets: self.duration_seconds_buckets.clone(),
282        });
283        let http_response_bytes = Family::new(HistogramFactory {
284            buckets: self.bytes_buckets.clone(),
285        });
286        let http_response_bytes_rate = Family::new(HistogramFactory {
287            buckets: self.bytes_rate_buckets.clone(),
288        });
289        let http_response_duration_seconds = Family::new(HistogramFactory {
290            buckets: self.duration_seconds_buckets.clone(),
291        });
292        let http_connection_errors_total = Family::default();
293        let http_status_errors_total = Family::default();
294
295        let interceptor = FastmetricsInterceptor {
296            operation_bytes,
297            operation_bytes_rate,
298            operation_entries,
299            operation_entries_rate,
300            operation_duration_seconds,
301            operation_errors_total,
302            operation_executing,
303            operation_ttfb_seconds,
304
305            http_executing,
306            http_request_bytes,
307            http_request_bytes_rate,
308            http_request_duration_seconds,
309            http_response_bytes,
310            http_response_bytes_rate,
311            http_response_duration_seconds,
312            http_connection_errors_total,
313            http_status_errors_total,
314
315            disable_label_root: self.disable_label_root,
316        };
317        interceptor
318            .register(registry)
319            .map_err(|err| Error::new(ErrorKind::Unexpected, err.to_string()).set_source(err))?;
320
321        Ok(FastmetricsLayer { interceptor })
322    }
323
324    /// Register the metrics into the global registry and return a [`FastmetricsLayer`].
325    ///
326    /// # Example
327    ///
328    /// ```no_run
329    /// # use opendal::layers::FastmetricsLayer;
330    /// # use opendal::services;
331    /// # use opendal::Operator;
332    /// # use opendal::Result;
333    ///
334    /// # fn main() -> Result<()> {
335    /// // Pick a builder and configure it.
336    /// let builder = services::Memory::default();
337    /// let _ = Operator::new(builder)?
338    ///     .layer(FastmetricsLayer::builder().register_global()?)
339    ///     .finish();
340    /// # Ok(())
341    /// # }
342    /// ```
343    pub fn register_global(self) -> Result<FastmetricsLayer> {
344        with_global_registry_mut(|registry| self.register(registry))
345    }
346}
347
348#[derive(Clone)]
349struct HistogramFactory {
350    buckets: Vec<f64>,
351}
352
353impl MetricFactory<Histogram> for HistogramFactory {
354    fn new_metric(&self) -> Histogram {
355        Histogram::new(self.buckets.iter().cloned())
356    }
357}
358
359#[derive(Clone, Debug)]
360pub struct FastmetricsInterceptor {
361    operation_bytes: Family<OperationLabels, Histogram, HistogramFactory>,
362    operation_bytes_rate: Family<OperationLabels, Histogram, HistogramFactory>,
363    operation_entries: Family<OperationLabels, Histogram, HistogramFactory>,
364    operation_entries_rate: Family<OperationLabels, Histogram, HistogramFactory>,
365    operation_duration_seconds: Family<OperationLabels, Histogram, HistogramFactory>,
366    operation_errors_total: Family<OperationLabels, Counter>,
367    operation_executing: Family<OperationLabels, Gauge>,
368    operation_ttfb_seconds: Family<OperationLabels, Histogram, HistogramFactory>,
369
370    http_executing: Family<OperationLabels, Gauge>,
371    http_request_bytes: Family<OperationLabels, Histogram, HistogramFactory>,
372    http_request_bytes_rate: Family<OperationLabels, Histogram, HistogramFactory>,
373    http_request_duration_seconds: Family<OperationLabels, Histogram, HistogramFactory>,
374    http_response_bytes: Family<OperationLabels, Histogram, HistogramFactory>,
375    http_response_bytes_rate: Family<OperationLabels, Histogram, HistogramFactory>,
376    http_response_duration_seconds: Family<OperationLabels, Histogram, HistogramFactory>,
377    http_connection_errors_total: Family<OperationLabels, Counter>,
378    http_status_errors_total: Family<OperationLabels, Counter>,
379
380    disable_label_root: bool,
381}
382
383impl Register for FastmetricsInterceptor {
384    fn register(&self, registry: &mut Registry) -> Result<(), RegistryError> {
385        macro_rules! register_metrics {
386            ($($field:ident => $value:expr),* $(,)?) => {
387                $(
388                    {
389                        let ((name, unit), help) = ($value.name_with_unit(), $value.help());
390                        registry.register_metric(name, help, unit, self.$field.clone())?;
391                    }
392                )*
393            };
394        }
395
396        register_metrics! {
397            // Operation metrics
398            operation_bytes => observe::MetricValue::OperationBytes(0),
399            operation_bytes_rate => observe::MetricValue::OperationBytesRate(0.0),
400            operation_entries => observe::MetricValue::OperationEntries(0),
401            operation_entries_rate => observe::MetricValue::OperationEntriesRate(0.0),
402            operation_duration_seconds => observe::MetricValue::OperationDurationSeconds(Duration::default()),
403            operation_errors_total => observe::MetricValue::OperationErrorsTotal,
404            operation_executing => observe::MetricValue::OperationExecuting(0),
405            operation_ttfb_seconds => observe::MetricValue::OperationTtfbSeconds(Duration::default()),
406
407            // HTTP metrics
408            http_executing => observe::MetricValue::HttpExecuting(0),
409            http_request_bytes => observe::MetricValue::HttpRequestBytes(0),
410            http_request_bytes_rate => observe::MetricValue::HttpRequestBytesRate(0.0),
411            http_request_duration_seconds => observe::MetricValue::HttpRequestDurationSeconds(Duration::default()),
412            http_response_bytes => observe::MetricValue::HttpResponseBytes(0),
413            http_response_bytes_rate => observe::MetricValue::HttpResponseBytesRate(0.0),
414            http_response_duration_seconds => observe::MetricValue::HttpResponseDurationSeconds(Duration::default()),
415            http_connection_errors_total => observe::MetricValue::HttpConnectionErrorsTotal,
416            http_status_errors_total => observe::MetricValue::HttpStatusErrorsTotal,
417        }
418
419        Ok(())
420    }
421}
422
423impl observe::MetricsIntercept for FastmetricsInterceptor {
424    fn observe(&self, labels: observe::MetricLabels, value: observe::MetricValue) {
425        let labels = OperationLabels {
426            labels,
427            disable_label_root: self.disable_label_root,
428        };
429        match value {
430            observe::MetricValue::OperationBytes(v) => {
431                self.operation_bytes
432                    .with_or_new(&labels, |hist| hist.observe(v as f64));
433            }
434            observe::MetricValue::OperationBytesRate(v) => {
435                self.operation_bytes_rate
436                    .with_or_new(&labels, |hist| hist.observe(v));
437            }
438            observe::MetricValue::OperationEntries(v) => {
439                self.operation_entries
440                    .with_or_new(&labels, |hist| hist.observe(v as f64));
441            }
442            observe::MetricValue::OperationEntriesRate(v) => {
443                self.operation_entries_rate
444                    .with_or_new(&labels, |hist| hist.observe(v));
445            }
446            observe::MetricValue::OperationDurationSeconds(v) => {
447                self.operation_duration_seconds
448                    .with_or_new(&labels, |hist| hist.observe(v.as_secs_f64()));
449            }
450            observe::MetricValue::OperationErrorsTotal => {
451                self.operation_errors_total
452                    .with_or_new(&labels, |counter| counter.inc());
453            }
454            observe::MetricValue::OperationExecuting(v) => {
455                self.operation_executing
456                    .with_or_new(&labels, |gauge| gauge.inc_by(v as i64));
457            }
458            observe::MetricValue::OperationTtfbSeconds(v) => {
459                self.operation_ttfb_seconds
460                    .with_or_new(&labels, |hist| hist.observe(v.as_secs_f64()));
461            }
462
463            observe::MetricValue::HttpExecuting(v) => {
464                self.http_executing
465                    .with_or_new(&labels, |gauge| gauge.inc_by(v as i64));
466            }
467            observe::MetricValue::HttpRequestBytes(v) => {
468                self.http_request_bytes
469                    .with_or_new(&labels, |hist| hist.observe(v as f64));
470            }
471            observe::MetricValue::HttpRequestBytesRate(v) => {
472                self.http_request_bytes_rate
473                    .with_or_new(&labels, |hist| hist.observe(v));
474            }
475            observe::MetricValue::HttpRequestDurationSeconds(v) => {
476                self.http_request_duration_seconds
477                    .with_or_new(&labels, |hist| hist.observe(v.as_secs_f64()));
478            }
479            observe::MetricValue::HttpResponseBytes(v) => {
480                self.http_response_bytes
481                    .with_or_new(&labels, |hist| hist.observe(v as f64));
482            }
483            observe::MetricValue::HttpResponseBytesRate(v) => {
484                self.http_response_bytes_rate
485                    .with_or_new(&labels, |hist| hist.observe(v));
486            }
487            observe::MetricValue::HttpResponseDurationSeconds(v) => {
488                self.http_response_duration_seconds
489                    .with_or_new(&labels, |hist| hist.observe(v.as_secs_f64()));
490            }
491            observe::MetricValue::HttpConnectionErrorsTotal => {
492                self.http_connection_errors_total
493                    .with_or_new(&labels, |counter| counter.inc());
494            }
495            observe::MetricValue::HttpStatusErrorsTotal => {
496                self.http_status_errors_total
497                    .with_or_new(&labels, |counter| counter.inc());
498            }
499        };
500    }
501}
502
503#[derive(Clone, Debug, PartialEq, Eq, Hash)]
504struct OperationLabels {
505    labels: observe::MetricLabels,
506    disable_label_root: bool,
507}
508
509impl EncodeLabelSet for OperationLabels {
510    fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> fmt::Result {
511        encoder.encode(&(observe::LABEL_SCHEME, self.labels.scheme))?;
512        encoder.encode(&(observe::LABEL_NAMESPACE, self.labels.namespace.as_ref()))?;
513        if !self.disable_label_root {
514            encoder.encode(&(observe::LABEL_ROOT, self.labels.root.as_ref()))?;
515        }
516        encoder.encode(&(observe::LABEL_OPERATION, self.labels.operation))?;
517        if let Some(error) = &self.labels.error {
518            encoder.encode(&(observe::LABEL_ERROR, error.into_static()))?;
519        }
520        if let Some(code) = &self.labels.status_code {
521            encoder.encode(&(observe::LABEL_STATUS_CODE, code.as_str()))?;
522        }
523        Ok(())
524    }
525}
```
