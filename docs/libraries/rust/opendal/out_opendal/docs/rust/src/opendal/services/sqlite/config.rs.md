# 

opendal/services/sqlite/

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
21use super::backend::SqliteBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for Sqlite support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct SqliteConfig {
30    /// Set the connection_string of the sqlite service.
31    ///
32    /// This connection string is used to connect to the sqlite service.
33    ///
34    /// The format of connect string resembles the url format of the sqlite client:
35    ///
36    /// - `sqlite::memory:`
37    /// - `sqlite:data.db`
38    /// - `sqlite://data.db`
39    ///
40    /// For more information, please visit <https://docs.rs/sqlx/latest/sqlx/sqlite/struct.SqliteConnectOptions.html>.
41    pub connection_string: Option<String>,
42
43    /// Set the table name of the sqlite service to read/write.
44    pub table: Option<String>,
45    /// Set the key field name of the sqlite service to read/write.
46    ///
47    /// Default to `key` if not specified.
48    pub key_field: Option<String>,
49    /// Set the value field name of the sqlite service to read/write.
50    ///
51    /// Default to `value` if not specified.
52    pub value_field: Option<String>,
53    /// set the working directory, all operations will be performed under it.
54    ///
55    /// default: "/"
56    pub root: Option<String>,
57}
58
59impl Debug for SqliteConfig {
60    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
61        let mut d = f.debug_struct("SqliteConfig");
62
63        d.field("connection_string", &self.connection_string)
64            .field("table", &self.table)
65            .field("key_field", &self.key_field)
66            .field("value_field", &self.value_field)
67            .field("root", &self.root);
68
69        d.finish_non_exhaustive()
70    }
71}
72
73impl crate::Configurator for SqliteConfig {
74    type Builder = SqliteBuilder;
75    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
76        let mut map = uri.options().clone();
77
78        if let Some(authority) = uri.authority() {
79            map.entry("connection_string".to_string())
80                .or_insert_with(|| format!("sqlite://{authority}"));
81        }
82
83        if let Some(path) = uri.root() {
84            if !path.is_empty() {
85                let (table, rest) = match path.split_once('/') {
86                    Some((table, remainder)) => (table, Some(remainder)),
87                    None => (path, None),
88                };
89
90                if !table.is_empty() {
91                    map.entry("table".to_string())
92                        .or_insert_with(|| table.to_string());
93                }
94
95                if let Some(root) = rest {
96                    if !root.is_empty() {
97                        map.insert("root".to_string(), root.to_string());
98                    }
99                }
100            }
101        }
102
103        Self::from_iter(map)
104    }
105
106    fn into_builder(self) -> Self::Builder {
107        SqliteBuilder { config: self }
108    }
109}
110
111#[cfg(test)]
112mod tests {
113    use super::*;
114    use crate::Configurator;
115    use crate::types::OperatorUri;
116
117    #[test]
118    fn from_uri_sets_connection_string_table_and_root() {
119        let uri =
120            OperatorUri::new("sqlite://data.db/kv/cache", Vec::<(String, String)>::new()).unwrap();
121
122        let cfg = SqliteConfig::from_uri(&uri).unwrap();
123        assert_eq!(cfg.connection_string.as_deref(), Some("sqlite://data.db"));
124        assert_eq!(cfg.table.as_deref(), Some("kv"));
125        assert_eq!(cfg.root.as_deref(), Some("cache"));
126    }
127}
```
