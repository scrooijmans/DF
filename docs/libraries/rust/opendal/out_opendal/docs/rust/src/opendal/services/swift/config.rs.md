# 

opendal/services/swift/

config.rs

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
18use std::fmt::Debug;
19use std::fmt::Formatter;
20
21use super::backend::SwiftBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for OpenStack Swift support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct SwiftConfig {
30    /// The endpoint for Swift.
31    pub endpoint: Option<String>,
32    /// The container for Swift.
33    pub container: Option<String>,
34    /// The root for Swift.
35    pub root: Option<String>,
36    /// The token for Swift.
37    pub token: Option<String>,
38}
39
40impl Debug for SwiftConfig {
41    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
42        let mut ds = f.debug_struct("SwiftConfig");
43
44        ds.field("root", &self.root);
45        ds.field("endpoint", &self.endpoint);
46        ds.field("container", &self.container);
47
48        if self.token.is_some() {
49            ds.field("token", &"<redacted>");
50        }
51
52        ds.finish()
53    }
54}
55
56impl crate::Configurator for SwiftConfig {
57    type Builder = SwiftBuilder;
58
59    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
60        let mut map = uri.options().clone();
61
62        if let Some(authority) = uri.authority() {
63            map.entry("endpoint".to_string())
64                .or_insert_with(|| format!("https://{authority}"));
65        } else if !map.contains_key("endpoint") {
66            return Err(
67                crate::Error::new(crate::ErrorKind::ConfigInvalid, "endpoint is required")
68                    .with_context("service", crate::Scheme::Swift),
69            );
70        }
71
72        if let Some(path) = uri.root() {
73            if let Some((container, rest)) = path.split_once('/') {
74                if !container.is_empty() {
75                    map.insert("container".to_string(), container.to_string());
76                }
77                if !rest.is_empty() {
78                    map.insert("root".to_string(), rest.to_string());
79                }
80            } else if !path.is_empty() {
81                map.insert("container".to_string(), path.to_string());
82            }
83        }
84
85        if !map.contains_key("container") {
86            return Err(crate::Error::new(
87                crate::ErrorKind::ConfigInvalid,
88                "container is required",
89            )
90            .with_context("service", crate::Scheme::Swift));
91        }
92
93        Self::from_iter(map)
94    }
95
96    fn into_builder(self) -> Self::Builder {
97        SwiftBuilder { config: self }
98    }
99}
100
101#[cfg(test)]
102mod tests {
103    use super::*;
104    use crate::Configurator;
105    use crate::types::OperatorUri;
106
107    #[test]
108    fn from_uri_sets_endpoint_container_and_root() {
109        let uri = OperatorUri::new(
110            "swift://swift.example.com/container/assets/images",
111            Vec::<(String, String)>::new(),
112        )
113        .unwrap();
114
115        let cfg = SwiftConfig::from_uri(&uri).unwrap();
116        assert_eq!(cfg.endpoint.as_deref(), Some("https://swift.example.com"));
117        assert_eq!(cfg.container.as_deref(), Some("container"));
118        assert_eq!(cfg.root.as_deref(), Some("assets/images"));
119    }
120
121    #[test]
122    fn from_uri_accepts_container_from_query() {
123        let uri = OperatorUri::new(
124            "swift://swift.example.com",
125            vec![("container".to_string(), "logs".to_string())],
126        )
127        .unwrap();
128
129        let cfg = SwiftConfig::from_uri(&uri).unwrap();
130        assert_eq!(cfg.container.as_deref(), Some("logs"));
131        assert_eq!(cfg.endpoint.as_deref(), Some("https://swift.example.com"));
132    }
133}
```
