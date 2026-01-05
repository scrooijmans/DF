# 

opendal/services/memcached/

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
19use std::time::Duration;
20
21use super::backend::MemcachedBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for MemCached services support
26#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct MemcachedConfig {
30    /// network address of the memcached service.
31    ///
32    /// For example: "tcp://localhost:11211"
33    pub endpoint: Option<String>,
34    /// the working directory of the service. Can be "/path/to/dir"
35    ///
36    /// default is "/"
37    pub root: Option<String>,
38    /// Memcached username, optional.
39    pub username: Option<String>,
40    /// Memcached password, optional.
41    pub password: Option<String>,
42    /// The default ttl for put operations.
43    pub default_ttl: Option<Duration>,
44}
45
46impl crate::Configurator for MemcachedConfig {
47    type Builder = MemcachedBuilder;
48
49    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
50        let authority = uri.authority().ok_or_else(|| {
51            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
52                .with_context("service", crate::Scheme::Memcached)
53        })?;
54
55        let mut map = uri.options().clone();
56        map.insert("endpoint".to_string(), format!("tcp://{authority}"));
57
58        if let Some(root) = uri.root() {
59            if !root.is_empty() {
60                map.insert("root".to_string(), root.to_string());
61            }
62        }
63
64        Self::from_iter(map)
65    }
66
67    fn into_builder(self) -> Self::Builder {
68        MemcachedBuilder { config: self }
69    }
70}
71
72#[cfg(test)]
73mod tests {
74    use super::*;
75    use crate::Configurator;
76    use crate::types::OperatorUri;
77
78    #[test]
79    fn from_uri_sets_endpoint_and_root() {
80        let uri = OperatorUri::new(
81            "memcached://cache.local:11211/app/session",
82            Vec::<(String, String)>::new(),
83        )
84        .unwrap();
85
86        let cfg = MemcachedConfig::from_uri(&uri).unwrap();
87        assert_eq!(cfg.endpoint.as_deref(), Some("tcp://cache.local:11211"));
88        assert_eq!(cfg.root.as_deref(), Some("app/session"));
89    }
90}
```
