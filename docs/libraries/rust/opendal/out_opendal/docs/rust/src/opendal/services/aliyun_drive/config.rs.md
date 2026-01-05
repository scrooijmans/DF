# 

opendal/services/aliyun_drive/

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
21use super::backend::AliyunDriveBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for Aliyun Drive services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct AliyunDriveConfig {
30    /// The Root of this backend.
31    ///
32    /// All operations will happen under this root.
33    ///
34    /// Default to `/` if not set.
35    pub root: Option<String>,
36    /// The access_token of this backend.
37    ///
38    /// Solution for client-only purpose. #4733
39    ///
40    /// Required if no client_id, client_secret and refresh_token are provided.
41    pub access_token: Option<String>,
42    /// The client_id of this backend.
43    ///
44    /// Required if no access_token is provided.
45    pub client_id: Option<String>,
46    /// The client_secret of this backend.
47    ///
48    /// Required if no access_token is provided.
49    pub client_secret: Option<String>,
50    /// The refresh_token of this backend.
51    ///
52    /// Required if no access_token is provided.
53    pub refresh_token: Option<String>,
54    /// The drive_type of this backend.
55    ///
56    /// All operations will happen under this type of drive.
57    ///
58    /// Available values are `default`, `backup` and `resource`.
59    ///
60    /// Fallback to default if not set or no other drives can be found.
61    pub drive_type: String,
62}
63
64impl Debug for AliyunDriveConfig {
65    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
66        let mut d = f.debug_struct("AliyunDriveConfig");
67
68        d.field("root", &self.root)
69            .field("drive_type", &self.drive_type);
70
71        d.finish_non_exhaustive()
72    }
73}
74
75impl crate::Configurator for AliyunDriveConfig {
76    type Builder = AliyunDriveBuilder;
77
78    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
79        let mut map = uri.options().clone();
80
81        if let Some(drive_type) = uri.name() {
82            if !drive_type.is_empty() {
83                map.insert("drive_type".to_string(), drive_type.to_string());
84            }
85        }
86
87        if let Some(root) = uri.root() {
88            if !root.is_empty() {
89                map.insert("root".to_string(), root.to_string());
90            }
91        }
92
93        Self::from_iter(map)
94    }
95
96    #[allow(deprecated)]
97    fn into_builder(self) -> Self::Builder {
98        AliyunDriveBuilder {
99            config: self,
100            http_client: None,
101        }
102    }
103}
104
105#[cfg(test)]
106mod tests {
107    use super::*;
108    use crate::Configurator;
109    use crate::types::OperatorUri;
110
111    #[test]
112    fn from_uri_sets_drive_type_and_root() {
113        let uri = OperatorUri::new(
114            "aliyun-drive://resource/library/photos",
115            Vec::<(String, String)>::new(),
116        )
117        .unwrap();
118
119        let cfg = AliyunDriveConfig::from_uri(&uri).unwrap();
120        assert_eq!(cfg.drive_type, "resource".to_string());
121        assert_eq!(cfg.root.as_deref(), Some("library/photos"));
122    }
123
124    #[test]
125    fn from_uri_allows_missing_drive_type() {
126        let uri =
127            OperatorUri::new("aliyun-drive:///documents", Vec::<(String, String)>::new()).unwrap();
128
129        let cfg = AliyunDriveConfig::from_uri(&uri).unwrap();
130        assert_eq!(cfg.drive_type, String::default());
131        assert_eq!(cfg.root.as_deref(), Some("documents"));
132    }
133}
```
