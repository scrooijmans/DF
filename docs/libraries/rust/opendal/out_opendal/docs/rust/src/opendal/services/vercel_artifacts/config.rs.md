# 

opendal/services/vercel_artifacts/

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
21use super::builder::VercelArtifactsBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for Vercel Cache support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct VercelArtifactsConfig {
30    /// The access token for Vercel.
31    pub access_token: Option<String>,
32}
33
34impl Debug for VercelArtifactsConfig {
35    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
36        f.debug_struct("VercelArtifactsConfig")
37            .field("access_token", &"<redacted>")
38            .finish()
39    }
40}
41
42impl crate::Configurator for VercelArtifactsConfig {
43    type Builder = VercelArtifactsBuilder;
44
45    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
46        Self::from_iter(uri.options().clone())
47    }
48
49    #[allow(deprecated)]
50    fn into_builder(self) -> Self::Builder {
51        VercelArtifactsBuilder {
52            config: self,
53            http_client: None,
54        }
55    }
56}
57
58#[cfg(test)]
59mod tests {
60    use super::*;
61    use crate::Configurator;
62    use crate::types::OperatorUri;
63
64    #[test]
65    fn from_uri_loads_access_token() {
66        let uri = OperatorUri::new(
67            "vercel-artifacts://cache",
68            vec![("access_token".to_string(), "token123".to_string())],
69        )
70        .unwrap();
71
72        let cfg = VercelArtifactsConfig::from_uri(&uri).unwrap();
73        assert_eq!(cfg.access_token.as_deref(), Some("token123"));
74    }
75}
```
