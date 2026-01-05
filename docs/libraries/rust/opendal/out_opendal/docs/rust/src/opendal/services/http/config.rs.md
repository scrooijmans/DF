# 

opendal/services/http/

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
21use super::backend::HttpBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for Http service support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct HttpConfig {
30    /// endpoint of this backend
31    pub endpoint: Option<String>,
32    /// username of this backend
33    pub username: Option<String>,
34    /// password of this backend
35    pub password: Option<String>,
36    /// token of this backend
37    pub token: Option<String>,
38    /// root of this backend
39    pub root: Option<String>,
40}
41
42impl Debug for HttpConfig {
43    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
44        let mut de = f.debug_struct("HttpConfig");
45        de.field("endpoint", &self.endpoint);
46        de.field("root", &self.root);
47
48        de.finish_non_exhaustive()
49    }
50}
51
52impl crate::Configurator for HttpConfig {
53    type Builder = HttpBuilder;
54
55    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
56        let authority = uri.authority().ok_or_else(|| {
57            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
58                .with_context("service", crate::Scheme::Http)
59        })?;
60
61        let mut map = uri.options().clone();
62        map.insert(
63            "endpoint".to_string(),
64            format!("{}://{}", uri.scheme(), authority),
65        );
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
76        HttpBuilder {
77            config: self,
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
90    fn from_uri_sets_endpoint_and_root() {
91        let uri = OperatorUri::new(
92            "http://example.com/static/assets",
93            Vec::<(String, String)>::new(),
94        )
95        .unwrap();
96
97        let cfg = HttpConfig::from_uri(&uri).unwrap();
98        assert_eq!(cfg.endpoint.as_deref(), Some("http://example.com"));
99        assert_eq!(cfg.root.as_deref(), Some("static/assets"));
100    }
101
102    #[test]
103    fn from_uri_preserves_query_options() {
104        let uri = OperatorUri::new(
105            "http://cdn.example.com/data?token=abc123",
106            Vec::<(String, String)>::new(),
107        )
108        .unwrap();
109        let cfg = HttpConfig::from_uri(&uri).unwrap();
110
111        assert_eq!(cfg.endpoint.as_deref(), Some("http://cdn.example.com"));
112        assert_eq!(cfg.token.as_deref(), Some("abc123"));
113    }
114
115    #[test]
116    fn from_uri_ignores_endpoint_override() {
117        let uri = OperatorUri::new(
118            "http://example.com/data",
119            vec![(
120                "endpoint".to_string(),
121                "https://cdn.example.com".to_string(),
122            )],
123        )
124        .unwrap();
125        let cfg = HttpConfig::from_uri(&uri).unwrap();
126
127        assert_eq!(cfg.endpoint.as_deref(), Some("http://example.com"));
128    }
129}
```
