# 

opendal/layers/

otelmetrics.rs

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
20use opentelemetry::KeyValue;
21use opentelemetry::metrics::Counter;
22use opentelemetry::metrics::Histogram;
23use opentelemetry::metrics::Meter;
24use opentelemetry::metrics::UpDownCounter;
25
26use crate::layers::observe;
27use crate::raw::*;
28
29/// Add [opentelemetry::metrics](https://docs.rs/opentelemetry/latest/opentelemetry/metrics/index.html) for every operation.
30///
31/// # Examples
32///
33/// ```no_run
34/// # use opendal::layers::OtelMetricsLayer;
35/// # use opendal::services;
36/// # use opendal::Operator;
37/// # use opendal::Result;
38///
39/// # fn main() -> Result<()> {
40/// let meter = opentelemetry::global::meter("opendal");
41/// let _ = Operator::new(services::Memory::default())?
42///     .layer(OtelMetricsLayer::builder().register(&meter))
43///     .finish();
44/// Ok(())
45/// # }
46/// ```
47#[derive(Clone, Debug)]
48pub struct OtelMetricsLayer {
49    interceptor: OtelMetricsInterceptor,
50}
51
52impl OtelMetricsLayer {
53    /// Create a [`OtelMetricsLayerBuilder`] to set the configuration of metrics.
54    ///
55    /// # Examples
56    ///
57    /// ```no_run
58    /// # use opendal::layers::OtelMetricsLayer;
59    /// # use opendal::services;
60    /// # use opendal::Operator;
61    /// # use opendal::Result;
62    ///
63    /// # #[tokio::main]
64    /// # async fn main() -> Result<()> {
65    /// let meter = opentelemetry::global::meter("opendal");
66    /// let op = Operator::new(services::Memory::default())?
67    ///     .layer(OtelMetricsLayer::builder().register(&meter))
68    ///     .finish();
69    ///
70    /// Ok(())
71    /// # }
72    /// ```
73    pub fn builder() -> OtelMetricsLayerBuilder {
74        OtelMetricsLayerBuilder::default()
75    }
76}
77
78/// [`OtelMetricsLayerBuilder`] is a config builder to build a [`OtelMetricsLayer`].
79pub struct OtelMetricsLayerBuilder {
80    bytes_boundaries: Vec<f64>,
81    bytes_rate_boundaries: Vec<f64>,
82    entries_boundaries: Vec<f64>,
83    entries_rate_boundaries: Vec<f64>,
84    duration_seconds_boundaries: Vec<f64>,
85    ttfb_boundaries: Vec<f64>,
86}
87
88impl Default for OtelMetricsLayerBuilder {
89    fn default() -> Self {
90        Self {
91            bytes_boundaries: observe::DEFAULT_BYTES_BUCKETS.to_vec(),
92            bytes_rate_boundaries: observe::DEFAULT_BYTES_RATE_BUCKETS.to_vec(),
93            entries_boundaries: observe::DEFAULT_ENTRIES_BUCKETS.to_vec(),
94            entries_rate_boundaries: observe::DEFAULT_ENTRIES_RATE_BUCKETS.to_vec(),
95            duration_seconds_boundaries: observe::DEFAULT_DURATION_SECONDS_BUCKETS.to_vec(),
96            ttfb_boundaries: observe::DEFAULT_TTFB_BUCKETS.to_vec(),
97        }
98    }
99}
100
101impl OtelMetricsLayerBuilder {
102    /// Set boundaries for bytes related histogram like `operation_bytes`.
103    pub fn bytes_boundaries(mut self, boundaries: Vec<f64>) -> Self {
104        if !boundaries.is_empty() {
105            self.bytes_boundaries = boundaries;
106        }
107        self
108    }
109
110    /// Set boundaries for bytes rate related histogram like `operation_bytes_rate`.
111    pub fn bytes_rate_boundaries(mut self, boundaries: Vec<f64>) -> Self {
112        if !boundaries.is_empty() {
113            self.bytes_rate_boundaries = boundaries;
114        }
115        self
116    }
117
118    /// Set boundaries for entries related histogram like `operation_entries`.
119    pub fn entries_boundaries(mut self, boundaries: Vec<f64>) -> Self {
120        if !boundaries.is_empty() {
121            self.entries_boundaries = boundaries;
122        }
123        self
124    }
125
126    /// Set boundaries for entries rate related histogram like `operation_entries_rate`.
127    pub fn entries_rate_boundaries(mut self, boundaries: Vec<f64>) -> Self {
128        if !boundaries.is_empty() {
129            self.entries_rate_boundaries = boundaries;
130        }
131        self
132    }
133
134    /// Set boundaries for duration seconds related histogram like `operation_duration_seconds`.
135    pub fn duration_seconds_boundaries(mut self, boundaries: Vec<f64>) -> Self {
136        if !boundaries.is_empty() {
137            self.duration_seconds_boundaries = boundaries;
138        }
139        self
140    }
141
142    /// Set boundaries for ttfb related histogram like `operation_ttfb_seconds`.
143    pub fn ttfb_boundaries(mut self, boundaries: Vec<f64>) -> Self {
144        if !boundaries.is_empty() {
145            self.ttfb_boundaries = boundaries;
146        }
147        self
148    }
149
150    /// Register the metrics and return a [`OtelMetricsLayer`].
151    ///
152    /// # Examples
153    ///
154    /// ```no_run
155    /// # use opendal::layers::OtelMetricsLayer;
156    /// # use opendal::services;
157    /// # use opendal::Operator;
158    /// # use opendal::Result;
159    ///
160    /// # #[tokio::main]
161    /// # async fn main() -> Result<()> {
162    /// let meter = opentelemetry::global::meter("opendal");
163    /// let op = Operator::new(services::Memory::default())?
164    ///     .layer(OtelMetricsLayer::builder().register(&meter))
165    ///     .finish();
166    ///
167    /// Ok(())
168    /// # }
169    /// ```
170    pub fn register(self, meter: &Meter) -> OtelMetricsLayer {
171        let operation_bytes = {
172            let metric = observe::MetricValue::OperationBytes(0);
173            register_u64_histogram_meter(
174                meter,
175                "opendal.operation.bytes",
176                metric,
177                self.bytes_boundaries.clone(),
178            )
179        };
180        let operation_bytes_rate = {
181            let metric = observe::MetricValue::OperationBytesRate(0.0);
182            register_f64_histogram_meter(
183                meter,
184                "opendal.operation.bytes_rate",
185                metric,
186                self.bytes_rate_boundaries.clone(),
187            )
188        };
189        let operation_entries = {
190            let metric = observe::MetricValue::OperationEntries(0);
191            register_u64_histogram_meter(
192                meter,
193                "opendal.operation.entries",
194                metric,
195                self.entries_boundaries.clone(),
196            )
197        };
198        let operation_entries_rate = {
199            let metric = observe::MetricValue::OperationEntriesRate(0.0);
200            register_f64_histogram_meter(
201                meter,
202                "opendal.operation.entries_rate",
203                metric,
204                self.entries_rate_boundaries.clone(),
205            )
206        };
207        let operation_duration_seconds = {
208            let metric = observe::MetricValue::OperationDurationSeconds(Duration::default());
209            register_f64_histogram_meter(
210                meter,
211                "opendal.operation.duration",
212                metric,
213                self.duration_seconds_boundaries.clone(),
214            )
215        };
216        let operation_errors_total = {
217            let metric = observe::MetricValue::OperationErrorsTotal;
218            meter
219                .u64_counter("opendal.operation.errors")
220                .with_description(metric.help())
221                .build()
222        };
223        let operation_executing = {
224            let metric = observe::MetricValue::OperationExecuting(0);
225            meter
226                .i64_up_down_counter("opendal.operation.executing")
227                .with_description(metric.help())
228                .build()
229        };
230        let operation_ttfb_seconds = {
231            let metric = observe::MetricValue::OperationTtfbSeconds(Duration::default());
232            register_f64_histogram_meter(
233                meter,
234                "opendal.operation.ttfb",
235                metric,
236                self.duration_seconds_boundaries.clone(),
237            )
238        };
239
240        let http_executing = {
241            let metric = observe::MetricValue::HttpExecuting(0);
242            meter
243                .i64_up_down_counter("opendal.http.executing")
244                .with_description(metric.help())
245                .build()
246        };
247        let http_request_bytes = {
248            let metric = observe::MetricValue::HttpRequestBytes(0);
249            register_u64_histogram_meter(
250                meter,
251                "opendal.http.request.bytes",
252                metric,
253                self.bytes_boundaries.clone(),
254            )
255        };
256        let http_request_bytes_rate = {
257            let metric = observe::MetricValue::HttpRequestBytesRate(0.0);
258            register_f64_histogram_meter(
259                meter,
260                "opendal.http.request.bytes_rate",
261                metric,
262                self.bytes_rate_boundaries.clone(),
263            )
264        };
265        let http_request_duration_seconds = {
266            let metric = observe::MetricValue::HttpRequestDurationSeconds(Duration::default());
267            register_f64_histogram_meter(
268                meter,
269                "opendal.http.request.duration",
270                metric,
271                self.duration_seconds_boundaries.clone(),
272            )
273        };
274        let http_response_bytes = {
275            let metric = observe::MetricValue::HttpResponseBytes(0);
276            register_u64_histogram_meter(
277                meter,
278                "opendal.http.response.bytes",
279                metric,
280                self.bytes_boundaries.clone(),
281            )
282        };
283        let http_response_bytes_rate = {
284            let metric = observe::MetricValue::HttpResponseBytesRate(0.0);
285            register_f64_histogram_meter(
286                meter,
287                "opendal.http.response.bytes_rate",
288                metric,
289                self.bytes_rate_boundaries.clone(),
290            )
291        };
292        let http_response_duration_seconds = {
293            let metric = observe::MetricValue::HttpResponseDurationSeconds(Duration::default());
294            register_f64_histogram_meter(
295                meter,
296                "opendal.http.response.duration",
297                metric,
298                self.duration_seconds_boundaries.clone(),
299            )
300        };
301        let http_connection_errors_total = {
302            let metric = observe::MetricValue::HttpConnectionErrorsTotal;
303            meter
304                .u64_counter("opendal.http.connection_errors")
305                .with_description(metric.help())
306                .build()
307        };
308        let http_status_errors_total = {
309            let metric = observe::MetricValue::HttpStatusErrorsTotal;
310            meter
311                .u64_counter("opendal.http.status_errors")
312                .with_description(metric.help())
313                .build()
314        };
315
316        OtelMetricsLayer {
317            interceptor: OtelMetricsInterceptor {
318                operation_bytes,
319                operation_bytes_rate,
320                operation_entries,
321                operation_entries_rate,
322                operation_duration_seconds,
323                operation_errors_total,
324                operation_executing,
325                operation_ttfb_seconds,
326
327                http_executing,
328                http_request_bytes,
329                http_request_bytes_rate,
330                http_request_duration_seconds,
331                http_response_bytes,
332                http_response_bytes_rate,
333                http_response_duration_seconds,
334                http_connection_errors_total,
335                http_status_errors_total,
336            },
337        }
338    }
339}
340
341impl<A: Access> Layer<A> for OtelMetricsLayer {
342    type LayeredAccess = observe::MetricsAccessor<A, OtelMetricsInterceptor>;
343
344    fn layer(&self, inner: A) -> Self::LayeredAccess {
345        observe::MetricsLayer::new(self.interceptor.clone()).layer(inner)
346    }
347}
348
349#[derive(Clone, Debug)]
350pub struct OtelMetricsInterceptor {
351    operation_bytes: Histogram<u64>,
352    operation_bytes_rate: Histogram<f64>,
353    operation_entries: Histogram<u64>,
354    operation_entries_rate: Histogram<f64>,
355    operation_duration_seconds: Histogram<f64>,
356    operation_errors_total: Counter<u64>,
357    operation_executing: UpDownCounter<i64>,
358    operation_ttfb_seconds: Histogram<f64>,
359
360    http_executing: UpDownCounter<i64>,
361    http_request_bytes: Histogram<u64>,
362    http_request_bytes_rate: Histogram<f64>,
363    http_request_duration_seconds: Histogram<f64>,
364    http_response_bytes: Histogram<u64>,
365    http_response_bytes_rate: Histogram<f64>,
366    http_response_duration_seconds: Histogram<f64>,
367    http_connection_errors_total: Counter<u64>,
368    http_status_errors_total: Counter<u64>,
369}
370
371impl observe::MetricsIntercept for OtelMetricsInterceptor {
372    fn observe(&self, labels: observe::MetricLabels, value: observe::MetricValue) {
373        let attributes = self.create_attributes(labels);
374
375        match value {
376            observe::MetricValue::OperationBytes(v) => self.operation_bytes.record(v, &attributes),
377            observe::MetricValue::OperationBytesRate(v) => {
378                self.operation_bytes_rate.record(v, &attributes)
379            }
380            observe::MetricValue::OperationEntries(v) => {
381                self.operation_entries.record(v, &attributes)
382            }
383            observe::MetricValue::OperationEntriesRate(v) => {
384                self.operation_entries_rate.record(v, &attributes)
385            }
386            observe::MetricValue::OperationDurationSeconds(v) => self
387                .operation_duration_seconds
388                .record(v.as_secs_f64(), &attributes),
389            observe::MetricValue::OperationErrorsTotal => {
390                self.operation_errors_total.add(1, &attributes)
391            }
392            observe::MetricValue::OperationExecuting(v) => {
393                self.operation_executing.add(v as i64, &attributes)
394            }
395            observe::MetricValue::OperationTtfbSeconds(v) => self
396                .operation_ttfb_seconds
397                .record(v.as_secs_f64(), &attributes),
398
399            observe::MetricValue::HttpExecuting(v) => {
400                self.http_executing.add(v as i64, &attributes)
401            }
402            observe::MetricValue::HttpRequestBytes(v) => {
403                self.http_request_bytes.record(v, &attributes)
404            }
405            observe::MetricValue::HttpRequestBytesRate(v) => {
406                self.http_request_bytes_rate.record(v, &attributes)
407            }
408            observe::MetricValue::HttpRequestDurationSeconds(v) => self
409                .http_request_duration_seconds
410                .record(v.as_secs_f64(), &attributes),
411            observe::MetricValue::HttpResponseBytes(v) => {
412                self.http_response_bytes.record(v, &attributes)
413            }
414            observe::MetricValue::HttpResponseBytesRate(v) => {
415                self.http_response_bytes_rate.record(v, &attributes)
416            }
417            observe::MetricValue::HttpResponseDurationSeconds(v) => self
418                .http_response_duration_seconds
419                .record(v.as_secs_f64(), &attributes),
420            observe::MetricValue::HttpConnectionErrorsTotal => {
421                self.http_connection_errors_total.add(1, &attributes)
422            }
423            observe::MetricValue::HttpStatusErrorsTotal => {
424                self.http_status_errors_total.add(1, &attributes)
425            }
426        }
427    }
428}
429
430impl OtelMetricsInterceptor {
431    fn create_attributes(&self, attrs: observe::MetricLabels) -> Vec<KeyValue> {
432        let mut attributes = Vec::with_capacity(6);
433
434        attributes.extend([
435            KeyValue::new(observe::LABEL_SCHEME, attrs.scheme),
436            KeyValue::new(observe::LABEL_NAMESPACE, attrs.namespace),
437            KeyValue::new(observe::LABEL_ROOT, attrs.root),
438            KeyValue::new(observe::LABEL_OPERATION, attrs.operation),
439        ]);
440
441        if let Some(error) = attrs.error {
442            attributes.push(KeyValue::new(observe::LABEL_ERROR, error.into_static()));
443        }
444
445        if let Some(status_code) = attrs.status_code {
446            attributes.push(KeyValue::new(
447                observe::LABEL_STATUS_CODE,
448                status_code.as_u16() as i64,
449            ));
450        }
451
452        attributes
453    }
454}
455
456fn register_u64_histogram_meter(
457    meter: &Meter,
458    name: &'static str,
459    metric: observe::MetricValue,
460    boundaries: Vec<f64>,
461) -> Histogram<u64> {
462    let (_name, unit) = metric.name_with_unit();
463    let description = metric.help();
464
465    let builder = meter
466        .u64_histogram(name)
467        .with_description(description)
468        .with_boundaries(boundaries);
469
470    if let Some(unit) = unit {
471        builder.with_unit(unit).build()
472    } else {
473        builder.build()
474    }
475}
476
477fn register_f64_histogram_meter(
478    meter: &Meter,
479    name: &'static str,
480    metric: observe::MetricValue,
481    boundaries: Vec<f64>,
482) -> Histogram<f64> {
483    let (_name, unit) = metric.name_with_unit();
484    let description = metric.help();
485
486    let builder = meter
487        .f64_histogram(name)
488        .with_description(description)
489        .with_boundaries(boundaries);
490
491    if let Some(unit) = unit {
492        builder.with_unit(unit).build()
493    } else {
494        builder.build()
495    }
496}
```
