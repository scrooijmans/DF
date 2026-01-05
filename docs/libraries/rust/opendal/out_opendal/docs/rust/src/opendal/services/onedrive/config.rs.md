# 

opendal/services/onedrive/

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
21use super::builder::OnedriveBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for [OneDrive](https://onedrive.com) backend support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct OnedriveConfig {
30    /// The root path for the OneDrive service for the file access
31    pub root: Option<String>,
32    /// Microsoft Graph API (also OneDrive API) access token
33    pub access_token: Option<String>,
34    ///  Microsoft Graph API (also OneDrive API) refresh token
35    pub refresh_token: Option<String>,
36    /// Microsoft Graph API Application (client) ID that is in the Azure's app registration portal
37    pub client_id: Option<String>,
38    /// Microsoft Graph API Application client secret that is in the Azure's app registration portal
39    pub client_secret: Option<String>,
40    /// Enabling version support
41    pub enable_versioning: bool,
42}
43
44impl Debug for OnedriveConfig {
45    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
46        f.debug_struct("OnedriveConfig")
47            .field("root", &self.root)
48            .finish_non_exhaustive()
49    }
50}
51
52impl crate::Configurator for OnedriveConfig {
53    type Builder = OnedriveBuilder;
54
55    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
56        let mut map = uri.options().clone();
57
58        if let Some(root) = uri.root() {
59            if !root.is_empty() {
60                let normalized = match root.split_once('/') {
61                    Some((_, rest)) if !rest.is_empty() => rest.to_string(),
62                    _ => root.to_string(),
63                };
64                if !normalized.is_empty() {
65                    map.insert("root".to_string(), normalized);
66                }
67            }
68        }
69
70        Self::from_iter(map)
71    }
72
73    fn into_builder(self) -> Self::Builder {
74        OnedriveBuilder {
75            config: self,
76            http_client: None,
77        }
78    }
79}
80
81#[cfg(test)]
82mod tests {
83    use super::*;
84    use crate::Configurator;
85    use crate::types::OperatorUri;
86
87    #[test]
88    fn from_uri_sets_root() {
89        let uri = OperatorUri::new(
90            "onedrive://drive/root/documents",
91            Vec::<(String, String)>::new(),
92        )
93        .unwrap();
94
95        let cfg = OnedriveConfig::from_uri(&uri).unwrap();
96        assert_eq!(cfg.root.as_deref(), Some("documents"));
97    }
98}
```
