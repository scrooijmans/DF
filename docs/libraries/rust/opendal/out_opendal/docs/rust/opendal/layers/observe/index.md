# Module observe Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/observe/mod.rs.html#18-93" class="src">Source</a>

Expand description

OpenDAL Observability

This module offers essential components to facilitate the implementation of observability in OpenDAL.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/index.html#opendal-metrics-reference" class="doc-anchor">Â§</a>OpenDAL Metrics Reference

This document describes all metrics exposed by OpenDAL.

### <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/index.html#operation-metrics" class="doc-anchor">Â§</a>Operation Metrics

These metrics track operations at the storage abstraction level.

| Metric Name | Type | Description | Labels |
|----|----|----|----|
| operation_bytes | Histogram | Current operation size in bytes, represents the size of data being processed | scheme, namespace, root, operation, path |
| operation_bytes_rate | Histogram | Histogram of data processing rates in bytes per second within individual operations | scheme, namespace, root, operation, path |
| operation_entries | Histogram | Current operation size in entries, represents the entries being processed | scheme, namespace, root, operation, path |
| operation_entries_rate | Histogram | Histogram of entries processing rates in entries per second within individual operations | scheme, namespace, root, operation, path |
| operation_duration_seconds | Histogram | Duration of operations in seconds, measured from start to completion | scheme, namespace, root, operation, path |
| operation_errors_total | Counter | Total number of failed operations | scheme, namespace, root, operation, path, error |
| operation_executing | Gauge | Number of operations currently being executed | scheme, namespace, root, operation |
| operation_ttfb_seconds | Histogram | Time to first byte in seconds for operations | scheme, namespace, root, operation, path |

### <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/index.html#http-metrics" class="doc-anchor">Â§</a>HTTP Metrics

These metrics track the underlying HTTP requests made by OpenDAL services that use HTTP.

| Metric Name | Type | Description | Labels |
|----|----|----|----|
| http_connection_errors_total | Counter | Total number of HTTP requests that failed before receiving a response | scheme, namespace, root, operation, error |
| http_status_errors_total | Counter | Total number of HTTP requests that received error status codes (non-2xx responses) | scheme, namespace, root, operation, status |
| http_executing | Gauge | Number of HTTP requests currently in flight from this client | scheme, namespace, root |
| http_request_bytes | Histogram | Histogram of HTTP request body sizes in bytes | scheme, namespace, root, operation |
| http_request_bytes_rate | Histogram | Histogram of HTTP request bytes per second rates | scheme, namespace, root, operation |
| http_request_duration_seconds | Histogram | Histogram of time spent sending HTTP requests, from first byte sent to first byte received | scheme, namespace, root, operation |
| http_response_bytes | Histogram | Histogram of HTTP response body sizes in bytes | scheme, namespace, root, operation |
| http_response_bytes_rate | Histogram | Histogram of HTTP response bytes per second rates | scheme, namespace, root, operation |
| http_response_duration_seconds | Histogram | Histogram of time spent receiving HTTP responses, from first byte to last byte received | scheme, namespace, root, operation |

### <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/index.html#label-descriptions" class="doc-anchor">Â§</a>Label Descriptions

| Label | Description | Example Values |
|----|----|----|
| scheme | The storage service scheme | s3, gcs, azblob, fs, memory |
| namespace | The storage service namespace (bucket, container, etc.) | my-bucket, my-container |
| root | The root path within the namespace | /data, /backup |
| operation | The operation being performed | read, write, stat, list, delete |
| path | The path of the object being operated on | /path/to/file.txt |
| error | The error type or message for error metrics | not_found, permission_denied |
| status | The HTTP status code for HTTP error metrics | 404, 403, 500 |

### <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/index.html#metric-types" class="doc-anchor">Â§</a>Metric Types

- **Histogram**: Distribution of values with configurable buckets, includes count, sum and quantiles
- **Counter**: Cumulative metric that only increases over time (resets on restart)
- **Gauge**: Point-in-time metric that can increase and decrease

