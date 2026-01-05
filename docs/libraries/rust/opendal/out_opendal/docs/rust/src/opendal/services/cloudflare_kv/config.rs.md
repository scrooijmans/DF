# 

opendal/services/cloudflare_kv/

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
20use std::time::Duration;
21
22use super::backend::CloudflareKvBuilder;
23use serde::Deserialize;
24use serde::Serialize;
25
26/// Cloudflare KV Service Support.
27#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
28pub struct CloudflareKvConfig {
29    /// The token used to authenticate with CloudFlare.
30    pub api_token: Option<String>,
31    /// The account ID used to authenticate with CloudFlare. Used as URI path parameter.
32    pub account_id: Option<String>,
33    /// The namespace ID. Used as URI path parameter.
34    pub namespace_id: Option<String>,
35    /// The default ttl for write operations.
36    pub default_ttl: Option<Duration>,
37
38    /// Root within this backend.
39    pub root: Option<String>,
40}
41
42impl Debug for CloudflareKvConfig {
43    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
44        let mut ds = f.debug_struct("CloudflareKvConfig");
45
46        ds.field("root", &self.root);
47        ds.field("account_id", &self.account_id);
48        ds.field("namespace_id", &self.namespace_id);
49
50        if self.api_token.is_some() {
51            ds.field("api_token", &"<redacted>");
52        }
53
54        ds.finish()
55    }
56}
57
58impl crate::Configurator for CloudflareKvConfig {
59    type Builder = CloudflareKvBuilder;
60
61    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
62        let account_id = uri.name().ok_or_else(|| {
63            crate::Error::new(
64                crate::ErrorKind::ConfigInvalid,
65                "uri host must contain account id",
66            )
67            .with_context("service", crate::Scheme::CloudflareKv)
68        })?;
69
70        let raw_root = uri.root().ok_or_else(|| {
71            crate::Error::new(
72                crate::ErrorKind::ConfigInvalid,
73                "uri path must contain namespace id",
74            )
75            .with_context("service", crate::Scheme::CloudflareKv)
76        })?;
77
78        let mut segments = raw_root.splitn(2, '/');
79        let namespace_id = segments.next().filter(|s| !s.is_empty()).ok_or_else(|| {
80            crate::Error::new(
81                crate::ErrorKind::ConfigInvalid,
82                "namespace id is required in uri path",
83            )
84            .with_context("service", crate::Scheme::CloudflareKv)
85        })?;
86
87        let mut map = uri.options().clone();
88        map.insert("account_id".to_string(), account_id.to_string());
89        map.insert("namespace_id".to_string(), namespace_id.to_string());
90
91        if let Some(rest) = segments.next() {
92            if !rest.is_empty() {
93                map.insert("root".to_string(), rest.to_string());
94            }
95        }
96
97        Self::from_iter(map)
98    }
99
100    fn into_builder(self) -> Self::Builder {
101        CloudflareKvBuilder {
102            config: self,
103            http_client: None,
104        }
105    }
106}
107
108#[cfg(test)]
109mod tests {
110    use super::*;
111    use crate::Configurator;
112    use crate::types::OperatorUri;
113
114    #[test]
115    fn from_uri_extracts_ids_and_root() {
116        let uri = OperatorUri::new(
117            "cloudflare-kv://acc123/ns456/prefix/dir",
118            Vec::<(String, String)>::new(),
119        )
120        .unwrap();
121
122        let cfg = CloudflareKvConfig::from_uri(&uri).unwrap();
123        assert_eq!(cfg.account_id.as_deref(), Some("acc123"));
124        assert_eq!(cfg.namespace_id.as_deref(), Some("ns456"));
125        assert_eq!(cfg.root.as_deref(), Some("prefix/dir"));
126    }
127
128    #[test]
129    fn from_uri_requires_namespace() {
130        let uri =
131            OperatorUri::new("cloudflare-kv://acc123", Vec::<(String, String)>::new()).unwrap();
132
133        assert!(CloudflareKvConfig::from_uri(&uri).is_err());
134    }
135}
```
