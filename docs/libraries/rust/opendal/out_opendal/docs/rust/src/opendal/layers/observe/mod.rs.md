# 

opendal/layers/observe/

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
18//! OpenDAL Observability
19//!
20//! This module offers essential components to facilitate the implementation of observability in OpenDAL.
21//!
22//! # OpenDAL Metrics Reference
23//!
24//! This document describes all metrics exposed by OpenDAL.
25//!
26//! ## Operation Metrics
27//!
28//! These metrics track operations at the storage abstraction level.
29//!
30//! | Metric Name                      | Type      | Description                                                                                | Labels                                          |
31//! |----------------------------------|-----------|--------------------------------------------------------------------------------------------|-------------------------------------------------|
32//! | operation_bytes                  | Histogram | Current operation size in bytes, represents the size of data being processed               | scheme, namespace, root, operation, path        |
33//! | operation_bytes_rate             | Histogram | Histogram of data processing rates in bytes per second within individual operations        | scheme, namespace, root, operation, path        |
34//! | operation_entries                | Histogram | Current operation size in entries, represents the entries being processed                  | scheme, namespace, root, operation, path        |
35//! | operation_entries_rate           | Histogram | Histogram of entries processing rates in entries per second within individual operations   | scheme, namespace, root, operation, path        |
36//! | operation_duration_seconds       | Histogram | Duration of operations in seconds, measured from start to completion                       | scheme, namespace, root, operation, path        |
37//! | operation_errors_total           | Counter   | Total number of failed operations                                                         | scheme, namespace, root, operation, path, error |
38//! | operation_executing              | Gauge     | Number of operations currently being executed                                             | scheme, namespace, root, operation              |
39//! | operation_ttfb_seconds           | Histogram | Time to first byte in seconds for operations                                              | scheme, namespace, root, operation, path        |
40//!
41//! ## HTTP Metrics
42//!
43//! These metrics track the underlying HTTP requests made by OpenDAL services that use HTTP.
44//!
45//! | Metric Name                      | Type      | Description                                                                                | Labels                                          |
46//! |----------------------------------|-----------|--------------------------------------------------------------------------------------------|-------------------------------------------------|
47//! | http_connection_errors_total     | Counter   | Total number of HTTP requests that failed before receiving a response                      | scheme, namespace, root, operation, error       |
48//! | http_status_errors_total         | Counter   | Total number of HTTP requests that received error status codes (non-2xx responses)         | scheme, namespace, root, operation, status      |
49//! | http_executing                   | Gauge     | Number of HTTP requests currently in flight from this client                              | scheme, namespace, root                         |
50//! | http_request_bytes               | Histogram | Histogram of HTTP request body sizes in bytes                                             | scheme, namespace, root, operation              |
51//! | http_request_bytes_rate          | Histogram | Histogram of HTTP request bytes per second rates                                          | scheme, namespace, root, operation              |
52//! | http_request_duration_seconds    | Histogram | Histogram of time spent sending HTTP requests, from first byte sent to first byte received | scheme, namespace, root, operation              |
53//! | http_response_bytes              | Histogram | Histogram of HTTP response body sizes in bytes                                            | scheme, namespace, root, operation              |
54//! | http_response_bytes_rate         | Histogram | Histogram of HTTP response bytes per second rates                                         | scheme, namespace, root, operation              |
55//! | http_response_duration_seconds   | Histogram | Histogram of time spent receiving HTTP responses, from first byte to last byte received   | scheme, namespace, root, operation              |
56//!
57//! ## Label Descriptions
58//!
59//! | Label     | Description                                                   | Example Values                         |
60//! |-----------|---------------------------------------------------------------|----------------------------------------|
61//! | scheme    | The storage service scheme                                    | s3, gcs, azblob, fs, memory            |
62//! | namespace | The storage service namespace (bucket, container, etc.)       | my-bucket, my-container                |
63//! | root      | The root path within the namespace                            | /data, /backup                         |
64//! | operation | The operation being performed                                 | read, write, stat, list, delete        |
65//! | path      | The path of the object being operated on                      | /path/to/file.txt                      |
66//! | error     | The error type or message for error metrics                   | not_found, permission_denied           |
67//! | status    | The HTTP status code for HTTP error metrics                   | 404, 403, 500                          |
68//!
69//! ## Metric Types
70//!
71//! * **Histogram**: Distribution of values with configurable buckets, includes count, sum and quantiles
72//! * **Counter**: Cumulative metric that only increases over time (resets on restart)
73//! * **Gauge**: Point-in-time metric that can increase and decrease
74
75mod metrics;
76
77pub use metrics::DEFAULT_BYTES_BUCKETS;
78pub use metrics::DEFAULT_BYTES_RATE_BUCKETS;
79pub use metrics::DEFAULT_DURATION_SECONDS_BUCKETS;
80pub use metrics::DEFAULT_ENTRIES_BUCKETS;
81pub use metrics::DEFAULT_ENTRIES_RATE_BUCKETS;
82pub use metrics::DEFAULT_TTFB_BUCKETS;
83pub use metrics::LABEL_ERROR;
84pub use metrics::LABEL_NAMESPACE;
85pub use metrics::LABEL_OPERATION;
86pub use metrics::LABEL_ROOT;
87pub use metrics::LABEL_SCHEME;
88pub use metrics::LABEL_STATUS_CODE;
89pub use metrics::MetricLabels;
90pub use metrics::MetricValue;
91pub use metrics::MetricsAccessor;
92pub use metrics::MetricsIntercept;
93pub use metrics::MetricsLayer;
```
