# 

opendal/services/azfile/

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
21use super::backend::AzfileBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Azure File services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27pub struct AzfileConfig {
28    /// The root path for azfile.
29    pub root: Option<String>,
30    /// The endpoint for azfile.
31    pub endpoint: Option<String>,
32    /// The share name for azfile.
33    pub share_name: String,
34    /// The account name for azfile.
35    pub account_name: Option<String>,
36    /// The account key for azfile.
37    pub account_key: Option<String>,
38    /// The sas token for azfile.
39    pub sas_token: Option<String>,
40}
41
42impl Debug for AzfileConfig {
43    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
44        let mut ds = f.debug_struct("AzfileConfig");
45
46        ds.field("root", &self.root);
47        ds.field("share_name", &self.share_name);
48        ds.field("endpoint", &self.endpoint);
49
50        if self.account_name.is_some() {
51            ds.field("account_name", &"<redacted>");
52        }
53        if self.account_key.is_some() {
54            ds.field("account_key", &"<redacted>");
55        }
56        if self.sas_token.is_some() {
57            ds.field("sas_token", &"<redacted>");
58        }
59
60        ds.finish()
61    }
62}
63
64impl crate::Configurator for AzfileConfig {
65    type Builder = AzfileBuilder;
66
67    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
68        let authority = uri.authority().ok_or_else(|| {
69            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
70                .with_context("service", crate::Scheme::Azfile)
71        })?;
72
73        let mut map = uri.options().clone();
74        map.insert("endpoint".to_string(), format!("https://{authority}"));
75
76        if let Some(host) = uri.name() {
77            if let Some(account) = host.split('.').next() {
78                if !account.is_empty() {
79                    map.entry("account_name".to_string())
80                        .or_insert_with(|| account.to_string());
81                }
82            }
83        }
84
85        if let Some(root) = uri.root() {
86            if let Some((share, rest)) = root.split_once('/') {
87                if share.is_empty() {
88                    return Err(crate::Error::new(
89                        crate::ErrorKind::ConfigInvalid,
90                        "share name is required in uri path",
91                    )
92                    .with_context("service", crate::Scheme::Azfile));
93                }
94                map.insert("share_name".to_string(), share.to_string());
95                if !rest.is_empty() {
96                    map.insert("root".to_string(), rest.to_string());
97                }
98            } else if !root.is_empty() {
99                map.insert("share_name".to_string(), root.to_string());
100            }
101        }
102
103        if !map.contains_key("share_name") {
104            return Err(crate::Error::new(
105                crate::ErrorKind::ConfigInvalid,
106                "share name is required",
107            )
108            .with_context("service", crate::Scheme::Azfile));
109        }
110
111        Self::from_iter(map)
112    }
113
114    #[allow(deprecated)]
115    fn into_builder(self) -> Self::Builder {
116        AzfileBuilder {
117            config: self,
118            http_client: None,
119        }
120    }
121}
122
123#[cfg(test)]
124mod tests {
125    use super::*;
126    use crate::Configurator;
127    use crate::types::OperatorUri;
128
129    #[test]
130    fn from_uri_sets_endpoint_share_root_and_account() {
131        let uri = OperatorUri::new(
132            "azfile://account.file.core.windows.net/share/documents/reports",
133            Vec::<(String, String)>::new(),
134        )
135        .unwrap();
136
137        let cfg = AzfileConfig::from_uri(&uri).unwrap();
138        assert_eq!(
139            cfg.endpoint.as_deref(),
140            Some("https://account.file.core.windows.net")
141        );
142        assert_eq!(cfg.share_name, "share".to_string());
143        assert_eq!(cfg.root.as_deref(), Some("documents/reports"));
144        assert_eq!(cfg.account_name.as_deref(), Some("account"));
145    }
146
147    #[test]
148    fn from_uri_accepts_share_from_query() {
149        let uri = OperatorUri::new(
150            "azfile://account.file.core.windows.net",
151            vec![("share_name".to_string(), "data".to_string())],
152        )
153        .unwrap();
154
155        let cfg = AzfileConfig::from_uri(&uri).unwrap();
156        assert_eq!(
157            cfg.endpoint.as_deref(),
158            Some("https://account.file.core.windows.net")
159        );
160        assert_eq!(cfg.share_name, "data".to_string());
161    }
162}
```
