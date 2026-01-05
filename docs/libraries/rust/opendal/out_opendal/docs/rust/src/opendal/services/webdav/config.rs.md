# 

opendal/services/webdav/

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
21use super::backend::WebdavBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for [WebDAV](https://datatracker.ietf.org/doc/html/rfc4918) backend support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct WebdavConfig {
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
40    /// WebDAV Service doesn't support copy.
41    pub disable_copy: bool,
42}
43
44impl Debug for WebdavConfig {
45    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
46        let mut d = f.debug_struct("WebdavConfig");
47
48        d.field("endpoint", &self.endpoint)
49            .field("username", &self.username)
50            .field("root", &self.root);
51
52        d.finish_non_exhaustive()
53    }
54}
55
56impl crate::Configurator for WebdavConfig {
57    type Builder = WebdavBuilder;
58
59    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
60        let authority = uri.authority().ok_or_else(|| {
61            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
62                .with_context("service", crate::Scheme::Webdav)
63        })?;
64
65        let mut map = uri.options().clone();
66        map.insert("endpoint".to_string(), format!("https://{authority}"));
67
68        if let Some(root) = uri.root() {
69            map.insert("root".to_string(), root.to_string());
70        }
71
72        Self::from_iter(map)
73    }
74
75    #[allow(deprecated)]
76    fn into_builder(self) -> Self::Builder {
77        WebdavBuilder {
78            config: self,
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
91    fn from_uri_sets_endpoint_and_root() {
92        let uri = OperatorUri::new(
93            "webdav://webdav.example.com/remote.php/webdav",
94            Vec::<(String, String)>::new(),
95        )
96        .unwrap();
97
98        let cfg = WebdavConfig::from_uri(&uri).unwrap();
99        assert_eq!(cfg.endpoint.as_deref(), Some("https://webdav.example.com"));
100        assert_eq!(cfg.root.as_deref(), Some("remote.php/webdav"));
101    }
102
103    #[test]
104    fn from_uri_ignores_endpoint_override() {
105        let uri = OperatorUri::new(
106            "webdav://dav.internal/data",
107            vec![(
108                "endpoint".to_string(),
109                "http://dav.internal:8080".to_string(),
110            )],
111        )
112        .unwrap();
113
114        let cfg = WebdavConfig::from_uri(&uri).unwrap();
115        assert_eq!(cfg.endpoint.as_deref(), Some("https://dav.internal"));
116    }
117
118    #[test]
119    fn from_uri_propagates_disable_copy() {
120        let uri = OperatorUri::new(
121            "webdav://dav.example.com",
122            vec![("disable_copy".to_string(), "true".to_string())],
123        )
124        .unwrap();
125
126        let cfg = WebdavConfig::from_uri(&uri).unwrap();
127        assert!(cfg.disable_copy);
128    }
129}
```
