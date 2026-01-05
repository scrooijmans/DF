# 

opendal/services/koofr/

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
21use super::backend::KoofrBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for Koofr services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct KoofrConfig {
30    /// root of this backend.
31    ///
32    /// All operations will happen under this root.
33    pub root: Option<String>,
34    /// Koofr endpoint.
35    pub endpoint: String,
36    /// Koofr email.
37    pub email: String,
38    /// password of this backend. (Must be the application password)
39    pub password: Option<String>,
40}
41
42impl Debug for KoofrConfig {
43    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
44        let mut ds = f.debug_struct("Config");
45
46        ds.field("root", &self.root);
47        ds.field("email", &self.email);
48
49        ds.finish()
50    }
51}
52
53impl crate::Configurator for KoofrConfig {
54    type Builder = KoofrBuilder;
55
56    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
57        let authority = uri.authority().ok_or_else(|| {
58            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
59                .with_context("service", crate::Scheme::Koofr)
60        })?;
61
62        let raw_path = uri.root().ok_or_else(|| {
63            crate::Error::new(
64                crate::ErrorKind::ConfigInvalid,
65                "uri path must contain email",
66            )
67            .with_context("service", crate::Scheme::Koofr)
68        })?;
69
70        let mut segments = raw_path.splitn(2, '/');
71        let email = segments.next().filter(|s| !s.is_empty()).ok_or_else(|| {
72            crate::Error::new(
73                crate::ErrorKind::ConfigInvalid,
74                "email is required in uri path",
75            )
76            .with_context("service", crate::Scheme::Koofr)
77        })?;
78
79        let mut map = uri.options().clone();
80        map.insert("endpoint".to_string(), format!("https://{authority}"));
81        map.insert("email".to_string(), email.to_string());
82
83        if let Some(rest) = segments.next() {
84            if !rest.is_empty() {
85                map.insert("root".to_string(), rest.to_string());
86            }
87        }
88
89        Self::from_iter(map)
90    }
91
92    #[allow(deprecated)]
93    fn into_builder(self) -> Self::Builder {
94        KoofrBuilder {
95            config: self,
96            http_client: None,
97        }
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
108    fn from_uri_sets_endpoint_email_and_root() {
109        let uri = OperatorUri::new(
110            "koofr://api.koofr.net/me%40example.com/library",
111            Vec::<(String, String)>::new(),
112        )
113        .unwrap();
114
115        let cfg = KoofrConfig::from_uri(&uri).unwrap();
116        assert_eq!(cfg.endpoint, "https://api.koofr.net".to_string());
117        assert_eq!(cfg.email, "me@example.com".to_string());
118        assert_eq!(cfg.root.as_deref(), Some("library"));
119    }
120
121    #[test]
122    fn from_uri_requires_email_segment() {
123        let uri =
124            OperatorUri::new("koofr://api.koofr.net", Vec::<(String, String)>::new()).unwrap();
125
126        assert!(KoofrConfig::from_uri(&uri).is_err());
127    }
128}
```
