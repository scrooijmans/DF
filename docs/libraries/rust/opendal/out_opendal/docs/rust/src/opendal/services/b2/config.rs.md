# 

opendal/services/b2/

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
21use super::backend::B2Builder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for backblaze b2 services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct B2Config {
30    /// root of this backend.
31    ///
32    /// All operations will happen under this root.
33    pub root: Option<String>,
34    /// keyID of this backend.
35    ///
36    /// - If application_key_id is set, we will take user's input first.
37    /// - If not, we will try to load it from environment.
38    pub application_key_id: Option<String>,
39    /// applicationKey of this backend.
40    ///
41    /// - If application_key is set, we will take user's input first.
42    /// - If not, we will try to load it from environment.
43    pub application_key: Option<String>,
44    /// bucket of this backend.
45    ///
46    /// required.
47    pub bucket: String,
48    /// bucket id of this backend.
49    ///
50    /// required.
51    pub bucket_id: String,
52}
53
54impl Debug for B2Config {
55    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
56        let mut d = f.debug_struct("B2Config");
57
58        d.field("root", &self.root)
59            .field("application_key_id", &self.application_key_id)
60            .field("bucket_id", &self.bucket_id)
61            .field("bucket", &self.bucket);
62
63        d.finish_non_exhaustive()
64    }
65}
66
67impl crate::Configurator for B2Config {
68    type Builder = B2Builder;
69
70    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
71        let mut map = uri.options().clone();
72
73        if let Some(name) = uri.name() {
74            map.insert("bucket".to_string(), name.to_string());
75        }
76
77        if let Some(root) = uri.root() {
78            map.insert("root".to_string(), root.to_string());
79        }
80
81        Self::from_iter(map)
82    }
83
84    #[allow(deprecated)]
85    fn into_builder(self) -> Self::Builder {
86        B2Builder {
87            config: self,
88            http_client: None,
89        }
90    }
91}
92
93#[cfg(test)]
94mod tests {
95    use super::*;
96    use crate::Configurator;
97    use crate::types::OperatorUri;
98
99    #[test]
100    fn from_uri_extracts_bucket_and_root() {
101        let uri = OperatorUri::new(
102            "b2://example-bucket/path/to/root",
103            vec![("bucket_id".to_string(), "bucket-id".to_string())],
104        )
105        .unwrap();
106
107        let cfg = B2Config::from_uri(&uri).unwrap();
108        assert_eq!(cfg.bucket, "example-bucket");
109        assert_eq!(cfg.root.as_deref(), Some("path/to/root"));
110        assert_eq!(cfg.bucket_id, "bucket-id");
111    }
112}
```
