# 

opendal/services/mysql/

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
24use super::backend::MysqlBuilder;
25
26/// Config for Mysql services support.
27#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
28#[serde(default)]
29#[non_exhaustive]
30pub struct MysqlConfig {
31    /// This connection string is used to connect to the mysql service. There are url based formats.
32    ///
33    /// The format of connect string resembles the url format of the mysql client.
34    /// The format is: `[scheme://][user[:[password]]@]host[:port][/schema][?attribute1=value1&attribute2=value2...`
35    ///
36    /// - `mysql://user@localhost`
37    /// - `mysql://user:password@localhost`
38    /// - `mysql://user:password@localhost:3306`
39    /// - `mysql://user:password@localhost:3306/db`
40    ///
41    /// For more information, please refer to <https://docs.rs/sqlx/latest/sqlx/mysql/struct.MySqlConnectOptions.html>.
42    pub connection_string: Option<String>,
43
44    /// The table name for mysql.
45    pub table: Option<String>,
46    /// The key field name for mysql.
47    pub key_field: Option<String>,
48    /// The value field name for mysql.
49    pub value_field: Option<String>,
50    /// The root for mysql.
51    pub root: Option<String>,
52}
53
54impl Debug for MysqlConfig {
55    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
56        let mut d = f.debug_struct("MysqlConfig");
57
58        if self.connection_string.is_some() {
59            d.field("connection_string", &"<redacted>");
60        }
61
62        d.field("root", &self.root)
63            .field("table", &self.table)
64            .field("key_field", &self.key_field)
65            .field("value_field", &self.value_field)
66            .finish()
67    }
68}
69
70impl crate::Configurator for MysqlConfig {
71    type Builder = MysqlBuilder;
72
73    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
74        let mut map = uri.options().clone();
75
76        if let Some(authority) = uri.authority() {
77            map.entry("connection_string".to_string())
78                .or_insert_with(|| format!("mysql://{authority}"));
79        }
80
81        if let Some(path) = uri.root() {
82            if !path.is_empty() {
83                let (table_segment, rest) = match path.split_once('/') {
84                    Some((table, remainder)) => (table, Some(remainder)),
85                    None => (path, None),
86                };
87
88                if !table_segment.is_empty() {
89                    map.entry("table".to_string())
90                        .or_insert_with(|| table_segment.to_string());
91                }
92
93                if let Some(root) = rest {
94                    if !root.is_empty() {
95                        map.insert("root".to_string(), root.to_string());
96                    }
97                }
98            }
99        }
100
101        Self::from_iter(map)
102    }
103
104    fn into_builder(self) -> Self::Builder {
105        MysqlBuilder { config: self }
106    }
107}
108
109#[cfg(test)]
110mod tests {
111    use super::*;
112    use crate::Configurator;
113    use crate::types::OperatorUri;
114
115    #[test]
116    fn from_uri_sets_connection_string_table_and_root() {
117        let uri = OperatorUri::new(
118            "mysql://db.example.com:3306/kv/cache",
119            Vec::<(String, String)>::new(),
120        )
121        .unwrap();
122
123        let cfg = MysqlConfig::from_uri(&uri).unwrap();
124        assert_eq!(
125            cfg.connection_string.as_deref(),
126            Some("mysql://db.example.com:3306")
127        );
128        assert_eq!(cfg.table.as_deref(), Some("kv"));
129        assert_eq!(cfg.root.as_deref(), Some("cache"));
130    }
131
132    #[test]
133    fn from_uri_respects_existing_table() {
134        let uri = OperatorUri::new(
135            "mysql://db.example.com:3306/users?root=logs",
136            Vec::<(String, String)>::new(),
137        )
138        .unwrap();
139
140        let cfg = MysqlConfig::from_uri(&uri).unwrap();
141        assert_eq!(cfg.table.as_deref(), Some("users"));
142        assert_eq!(cfg.root.as_deref(), Some("logs"));
143    }
144}
```
