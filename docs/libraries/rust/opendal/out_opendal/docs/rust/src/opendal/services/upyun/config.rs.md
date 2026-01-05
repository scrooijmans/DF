# 

opendal/services/upyun/

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
21use super::backend::UpyunBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for upyun services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct UpyunConfig {
30    /// root of this backend.
31    ///
32    /// All operations will happen under this root.
33    pub root: Option<String>,
34    /// bucket address of this backend.
35    pub bucket: String,
36    /// username of this backend.
37    pub operator: Option<String>,
38    /// password of this backend.
39    pub password: Option<String>,
40}
41
42impl Debug for UpyunConfig {
43    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
44        let mut ds = f.debug_struct("Config");
45
46        ds.field("root", &self.root);
47        ds.field("bucket", &self.bucket);
48        ds.field("operator", &self.operator);
49
50        ds.finish()
51    }
52}
53
54impl crate::Configurator for UpyunConfig {
55    type Builder = UpyunBuilder;
56
57    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
58        let mut map = uri.options().clone();
59
60        if let Some(name) = uri.name() {
61            map.insert("bucket".to_string(), name.to_string());
62        }
63
64        if let Some(root) = uri.root() {
65            map.insert("root".to_string(), root.to_string());
66        }
67
68        Self::from_iter(map)
69    }
70
71    #[allow(deprecated)]
72    fn into_builder(self) -> Self::Builder {
73        UpyunBuilder {
74            config: self,
75            http_client: None,
76        }
77    }
78}
79
80#[cfg(test)]
81mod tests {
82    use super::*;
83    use crate::Configurator;
84    use crate::types::OperatorUri;
85
86    #[test]
87    fn from_uri_extracts_bucket_and_root() {
88        let uri = OperatorUri::new(
89            "upyun://example-bucket/path/to/root",
90            Vec::<(String, String)>::new(),
91        )
92        .unwrap();
93        let cfg = UpyunConfig::from_uri(&uri).unwrap();
94        assert_eq!(cfg.bucket, "example-bucket");
95        assert_eq!(cfg.root.as_deref(), Some("path/to/root"));
96    }
97}
```
