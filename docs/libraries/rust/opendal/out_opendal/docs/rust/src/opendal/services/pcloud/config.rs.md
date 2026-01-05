# 

opendal/services/pcloud/

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
21use super::backend::PcloudBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for Pcloud services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct PcloudConfig {
30    /// root of this backend.
31    ///
32    /// All operations will happen under this root.
33    pub root: Option<String>,
34    ///pCloud  endpoint address.
35    pub endpoint: String,
36    /// pCloud username.
37    pub username: Option<String>,
38    /// pCloud password.
39    pub password: Option<String>,
40}
41
42impl Debug for PcloudConfig {
43    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
44        let mut ds = f.debug_struct("Config");
45
46        ds.field("root", &self.root);
47        ds.field("endpoint", &self.endpoint);
48        ds.field("username", &self.username);
49
50        ds.finish()
51    }
52}
53
54impl crate::Configurator for PcloudConfig {
55    type Builder = PcloudBuilder;
56
57    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
58        let authority = uri.authority().ok_or_else(|| {
59            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
60                .with_context("service", crate::Scheme::Pcloud)
61        })?;
62
63        let mut map = uri.options().clone();
64        map.insert("endpoint".to_string(), format!("https://{authority}"));
65
66        if let Some(root) = uri.root() {
67            if !root.is_empty() {
68                map.insert("root".to_string(), root.to_string());
69            }
70        }
71
72        Self::from_iter(map)
73    }
74
75    #[allow(deprecated)]
76    fn into_builder(self) -> Self::Builder {
77        PcloudBuilder {
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
93            "pcloud://api.pcloud.com/drive/photos",
94            vec![("username".to_string(), "alice".to_string())],
95        )
96        .unwrap();
97
98        let cfg = PcloudConfig::from_uri(&uri).unwrap();
99        assert_eq!(cfg.endpoint, "https://api.pcloud.com".to_string());
100        assert_eq!(cfg.root.as_deref(), Some("drive/photos"));
101        assert_eq!(cfg.username.as_deref(), Some("alice"));
102    }
103
104    #[test]
105    fn from_uri_requires_authority() {
106        let uri = OperatorUri::new("pcloud:///drive", Vec::<(String, String)>::new()).unwrap();
107
108        assert!(PcloudConfig::from_uri(&uri).is_err());
109    }
110}
```
