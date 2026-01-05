# 

opendal/services/seafile/

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
21use super::backend::SeafileBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for seafile services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct SeafileConfig {
30    /// root of this backend.
31    ///
32    /// All operations will happen under this root.
33    pub root: Option<String>,
34    /// endpoint address of this backend.
35    pub endpoint: Option<String>,
36    /// username of this backend.
37    pub username: Option<String>,
38    /// password of this backend.
39    pub password: Option<String>,
40    /// repo_name of this backend.
41    ///
42    /// required.
43    pub repo_name: String,
44}
45
46impl Debug for SeafileConfig {
47    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
48        let mut d = f.debug_struct("SeafileConfig");
49
50        d.field("root", &self.root)
51            .field("endpoint", &self.endpoint)
52            .field("username", &self.username)
53            .field("repo_name", &self.repo_name);
54
55        d.finish_non_exhaustive()
56    }
57}
58
59impl crate::Configurator for SeafileConfig {
60    type Builder = SeafileBuilder;
61
62    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
63        let authority = uri.authority().ok_or_else(|| {
64            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
65                .with_context("service", crate::Scheme::Seafile)
66        })?;
67
68        let raw_path = uri.root().ok_or_else(|| {
69            crate::Error::new(
70                crate::ErrorKind::ConfigInvalid,
71                "uri path must start with repo name",
72            )
73            .with_context("service", crate::Scheme::Seafile)
74        })?;
75
76        let mut segments = raw_path.splitn(2, '/');
77        let repo_name = segments.next().filter(|s| !s.is_empty()).ok_or_else(|| {
78            crate::Error::new(
79                crate::ErrorKind::ConfigInvalid,
80                "repo name is required in uri path",
81            )
82            .with_context("service", crate::Scheme::Seafile)
83        })?;
84
85        let mut map = uri.options().clone();
86        map.insert("endpoint".to_string(), format!("https://{authority}"));
87        map.insert("repo_name".to_string(), repo_name.to_string());
88
89        if let Some(rest) = segments.next() {
90            if !rest.is_empty() {
91                map.insert("root".to_string(), rest.to_string());
92            }
93        }
94
95        Self::from_iter(map)
96    }
97
98    #[allow(deprecated)]
99    fn into_builder(self) -> Self::Builder {
100        SeafileBuilder {
101            config: self,
102            http_client: None,
103        }
104    }
105}
106
107#[cfg(test)]
108mod tests {
109    use super::*;
110    use crate::Configurator;
111    use crate::types::OperatorUri;
112
113    #[test]
114    fn from_uri_sets_endpoint_repo_and_root() {
115        let uri = OperatorUri::new(
116            "seafile://files.example.com/myrepo/projects/app",
117            Vec::<(String, String)>::new(),
118        )
119        .unwrap();
120
121        let cfg = SeafileConfig::from_uri(&uri).unwrap();
122        assert_eq!(cfg.endpoint.as_deref(), Some("https://files.example.com"));
123        assert_eq!(cfg.repo_name, "myrepo".to_string());
124        assert_eq!(cfg.root.as_deref(), Some("projects/app"));
125    }
126
127    #[test]
128    fn from_uri_requires_repo_name() {
129        let uri = OperatorUri::new(
130            "seafile://files.example.com",
131            Vec::<(String, String)>::new(),
132        )
133        .unwrap();
134
135        assert!(SeafileConfig::from_uri(&uri).is_err());
136    }
137}
```
