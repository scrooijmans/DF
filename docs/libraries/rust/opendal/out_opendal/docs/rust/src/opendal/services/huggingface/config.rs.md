# 

opendal/services/huggingface/

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
21use super::backend::HuggingfaceBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Configuration for Huggingface service support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct HuggingfaceConfig {
30    /// Repo type of this backend. Default is model.
31    ///
32    /// Available values:
33    /// - model
34    /// - dataset
35    pub repo_type: Option<String>,
36    /// Repo id of this backend.
37    ///
38    /// This is required.
39    pub repo_id: Option<String>,
40    /// Revision of this backend.
41    ///
42    /// Default is main.
43    pub revision: Option<String>,
44    /// Root of this backend. Can be "/path/to/dir".
45    ///
46    /// Default is "/".
47    pub root: Option<String>,
48    /// Token of this backend.
49    ///
50    /// This is optional.
51    pub token: Option<String>,
52}
53
54impl Debug for HuggingfaceConfig {
55    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
56        let mut ds = f.debug_struct("HuggingfaceConfig");
57
58        if let Some(repo_type) = &self.repo_type {
59            ds.field("repo_type", &repo_type);
60        }
61        if let Some(repo_id) = &self.repo_id {
62            ds.field("repo_id", &repo_id);
63        }
64        if let Some(revision) = &self.revision {
65            ds.field("revision", &revision);
66        }
67        if let Some(root) = &self.root {
68            ds.field("root", &root);
69        }
70        if self.token.is_some() {
71            ds.field("token", &"<redacted>");
72        }
73
74        ds.finish()
75    }
76}
77
78impl crate::Configurator for HuggingfaceConfig {
79    type Builder = HuggingfaceBuilder;
80
81    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
82        let mut map = uri.options().clone();
83
84        if let Some(repo_type) = uri.name() {
85            if !repo_type.is_empty() {
86                map.insert("repo_type".to_string(), repo_type.to_string());
87            }
88        }
89
90        let raw_path = uri.root().ok_or_else(|| {
91            crate::Error::new(
92                crate::ErrorKind::ConfigInvalid,
93                "uri path must include owner and repo",
94            )
95            .with_context("service", crate::Scheme::Huggingface)
96        })?;
97
98        let mut segments = raw_path.splitn(4, '/');
99        let owner = segments.next().filter(|s| !s.is_empty()).ok_or_else(|| {
100            crate::Error::new(
101                crate::ErrorKind::ConfigInvalid,
102                "repository owner is required in uri path",
103            )
104            .with_context("service", crate::Scheme::Huggingface)
105        })?;
106        let repo = segments.next().filter(|s| !s.is_empty()).ok_or_else(|| {
107            crate::Error::new(
108                crate::ErrorKind::ConfigInvalid,
109                "repository name is required in uri path",
110            )
111            .with_context("service", crate::Scheme::Huggingface)
112        })?;
113
114        map.insert("repo_id".to_string(), format!("{owner}/{repo}"));
115
116        if let Some(segment) = segments.next() {
117            if map.contains_key("revision") {
118                let mut root_value = segment.to_string();
119                if let Some(rest) = segments.next() {
120                    if !rest.is_empty() {
121                        if !root_value.is_empty() {
122                            root_value.push('/');
123                            root_value.push_str(rest);
124                        } else {
125                            root_value = rest.to_string();
126                        }
127                    }
128                }
129                if !root_value.is_empty() {
130                    map.insert("root".to_string(), root_value);
131                }
132            } else {
133                if !segment.is_empty() {
134                    map.insert("revision".to_string(), segment.to_string());
135                }
136                if let Some(rest) = segments.next() {
137                    if !rest.is_empty() {
138                        map.insert("root".to_string(), rest.to_string());
139                    }
140                }
141            }
142        }
143
144        Self::from_iter(map)
145    }
146
147    fn into_builder(self) -> Self::Builder {
148        HuggingfaceBuilder { config: self }
149    }
150}
151
152#[cfg(test)]
153mod tests {
154    use super::*;
155    use crate::Configurator;
156    use crate::types::OperatorUri;
157
158    #[test]
159    fn from_uri_sets_repo_type_id_and_revision() {
160        let uri = OperatorUri::new(
161            "huggingface://model/opendal/sample/main/dataset",
162            Vec::<(String, String)>::new(),
163        )
164        .unwrap();
165
166        let cfg = HuggingfaceConfig::from_uri(&uri).unwrap();
167        assert_eq!(cfg.repo_type.as_deref(), Some("model"));
168        assert_eq!(cfg.repo_id.as_deref(), Some("opendal/sample"));
169        assert_eq!(cfg.revision.as_deref(), Some("main"));
170        assert_eq!(cfg.root.as_deref(), Some("dataset"));
171    }
172
173    #[test]
174    fn from_uri_uses_existing_revision_and_sets_root() {
175        let uri = OperatorUri::new(
176            "huggingface://dataset/opendal/sample/data/train",
177            vec![("revision".to_string(), "dev".to_string())],
178        )
179        .unwrap();
180
181        let cfg = HuggingfaceConfig::from_uri(&uri).unwrap();
182        assert_eq!(cfg.repo_type.as_deref(), Some("dataset"));
183        assert_eq!(cfg.repo_id.as_deref(), Some("opendal/sample"));
184        assert_eq!(cfg.revision.as_deref(), Some("dev"));
185        assert_eq!(cfg.root.as_deref(), Some("data/train"));
186    }
187
188    #[test]
189    fn from_uri_requires_owner_and_repo() {
190        let uri = OperatorUri::new(
191            "huggingface://model/opendal",
192            Vec::<(String, String)>::new(),
193        )
194        .unwrap();
195
196        assert!(HuggingfaceConfig::from_uri(&uri).is_err());
197    }
198}
```
