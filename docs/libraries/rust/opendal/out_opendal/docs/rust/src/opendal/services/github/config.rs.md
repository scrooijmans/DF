# 

opendal/services/github/

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
21use super::backend::GithubBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for GitHub services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct GithubConfig {
30    /// root of this backend.
31    ///
32    /// All operations will happen under this root.
33    pub root: Option<String>,
34    /// GitHub access_token.
35    ///
36    /// optional.
37    /// If not provided, the backend will only support read operations for public repositories.
38    /// And rate limit will be limited to 60 requests per hour.
39    pub token: Option<String>,
40    /// GitHub repo owner.
41    ///
42    /// required.
43    pub owner: String,
44    /// GitHub repo name.
45    ///
46    /// required.
47    pub repo: String,
48}
49
50impl Debug for GithubConfig {
51    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
52        let mut d = f.debug_struct("GithubConfig");
53
54        d.field("root", &self.root)
55            .field("owner", &self.owner)
56            .field("repo", &self.repo);
57
58        d.finish_non_exhaustive()
59    }
60}
61
62impl crate::Configurator for GithubConfig {
63    type Builder = GithubBuilder;
64
65    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
66        let owner = uri.name().ok_or_else(|| {
67            crate::Error::new(
68                crate::ErrorKind::ConfigInvalid,
69                "uri host must contain owner",
70            )
71            .with_context("service", crate::Scheme::Github)
72        })?;
73
74        let raw_path = uri.root().ok_or_else(|| {
75            crate::Error::new(
76                crate::ErrorKind::ConfigInvalid,
77                "uri path must contain repository",
78            )
79            .with_context("service", crate::Scheme::Github)
80        })?;
81
82        let (repo, remainder) = match raw_path.split_once('/') {
83            Some((repo, rest)) => (repo, Some(rest)),
84            None => (raw_path, None),
85        };
86
87        if repo.is_empty() {
88            return Err(crate::Error::new(
89                crate::ErrorKind::ConfigInvalid,
90                "repository name is required",
91            )
92            .with_context("service", crate::Scheme::Github));
93        }
94
95        let mut map = uri.options().clone();
96        map.insert("owner".to_string(), owner.to_string());
97        map.insert("repo".to_string(), repo.to_string());
98
99        if let Some(rest) = remainder {
100            if !rest.is_empty() {
101                map.insert("root".to_string(), rest.to_string());
102            }
103        }
104
105        Self::from_iter(map)
106    }
107
108    #[allow(deprecated)]
109    fn into_builder(self) -> Self::Builder {
110        GithubBuilder {
111            config: self,
112            http_client: None,
113        }
114    }
115}
116
117#[cfg(test)]
118mod tests {
119    use super::*;
120    use crate::Configurator;
121    use crate::types::OperatorUri;
122
123    #[test]
124    fn from_uri_sets_owner_repo_and_root() {
125        let uri = OperatorUri::new(
126            "github://apache/opendal/src/services",
127            Vec::<(String, String)>::new(),
128        )
129        .unwrap();
130
131        let cfg = GithubConfig::from_uri(&uri).unwrap();
132        assert_eq!(cfg.owner, "apache".to_string());
133        assert_eq!(cfg.repo, "opendal".to_string());
134        assert_eq!(cfg.root.as_deref(), Some("src/services"));
135    }
136
137    #[test]
138    fn from_uri_requires_repository() {
139        let uri = OperatorUri::new("github://apache", Vec::<(String, String)>::new()).unwrap();
140
141        assert!(GithubConfig::from_uri(&uri).is_err());
142    }
143}
```
