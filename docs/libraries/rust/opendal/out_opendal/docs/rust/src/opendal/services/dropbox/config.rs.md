# 

opendal/services/dropbox/

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
21use super::builder::DropboxBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for [Dropbox](https://www.dropbox.com/) backend support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct DropboxConfig {
30    /// root path for dropbox.
31    pub root: Option<String>,
32    /// access token for dropbox.
33    pub access_token: Option<String>,
34    /// refresh_token for dropbox.
35    pub refresh_token: Option<String>,
36    /// client_id for dropbox.
37    pub client_id: Option<String>,
38    /// client_secret for dropbox.
39    pub client_secret: Option<String>,
40}
41
42impl Debug for DropboxConfig {
43    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
44        f.debug_struct("DropBoxConfig")
45            .field("root", &self.root)
46            .finish_non_exhaustive()
47    }
48}
49
50impl crate::Configurator for DropboxConfig {
51    type Builder = DropboxBuilder;
52
53    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
54        let mut map = uri.options().clone();
55
56        if let Some(root) = uri.root() {
57            if !root.is_empty() {
58                map.insert("root".to_string(), root.to_string());
59            }
60        }
61
62        Self::from_iter(map)
63    }
64
65    #[allow(deprecated)]
66    fn into_builder(self) -> Self::Builder {
67        DropboxBuilder {
68            config: self,
69            http_client: None,
70        }
71    }
72}
73
74#[cfg(test)]
75mod tests {
76    use super::*;
77    use crate::Configurator;
78    use crate::types::OperatorUri;
79
80    #[test]
81    fn from_uri_sets_root() {
82        let uri =
83            OperatorUri::new("dropbox://remote/documents", Vec::<(String, String)>::new()).unwrap();
84
85        let cfg = DropboxConfig::from_uri(&uri).unwrap();
86        assert_eq!(cfg.root.as_deref(), Some("documents"));
87    }
88}
```
