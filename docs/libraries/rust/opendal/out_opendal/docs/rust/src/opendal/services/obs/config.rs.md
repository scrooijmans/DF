# 

opendal/services/obs/

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
21use super::backend::ObsBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for Huawei-Cloud Object Storage Service (OBS) support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct ObsConfig {
30    /// Root for obs.
31    pub root: Option<String>,
32    /// Endpoint for obs.
33    pub endpoint: Option<String>,
34    /// Access key id for obs.
35    pub access_key_id: Option<String>,
36    /// Secret access key for obs.
37    pub secret_access_key: Option<String>,
38    /// Bucket for obs.
39    pub bucket: Option<String>,
40    /// Is bucket versioning enabled for this bucket
41    pub enable_versioning: bool,
42}
43
44impl Debug for ObsConfig {
45    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
46        f.debug_struct("ObsConfig")
47            .field("root", &self.root)
48            .field("endpoint", &self.endpoint)
49            .field("access_key_id", &"<redacted>")
50            .field("secret_access_key", &"<redacted>")
51            .field("bucket", &self.bucket)
52            .finish()
53    }
54}
55
56impl crate::Configurator for ObsConfig {
57    type Builder = ObsBuilder;
58
59    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
60        let mut map = uri.options().clone();
61
62        if let Some(name) = uri.name() {
63            map.insert("bucket".to_string(), name.to_string());
64        }
65
66        if let Some(root) = uri.root() {
67            map.insert("root".to_string(), root.to_string());
68        }
69
70        Self::from_iter(map)
71    }
72
73    #[allow(deprecated)]
74    fn into_builder(self) -> Self::Builder {
75        ObsBuilder {
76            config: self,
77
78            http_client: None,
79        }
80    }
81}
82
83#[cfg(test)]
84mod tests {
85    use super::*;
86    use crate::Configurator;
87    use crate::types::OperatorUri;
88
89    #[test]
90    fn from_uri_extracts_bucket_and_root() {
91        let uri = OperatorUri::new(
92            "obs://example-bucket/path/to/root",
93            Vec::<(String, String)>::new(),
94        )
95        .unwrap();
96        let cfg = ObsConfig::from_uri(&uri).unwrap();
97        assert_eq!(cfg.bucket.as_deref(), Some("example-bucket"));
98        assert_eq!(cfg.root.as_deref(), Some("path/to/root"));
99    }
100}
```
