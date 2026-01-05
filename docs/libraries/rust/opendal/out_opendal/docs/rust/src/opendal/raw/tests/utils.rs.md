# 

opendal/raw/tests/

utils.rs

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
18use std::collections::HashMap;
19use std::env;
20use std::str::FromStr;
21use std::sync::LazyLock;
22
23use crate::*;
24
25/// TEST_RUNTIME is the runtime used for running tests.
26pub static TEST_RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
27    tokio::runtime::Builder::new_multi_thread()
28        .enable_all()
29        .build()
30        .unwrap()
31});
32
33/// Init a service with given scheme.
34///
35/// - Load scheme from `OPENDAL_TEST`
36/// - Construct a new Operator with given root.
37/// - Else, returns a `None` to represent no valid config for operator.
38pub fn init_test_service() -> Result<Option<Operator>> {
39    let _ = dotenvy::dotenv();
40
41    let scheme = if let Ok(v) = env::var("OPENDAL_TEST") {
42        v
43    } else {
44        return Ok(None);
45    };
46    let scheme = Scheme::from_str(&scheme).unwrap();
47
48    let scheme_key = String::from(scheme).replace('-', "_");
49    let prefix = format!("opendal_{scheme_key}_");
50
51    let mut cfg = env::vars()
52        .filter_map(|(k, v)| {
53            k.to_lowercase()
54                .strip_prefix(&prefix)
55                .map(|k| (k.to_string(), v))
56        })
57        .collect::<HashMap<String, String>>();
58
59    // Use random root unless OPENDAL_DISABLE_RANDOM_ROOT is set to true.
60    let disable_random_root = env::var("OPENDAL_DISABLE_RANDOM_ROOT").unwrap_or_default() == "true";
61    if !disable_random_root {
62        let root = format!(
63            "{}{}/",
64            cfg.get("root").cloned().unwrap_or_else(|| "/".to_string()),
65            uuid::Uuid::new_v4()
66        );
67        cfg.insert("root".to_string(), root);
68    }
69
70    let op = Operator::via_iter(scheme, cfg).expect("must succeed");
71
72    #[cfg(feature = "layers-chaos")]
73    let op = { op.layer(layers::ChaosLayer::new(0.1)) };
74
75    let op = op
76        .layer(layers::LoggingLayer::default())
77        .layer(layers::TimeoutLayer::new())
78        .layer(layers::RetryLayer::new().with_max_times(4));
79
80    Ok(Some(op))
81}
```
