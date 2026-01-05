# 

opendal/layers/

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
18use metrics::Label;
19use metrics::counter;
20use metrics::gauge;
21use metrics::histogram;
22
23use crate::layers::observe;
24use crate::raw::*;
25
26/// Add [metrics](https://docs.rs/metrics/) for every operation.
27///
28/// # Metrics
29///
30/// We provide several metrics, please see the documentation of [`observe`] module.
31///
32/// # Notes
33///
34/// Please make sure the exporter has been pulled in regular time.
35/// Otherwise, the histogram data collected by `requests_duration_seconds`
36/// could result in OOM.
37///
38/// # Examples
39///
40/// ```no_run
41/// # use opendal::layers::MetricsLayer;
42/// # use opendal::services;
43/// # use opendal::Operator;
44/// # use opendal::Result;
45///
46/// # fn main() -> Result<()> {
47/// let _ = Operator::new(services::Memory::default())?
48///     .layer(MetricsLayer::default())
49///     .finish();
50/// Ok(())
51/// # }
52/// ```
53///
54/// # Output
55///
56/// OpenDAL is using [`metrics`](https://docs.rs/metrics/latest/metrics/) for metrics internally.
57///
58/// To enable metrics output, please enable one of the exporters that `metrics` supports.
59///
60/// Take [`metrics_exporter_prometheus`](https://docs.rs/metrics-exporter-prometheus/latest/metrics_exporter_prometheus/) as an example:
61///
62/// ```ignore
63/// let builder = PrometheusBuilder::new()
64///     .set_buckets_for_metric(
65///         Matcher::Suffix("bytes".into()),
66///         &observe::DEFAULT_BYTES_BUCKETS,
67///     )
68///     .set_buckets_for_metric(
69///         Matcher::Suffix("duration_seconds".into()),
70///         &observe::DEFAULT_DURATION_SECONDS_BUCKETS,
71///     )
72///     // ..
73///     .expect("failed to create builder");
74/// builder.install().expect("failed to install recorder/exporter");
75/// let handle = builder.install_recorder().expect("failed to install recorder");
76/// let (recorder, exporter) = builder.build().expect("failed to build recorder/exporter");
77/// let recorder = builder.build_recorder().expect("failed to build recorder");
78/// ```
79#[derive(Clone, Debug, Default)]
80pub struct MetricsLayer {}
81
82impl<A: Access> Layer<A> for MetricsLayer {
83    type LayeredAccess = observe::MetricsAccessor<A, MetricsInterceptor>;
84
85    fn layer(&self, inner: A) -> Self::LayeredAccess {
86        let interceptor = MetricsInterceptor {};
87        observe::MetricsLayer::new(interceptor).layer(inner)
88    }
89}
90
91#[derive(Clone, Debug)]
92pub struct MetricsInterceptor {}
93
94impl observe::MetricsIntercept for MetricsInterceptor {
95    fn observe(&self, labels: observe::MetricLabels, value: observe::MetricValue) {
96        let labels = OperationLabels(labels).into_labels();
97
98        match value {
99            observe::MetricValue::OperationBytes(v) => {
100                histogram!(value.name(), labels).record(v as f64)
101            }
102            observe::MetricValue::OperationBytesRate(v) => {
103                histogram!(value.name(), labels).record(v)
104            }
105            observe::MetricValue::OperationEntries(v) => {
106                histogram!(value.name(), labels).record(v as f64)
107            }
108            observe::MetricValue::OperationEntriesRate(v) => {
109                histogram!(value.name(), labels).record(v)
110            }
111            observe::MetricValue::OperationDurationSeconds(v) => {
112                histogram!(value.name(), labels).record(v)
113            }
114            observe::MetricValue::OperationErrorsTotal => {
115                counter!(value.name(), labels).increment(1)
116            }
117            observe::MetricValue::OperationExecuting(v) => {
118                gauge!(value.name(), labels).increment(v as f64)
119            }
120            observe::MetricValue::OperationTtfbSeconds(v) => {
121                histogram!(value.name(), labels).record(v)
122            }
123
124            observe::MetricValue::HttpExecuting(v) => {
125                gauge!(value.name(), labels).increment(v as f64)
126            }
127            observe::MetricValue::HttpRequestBytes(v) => {
128                histogram!(value.name(), labels).record(v as f64)
129            }
130            observe::MetricValue::HttpRequestBytesRate(v) => {
131                histogram!(value.name(), labels).record(v)
132            }
133            observe::MetricValue::HttpRequestDurationSeconds(v) => {
134                histogram!(value.name(), labels).record(v)
135            }
136            observe::MetricValue::HttpResponseBytes(v) => {
137                histogram!(value.name(), labels).record(v as f64)
138            }
139            observe::MetricValue::HttpResponseBytesRate(v) => {
140                histogram!(value.name(), labels).record(v)
141            }
142            observe::MetricValue::HttpResponseDurationSeconds(v) => {
143                histogram!(value.name(), labels).record(v)
144            }
145            observe::MetricValue::HttpConnectionErrorsTotal => {
146                counter!(value.name(), labels).increment(1)
147            }
148            observe::MetricValue::HttpStatusErrorsTotal => {
149                counter!(value.name(), labels).increment(1)
150            }
151        }
152    }
153}
154
155#[derive(Clone, Debug, PartialEq, Eq, Hash)]
156struct OperationLabels(observe::MetricLabels);
157
158impl OperationLabels {
159    fn into_labels(self) -> Vec<Label> {
160        let mut labels = Vec::with_capacity(6);
161
162        labels.extend([
163            Label::new(observe::LABEL_SCHEME, self.0.scheme),
164            Label::new(observe::LABEL_NAMESPACE, self.0.namespace),
165            Label::new(observe::LABEL_ROOT, self.0.root),
166            Label::new(observe::LABEL_OPERATION, self.0.operation),
167        ]);
168
169        if let Some(error) = self.0.error {
170            labels.push(Label::new(observe::LABEL_ERROR, error.into_static()));
171        }
172
173        if let Some(status_code) = self.0.status_code {
174            labels.push(Label::new(
175                observe::LABEL_STATUS_CODE,
176                status_code.as_str().to_owned(),
177            ));
178        }
179
180        labels
181    }
182}
```
