# 

opendal/services/cos/

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
21use super::backend::CosBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Tencent-Cloud COS services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28pub struct CosConfig {
29    /// Root of this backend.
30    pub root: Option<String>,
31    /// Endpoint of this backend.
32    pub endpoint: Option<String>,
33    /// Secret ID of this backend.
34    pub secret_id: Option<String>,
35    /// Secret key of this backend.
36    pub secret_key: Option<String>,
37    /// Bucket of this backend.
38    pub bucket: Option<String>,
39    /// is bucket versioning enabled for this bucket
40    pub enable_versioning: bool,
41    /// Disable config load so that opendal will not load config from
42    pub disable_config_load: bool,
43}
44
45impl Debug for CosConfig {
46    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
47        f.debug_struct("CosConfig")
48            .field("root", &self.root)
49            .field("endpoint", &self.endpoint)
50            .field("secret_id", &"<redacted>")
51            .field("secret_key", &"<redacted>")
52            .field("bucket", &self.bucket)
53            .finish_non_exhaustive()
54    }
55}
56
57impl crate::Configurator for CosConfig {
58    type Builder = CosBuilder;
59
60    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
61        let mut map = uri.options().clone();
62
63        if let Some(name) = uri.name() {
64            map.insert("bucket".to_string(), name.to_string());
65        }
66
67        if let Some(root) = uri.root() {
68            map.insert("root".to_string(), root.to_string());
69        }
70
71        Self::from_iter(map)
72    }
73
74    #[allow(deprecated)]
75    fn into_builder(self) -> Self::Builder {
76        CosBuilder {
77            config: self,
78
79            http_client: None,
80        }
81    }
82}
83
84#[cfg(test)]
85mod tests {
86    use super::*;
87    use crate::Configurator;
88    use crate::types::OperatorUri;
89
90    #[test]
91    fn from_uri_extracts_bucket_and_root() {
92        let uri = OperatorUri::new(
93            "cos://example-bucket/path/to/root",
94            Vec::<(String, String)>::new(),
95        )
96        .unwrap();
97        let cfg = CosConfig::from_uri(&uri).unwrap();
98        assert_eq!(cfg.bucket.as_deref(), Some("example-bucket"));
99        assert_eq!(cfg.root.as_deref(), Some("path/to/root"));
100    }
101}
```
