# 

opendal/services/surrealdb/

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
21use serde::Deserialize;
22use serde::Serialize;
23
24use super::backend::SurrealdbBuilder;
25
26/// Config for Surrealdb services support.
27#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
28#[serde(default)]
29#[non_exhaustive]
30pub struct SurrealdbConfig {
31    /// The connection string for surrealdb.
32    pub connection_string: Option<String>,
33    /// The username for surrealdb.
34    pub username: Option<String>,
35    /// The password for surrealdb.
36    pub password: Option<String>,
37    /// The namespace for surrealdb.
38    pub namespace: Option<String>,
39    /// The database for surrealdb.
40    pub database: Option<String>,
41    /// The table for surrealdb.
42    pub table: Option<String>,
43    /// The key field for surrealdb.
44    pub key_field: Option<String>,
45    /// The value field for surrealdb.
46    pub value_field: Option<String>,
47    /// The root for surrealdb.
48    pub root: Option<String>,
49}
50
51impl Debug for SurrealdbConfig {
52    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
53        let mut d = f.debug_struct("SurrealdbConfig");
54
55        d.field("connection_string", &self.connection_string)
56            .field("username", &self.username)
57            .field("password", &"<redacted>")
58            .field("namespace", &self.namespace)
59            .field("database", &self.database)
60            .field("table", &self.table)
61            .field("key_field", &self.key_field)
62            .field("value_field", &self.value_field)
63            .field("root", &self.root)
64            .finish()
65    }
66}
67
68impl crate::Configurator for SurrealdbConfig {
69    type Builder = SurrealdbBuilder;
70
71    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
72        let mut map = uri.options().clone();
73
74        if let Some(authority) = uri.authority() {
75            map.entry("connection_string".to_string())
76                .or_insert_with(|| format!("ws://{authority}"));
77        }
78
79        if let Some(path) = uri.root() {
80            if !path.is_empty() {
81                let mut segments = path.splitn(4, '/');
82                if let Some(namespace) = segments.next() {
83                    if !namespace.is_empty() {
84                        map.entry("namespace".to_string())
85                            .or_insert_with(|| namespace.to_string());
86                    }
87                }
88                if let Some(database) = segments.next() {
89                    if !database.is_empty() {
90                        map.entry("database".to_string())
91                            .or_insert_with(|| database.to_string());
92                    }
93                }
94                if let Some(table) = segments.next() {
95                    if !table.is_empty() {
96                        map.entry("table".to_string())
97                            .or_insert_with(|| table.to_string());
98                    }
99                }
100                if let Some(rest) = segments.next() {
101                    if !rest.is_empty() {
102                        map.insert("root".to_string(), rest.to_string());
103                    }
104                }
105            }
106        }
107
108        Self::from_iter(map)
109    }
110
111    fn into_builder(self) -> Self::Builder {
112        SurrealdbBuilder { config: self }
113    }
114}
115
116#[cfg(test)]
117mod tests {
118    use super::*;
119    use crate::Configurator;
120    use crate::types::OperatorUri;
121
122    #[test]
123    fn from_uri_sets_connection_namespace_database_table_and_root() {
124        let uri = OperatorUri::new(
125            "surrealdb://db.example.com:8000/project/app/cache/static",
126            Vec::<(String, String)>::new(),
127        )
128        .unwrap();
129
130        let cfg = SurrealdbConfig::from_uri(&uri).unwrap();
131        assert_eq!(
132            cfg.connection_string.as_deref(),
133            Some("ws://db.example.com:8000")
134        );
135        assert_eq!(cfg.namespace.as_deref(), Some("project"));
136        assert_eq!(cfg.database.as_deref(), Some("app"));
137        assert_eq!(cfg.table.as_deref(), Some("cache"));
138        assert_eq!(cfg.root.as_deref(), Some("static"));
139    }
140}
```
