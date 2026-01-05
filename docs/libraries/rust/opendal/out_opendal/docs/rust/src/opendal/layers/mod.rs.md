# 

opendal/layers/

mod.rs

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
18//! `Layer` is the mechanism to intercept operations.
19
20mod type_eraser;
21pub(crate) use type_eraser::TypeEraseLayer;
22
23mod error_context;
24pub(crate) use error_context::ErrorContextLayer;
25
26mod complete;
27pub(crate) use complete::CompleteLayer;
28
29mod concurrent_limit;
30pub use concurrent_limit::ConcurrentLimitLayer;
31
32mod immutable_index;
33pub use immutable_index::ImmutableIndexLayer;
34
35mod logging;
36pub use logging::LoggingInterceptor;
37pub use logging::LoggingLayer;
38
39mod timeout;
40pub use timeout::TimeoutLayer;
41
42#[cfg(feature = "layers-chaos")]
43mod chaos;
44#[cfg(feature = "layers-chaos")]
45pub use chaos::ChaosLayer;
46
47#[cfg(feature = "layers-metrics")]
48mod metrics;
49#[cfg(feature = "layers-metrics")]
50pub use self::metrics::MetricsLayer;
51
52#[cfg(feature = "layers-mime-guess")]
53mod mime_guess;
54#[cfg(feature = "layers-mime-guess")]
55pub use self::mime_guess::MimeGuessLayer;
56
57#[cfg(feature = "layers-prometheus")]
58mod prometheus;
59#[cfg(feature = "layers-prometheus")]
60pub use self::prometheus::PrometheusLayer;
61#[cfg(feature = "layers-prometheus")]
62pub use self::prometheus::PrometheusLayerBuilder;
63
64#[cfg(feature = "layers-prometheus-client")]
65mod prometheus_client;
66#[cfg(feature = "layers-prometheus-client")]
67pub use self::prometheus_client::PrometheusClientLayer;
68#[cfg(feature = "layers-prometheus-client")]
69pub use self::prometheus_client::PrometheusClientLayerBuilder;
70
71#[cfg(feature = "layers-fastmetrics")]
72mod fastmetrics;
73#[cfg(feature = "layers-fastmetrics")]
74pub use self::fastmetrics::FastmetricsLayer;
75#[cfg(feature = "layers-fastmetrics")]
76pub use self::fastmetrics::FastmetricsLayerBuilder;
77
78mod retry;
79pub use self::retry::RetryInterceptor;
80pub use self::retry::RetryLayer;
81
82mod tail_cut;
83pub use self::tail_cut::TailCutLayer;
84pub use self::tail_cut::TailCutLayerBuilder;
85
86#[cfg(feature = "layers-tracing")]
87mod tracing;
88#[cfg(feature = "layers-tracing")]
89pub use self::tracing::TracingLayer;
90
91#[cfg(feature = "layers-fastrace")]
92mod fastrace;
93#[cfg(feature = "layers-fastrace")]
94pub use self::fastrace::FastraceLayer;
95
96#[cfg(feature = "layers-otel-metrics")]
97mod otelmetrics;
98#[cfg(feature = "layers-otel-metrics")]
99pub use self::otelmetrics::OtelMetricsLayer;
100
101#[cfg(feature = "layers-otel-trace")]
102mod oteltrace;
103#[cfg(feature = "layers-otel-trace")]
104pub use self::oteltrace::OtelTraceLayer;
105
106#[cfg(feature = "layers-throttle")]
107mod throttle;
108#[cfg(feature = "layers-throttle")]
109pub use self::throttle::ThrottleLayer;
110
111#[cfg(feature = "layers-await-tree")]
112mod await_tree;
113#[cfg(feature = "layers-await-tree")]
114pub use self::await_tree::AwaitTreeLayer;
115
116#[cfg(feature = "layers-async-backtrace")]
117mod async_backtrace;
118#[cfg(feature = "layers-async-backtrace")]
119pub use self::async_backtrace::AsyncBacktraceLayer;
120
121#[cfg(all(target_os = "linux", feature = "layers-dtrace"))]
122mod dtrace;
123#[cfg(all(target_os = "linux", feature = "layers-dtrace"))]
124pub use self::dtrace::DtraceLayer;
125
126pub mod observe;
127
128mod correctness_check;
129pub(crate) use correctness_check::CorrectnessCheckLayer;
130mod capability_check;
131pub use capability_check::CapabilityCheckLayer;
132
133mod http_client;
134pub use http_client::HttpClientLayer;
```
