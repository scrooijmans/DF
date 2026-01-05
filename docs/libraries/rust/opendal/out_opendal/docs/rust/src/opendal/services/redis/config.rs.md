# 

opendal/services/redis/

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
22use super::backend::RedisBuilder;
23use serde::Deserialize;
24use serde::Serialize;
25
26/// Config for Redis services support.
27#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
28#[serde(default)]
29#[non_exhaustive]
30pub struct RedisConfig {
31    /// network address of the Redis service. Can be "tcp://127.0.0.1:6379", e.g.
32    ///
33    /// default is "tcp://127.0.0.1:6379"
34    pub endpoint: Option<String>,
35    /// network address of the Redis cluster service. Can be "tcp://127.0.0.1:6379,tcp://127.0.0.1:6380,tcp://127.0.0.1:6381", e.g.
36    ///
37    /// default is None
38    pub cluster_endpoints: Option<String>,
39    /// the username to connect redis service.
40    ///
41    /// default is None
42    pub username: Option<String>,
43    /// the password for authentication
44    ///
45    /// default is None
46    pub password: Option<String>,
47    /// the working directory of the Redis service. Can be "/path/to/dir"
48    ///
49    /// default is "/"
50    pub root: Option<String>,
51    /// the number of DBs redis can take is unlimited
52    ///
53    /// default is db 0
54    pub db: i64,
55    /// The default ttl for put operations.
56    pub default_ttl: Option<Duration>,
57}
58
59impl Debug for RedisConfig {
60    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
61        let mut d = f.debug_struct("RedisConfig");
62
63        d.field("db", &self.db.to_string());
64        d.field("root", &self.root);
65        if let Some(endpoint) = self.endpoint.clone() {
66            d.field("endpoint", &endpoint);
67        }
68        if let Some(cluster_endpoints) = self.cluster_endpoints.clone() {
69            d.field("cluster_endpoints", &cluster_endpoints);
70        }
71        if let Some(username) = self.username.clone() {
72            d.field("username", &username);
73        }
74        if self.password.is_some() {
75            d.field("password", &"<redacted>");
76        }
77
78        d.finish_non_exhaustive()
79    }
80}
81
82impl crate::Configurator for RedisConfig {
83    type Builder = RedisBuilder;
84
85    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
86        let mut map = uri.options().clone();
87
88        if let Some(authority) = uri.authority() {
89            map.entry("endpoint".to_string())
90                .or_insert_with(|| format!("redis://{authority}"));
91        } else if !map.contains_key("endpoint") && !map.contains_key("cluster_endpoints") {
92            return Err(crate::Error::new(
93                crate::ErrorKind::ConfigInvalid,
94                "endpoint or cluster_endpoints is required",
95            )
96            .with_context("service", crate::Scheme::Redis));
97        }
98
99        if let Some(path) = uri.root() {
100            if !path.is_empty() {
101                if let Some((first, rest)) = path.split_once('/') {
102                    if let Ok(db) = first.parse::<i64>() {
103                        map.insert("db".to_string(), db.to_string());
104                        if !rest.is_empty() {
105                            map.insert("root".to_string(), rest.to_string());
106                        }
107                    } else {
108                        let mut root_value = first.to_string();
109                        if !rest.is_empty() {
110                            root_value.push('/');
111                            root_value.push_str(rest);
112                        }
113                        map.insert("root".to_string(), root_value);
114                    }
115                } else if let Ok(db) = path.parse::<i64>() {
116                    map.insert("db".to_string(), db.to_string());
117                } else {
118                    map.insert("root".to_string(), path.to_string());
119                }
120            }
121        }
122
123        Self::from_iter(map)
124    }
125
126    fn into_builder(self) -> Self::Builder {
127        RedisBuilder { config: self }
128    }
129}
130
131#[cfg(test)]
132mod tests {
133    use super::*;
134    use crate::Configurator;
135    use crate::types::OperatorUri;
136
137    #[test]
138    fn from_uri_sets_endpoint_db_and_root() {
139        let uri = OperatorUri::new(
140            "redis://localhost:6379/2/cache",
141            Vec::<(String, String)>::new(),
142        )
143        .unwrap();
144
145        let cfg = RedisConfig::from_uri(&uri).unwrap();
146        assert_eq!(cfg.endpoint.as_deref(), Some("redis://localhost:6379"));
147        assert_eq!(cfg.db, 2);
148        assert_eq!(cfg.root.as_deref(), Some("cache"));
149    }
150
151    #[test]
152    fn from_uri_treats_non_numeric_path_as_root() {
153        let uri = OperatorUri::new(
154            "redis://localhost:6379/app/data",
155            Vec::<(String, String)>::new(),
156        )
157        .unwrap();
158
159        let cfg = RedisConfig::from_uri(&uri).unwrap();
160        assert_eq!(cfg.endpoint.as_deref(), Some("redis://localhost:6379"));
161        assert_eq!(cfg.db, 0);
162        assert_eq!(cfg.root.as_deref(), Some("app/data"));
163    }
164
165    #[test]
166    fn test_redis_builder_interface() {
167        // Test that RedisBuilder still works with the new implementation
168        let builder = RedisBuilder::default()
169            .endpoint("redis://localhost:6379")
170            .username("testuser")
171            .password("testpass")
172            .db(1)
173            .root("/test");
174
175        // The builder should be able to create configuration
176        assert!(builder.config.endpoint.is_some());
177        assert_eq!(
178            builder.config.endpoint.as_ref().unwrap(),
179            "redis://localhost:6379"
180        );
181        assert_eq!(builder.config.username.as_ref().unwrap(), "testuser");
182        assert_eq!(builder.config.password.as_ref().unwrap(), "testpass");
183        assert_eq!(builder.config.db, 1);
184        assert_eq!(builder.config.root.as_ref().unwrap(), "/test");
185    }
186}
```
