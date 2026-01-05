# 

opendal/services/lakefs/

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
21use super::backend::LakefsBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Configuration for Lakefs service support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct LakefsConfig {
30    /// Base url.
31    ///
32    /// This is required.
33    pub endpoint: Option<String>,
34    /// Username for Lakefs basic authentication.
35    ///
36    /// This is required.
37    pub username: Option<String>,
38    /// Password for Lakefs basic authentication.
39    ///
40    /// This is required.
41    pub password: Option<String>,
42    /// Root of this backend. Can be "/path/to/dir".
43    ///
44    /// Default is "/".
45    pub root: Option<String>,
46
47    /// The repository name
48    ///
49    /// This is required.
50    pub repository: Option<String>,
51    /// Name of the branch or a commit ID. Default is main.
52    ///
53    /// This is optional.
54    pub branch: Option<String>,
55}
56
57impl Debug for LakefsConfig {
58    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
59        let mut ds = f.debug_struct("LakefsConfig");
60
61        if let Some(endpoint) = &self.endpoint {
62            ds.field("endpoint", &endpoint);
63        }
64        if let Some(_username) = &self.username {
65            ds.field("username", &"<redacted>");
66        }
67        if let Some(_password) = &self.password {
68            ds.field("password", &"<redacted>");
69        }
70        if let Some(root) = &self.root {
71            ds.field("root", &root);
72        }
73        if let Some(repository) = &self.repository {
74            ds.field("repository", &repository);
75        }
76        if let Some(branch) = &self.branch {
77            ds.field("branch", &branch);
78        }
79
80        ds.finish()
81    }
82}
83
84impl crate::Configurator for LakefsConfig {
85    type Builder = LakefsBuilder;
86
87    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
88        let authority = uri.authority().ok_or_else(|| {
89            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
90                .with_context("service", crate::Scheme::Lakefs)
91        })?;
92
93        let raw_path = uri.root().ok_or_else(|| {
94            crate::Error::new(
95                crate::ErrorKind::ConfigInvalid,
96                "uri path must contain repository",
97            )
98            .with_context("service", crate::Scheme::Lakefs)
99        })?;
100
101        let (repository, remainder) = match raw_path.split_once('/') {
102            Some((repo, rest)) => (repo, Some(rest)),
103            None => (raw_path, None),
104        };
105
106        let repository = if repository.is_empty() {
107            None
108        } else {
109            Some(repository)
110        }
111        .ok_or_else(|| {
112            crate::Error::new(
113                crate::ErrorKind::ConfigInvalid,
114                "repository is required in uri path",
115            )
116            .with_context("service", crate::Scheme::Lakefs)
117        })?;
118
119        let mut map = uri.options().clone();
120        map.insert("endpoint".to_string(), format!("https://{authority}"));
121        map.insert("repository".to_string(), repository.to_string());
122
123        if let Some(rest) = remainder {
124            if map.contains_key("branch") {
125                if !rest.is_empty() {
126                    map.insert("root".to_string(), rest.to_string());
127                }
128            } else {
129                let (branch, maybe_root) = match rest.split_once('/') {
130                    Some((branch_part, root_part)) => (branch_part, Some(root_part)),
131                    None => (rest, None),
132                };
133
134                if !branch.is_empty() {
135                    map.insert("branch".to_string(), branch.to_string());
136                }
137
138                if let Some(root_part) = maybe_root {
139                    if !root_part.is_empty() {
140                        map.insert("root".to_string(), root_part.to_string());
141                    }
142                }
143            }
144        }
145
146        Self::from_iter(map)
147    }
148
149    fn into_builder(self) -> Self::Builder {
150        LakefsBuilder { config: self }
151    }
152}
153
154#[cfg(test)]
155mod tests {
156    use super::*;
157    use crate::Configurator;
158    use crate::types::OperatorUri;
159
160    #[test]
161    fn from_uri_sets_endpoint_repository_branch_and_root() {
162        let uri = OperatorUri::new(
163            "lakefs://api.example.com/sample/main/data/dir",
164            Vec::<(String, String)>::new(),
165        )
166        .unwrap();
167
168        let cfg = LakefsConfig::from_uri(&uri).unwrap();
169        assert_eq!(cfg.endpoint.as_deref(), Some("https://api.example.com"));
170        assert_eq!(cfg.repository.as_deref(), Some("sample"));
171        assert_eq!(cfg.branch.as_deref(), Some("main"));
172        assert_eq!(cfg.root.as_deref(), Some("data/dir"));
173    }
174
175    #[test]
176    fn from_uri_requires_repository() {
177        let uri =
178            OperatorUri::new("lakefs://api.example.com", Vec::<(String, String)>::new()).unwrap();
179
180        assert!(LakefsConfig::from_uri(&uri).is_err());
181    }
182
183    #[test]
184    fn from_uri_respects_branch_override_and_sets_root() {
185        let uri = OperatorUri::new(
186            "lakefs://api.example.com/sample/content",
187            vec![("branch".to_string(), "develop".to_string())],
188        )
189        .unwrap();
190
191        let cfg = LakefsConfig::from_uri(&uri).unwrap();
192        assert_eq!(cfg.branch.as_deref(), Some("develop"));
193        assert_eq!(cfg.root.as_deref(), Some("content"));
194    }
195}
```
