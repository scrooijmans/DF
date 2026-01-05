# Module layers Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/mod.rs.html#18-134" class="src">Source</a>

Expand description

`Layer` is the mechanism to intercept operations.

## Modules<a href="https://opendal.apache.org/docs/rust/opendal/layers/index.html#modules" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/index.html" class="mod" title="mod opendal::layers::observe">observe</a>  
OpenDAL Observability

## Structs<a href="https://opendal.apache.org/docs/rust/opendal/layers/index.html#structs" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html" class="struct" title="struct opendal::layers::AsyncBacktraceLayer">AsyncBacktraceLayer</a>`layers-async-backtrace`  
Add Efficient, logical â€˜stackâ€™ traces of async functions for the underlying services.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html" class="struct" title="struct opendal::layers::AwaitTreeLayer">AwaitTreeLayer</a>`layers-await-tree`  
Add an Instrument await-tree for actor-based applications to the underlying services.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.CapabilityCheckLayer.html" class="struct" title="struct opendal::layers::CapabilityCheckLayer">CapabilityCheckLayer</a>  
Add an extra capability check layer for every operation

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html" class="struct" title="struct opendal::layers::ChaosLayer">ChaosLayer</a>`layers-chaos`  
Inject chaos into underlying services for robustness test.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html" class="struct" title="struct opendal::layers::ConcurrentLimitLayer">ConcurrentLimitLayer</a>  
Add concurrent request limit.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html" class="struct" title="struct opendal::layers::DtraceLayer">DtraceLayer</a>Linux and `layers-dtrace`  
Support User Statically-Defined Tracing(aka USDT) on Linux

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html" class="struct" title="struct opendal::layers::FastmetricsLayer">FastmetricsLayer</a>`layers-fastmetrics`  
Add [fastmetrics](https://docs.rs/fastmetrics/) for every operation.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html" class="struct" title="struct opendal::layers::FastmetricsLayerBuilder">FastmetricsLayerBuilder</a>`layers-fastmetrics`  
[`FastmetricsLayerBuilder`](https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html "struct opendal::layers::FastmetricsLayerBuilder") is a config builder to build a [`FastmetricsLayer`](https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html "struct opendal::layers::FastmetricsLayer").

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastraceLayer.html" class="struct" title="struct opendal::layers::FastraceLayer">FastraceLayer</a>`layers-fastrace`  
Add [fastrace](https://docs.rs/fastrace/) for every operation.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html" class="struct" title="struct opendal::layers::HttpClientLayer">HttpClientLayer</a>  
Layer for replacing the default HTTP client with a custom one.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html" class="struct" title="struct opendal::layers::ImmutableIndexLayer">ImmutableIndexLayer</a>  
Add an immutable in-memory index for underlying storage services.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html" class="struct" title="struct opendal::layers::LoggingLayer">LoggingLayer</a>  
Add [log](https://docs.rs/log/) for every operation.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html" class="struct" title="struct opendal::layers::MetricsLayer">MetricsLayer</a>`layers-metrics`  
Add [metrics](https://docs.rs/metrics/) for every operation.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html" class="struct" title="struct opendal::layers::MimeGuessLayer">MimeGuessLayer</a>`layers-mime-guess`  
A layer that can automatically set `Content-Type` based on the file extension in the path.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html" class="struct" title="struct opendal::layers::OtelMetricsLayer">OtelMetricsLayer</a>`layers-otel-metrics`  
Add [opentelemetry::metrics](https://docs.rs/opentelemetry/latest/opentelemetry/metrics/index.html) for every operation.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelTraceLayer.html" class="struct" title="struct opendal::layers::OtelTraceLayer">OtelTraceLayer</a>`layers-otel-trace`  
Add [opentelemetry::trace](https://docs.rs/opentelemetry/latest/opentelemetry/trace/index.html) for every operation.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html" class="struct" title="struct opendal::layers::PrometheusClientLayer">PrometheusClientLayer</a>`layers-prometheus-client`  
Add [prometheus-client](https://docs.rs/prometheus-client) for every operation.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html" class="struct" title="struct opendal::layers::PrometheusClientLayerBuilder">PrometheusClientLayerBuilder</a>`layers-prometheus-client`  
[`PrometheusClientLayerBuilder`](https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html "struct opendal::layers::PrometheusClientLayerBuilder") is a config builder to build a [`PrometheusClientLayer`](https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html "struct opendal::layers::PrometheusClientLayer").

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html" class="struct" title="struct opendal::layers::PrometheusLayer">PrometheusLayer</a>`layers-prometheus`  
Add [prometheus](https://docs.rs/prometheus) for every operation.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html" class="struct" title="struct opendal::layers::PrometheusLayerBuilder">PrometheusLayerBuilder</a>`layers-prometheus`  
[`PrometheusLayerBuilder`](https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html "struct opendal::layers::PrometheusLayerBuilder") is a config builder to build a [`PrometheusLayer`](https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html "struct opendal::layers::PrometheusLayer").

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.RetryLayer.html" class="struct" title="struct opendal::layers::RetryLayer">RetryLayer</a>  
Add retry for temporary failed operations.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html" class="struct" title="struct opendal::layers::TailCutLayer">TailCutLayer</a>  
Layer that automatically cancels long-tail requests.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html" class="struct" title="struct opendal::layers::TailCutLayerBuilder">TailCutLayerBuilder</a>  
Builder for TailCutLayer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html" class="struct" title="struct opendal::layers::ThrottleLayer">ThrottleLayer</a>`layers-throttle`  
Add a bandwidth rate limiter to the underlying services.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html" class="struct" title="struct opendal::layers::TimeoutLayer">TimeoutLayer</a>  
Add timeout for every operation to avoid slow or unexpected hang operations.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TracingLayer.html" class="struct" title="struct opendal::layers::TracingLayer">TracingLayer</a>`layers-tracing`  
Add [tracing](https://docs.rs/tracing/) for every operation.

## Traits<a href="https://opendal.apache.org/docs/rust/opendal/layers/index.html#traits" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.LoggingInterceptor.html" class="trait" title="trait opendal::layers::LoggingInterceptor">LoggingInterceptor</a>  
LoggingInterceptor is used to intercept the log.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.RetryInterceptor.html" class="trait" title="trait opendal::layers::RetryInterceptor">RetryInterceptor</a>  
RetryInterceptor is used to intercept while retry happened.