## Structs<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/index.html#structs" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html" class="struct" title="struct opendal::layers::observe::MetricLabels">MetricLabels</a>  
MetricLabels are the labels for the metrics.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html" class="struct" title="struct opendal::layers::observe::MetricsAccessor">MetricsAccessor</a>  
The metrics accessor for opendal.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html" class="struct" title="struct opendal::layers::observe::MetricsLayer">MetricsLayer</a>  
The metrics layer for opendal.

## Enums<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/index.html#enums" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html" class="enum" title="enum opendal::layers::observe::MetricValue">MetricValue</a>  
MetricValue is the value the opendal sends to the metrics impls.

## Constants<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/index.html#constants" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/constant.DEFAULT_BYTES_BUCKETS.html" class="constant" title="constant opendal::layers::observe::DEFAULT_BYTES_BUCKETS">DEFAULT_BYTES_BUCKETS</a>  
Buckets for data size metrics like OperationBytes Covers typical file and object sizes from small files to large objects

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/constant.DEFAULT_BYTES_RATE_BUCKETS.html" class="constant" title="constant opendal::layers::observe::DEFAULT_BYTES_RATE_BUCKETS">DEFAULT_BYTES_RATE_BUCKETS</a>  
Buckets for data transfer rate metrics like OperationBytesRate

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/constant.DEFAULT_DURATION_SECONDS_BUCKETS.html" class="constant" title="constant opendal::layers::observe::DEFAULT_DURATION_SECONDS_BUCKETS">DEFAULT_DURATION_SECONDS_BUCKETS</a>  
Buckets for operation duration metrics like OperationDurationSeconds Covers timeframes from fast metadata operations to long-running transfers

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/constant.DEFAULT_ENTRIES_BUCKETS.html" class="constant" title="constant opendal::layers::observe::DEFAULT_ENTRIES_BUCKETS">DEFAULT_ENTRIES_BUCKETS</a>  
Buckets for batch operation entry counts (OperationEntriesCount) Covers scenarios from single entry operations to large batch operations

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/constant.DEFAULT_ENTRIES_RATE_BUCKETS.html" class="constant" title="constant opendal::layers::observe::DEFAULT_ENTRIES_RATE_BUCKETS">DEFAULT_ENTRIES_RATE_BUCKETS</a>  
Buckets for batch operation processing rates (OperationEntriesRate) Measures how many entries can be processed per second

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/constant.DEFAULT_TTFB_BUCKETS.html" class="constant" title="constant opendal::layers::observe::DEFAULT_TTFB_BUCKETS">DEFAULT_TTFB_BUCKETS</a>  
Buckets for time to first byte metrics like OperationTtfbSeconds Focuses on initial response times, which are typically shorter than full operations

## Statics<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/index.html#statics" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/static.LABEL_ERROR.html" class="static" title="static opendal::layers::observe::LABEL_ERROR">LABEL_ERROR</a>  
The metric label for the error.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/static.LABEL_NAMESPACE.html" class="static" title="static opendal::layers::observe::LABEL_NAMESPACE">LABEL_NAMESPACE</a>  
The metric label for the namespace like bucket name in s3.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/static.LABEL_OPERATION.html" class="static" title="static opendal::layers::observe::LABEL_OPERATION">LABEL_OPERATION</a>  
The metric label for the operation like read, write, list.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/static.LABEL_ROOT.html" class="static" title="static opendal::layers::observe::LABEL_ROOT">LABEL_ROOT</a>  
The metric label for the root path.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/static.LABEL_SCHEME.html" class="static" title="static opendal::layers::observe::LABEL_SCHEME">LABEL_SCHEME</a>  
The metric label for the scheme like s3, fs, cos.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/static.LABEL_STATUS_CODE.html" class="static" title="static opendal::layers::observe::LABEL_STATUS_CODE">LABEL_STATUS_CODE</a>  
The metric label for the http code.

## Traits<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/index.html#traits" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/trait.MetricsIntercept.html" class="trait" title="trait opendal::layers::observe::MetricsIntercept">MetricsIntercept</a>  
The interceptor for metrics.
