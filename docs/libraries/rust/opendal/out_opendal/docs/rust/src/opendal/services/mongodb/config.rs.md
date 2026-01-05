# 

opendal/services/mongodb/

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
24use super::backend::MongodbBuilder;
25
26/// Config for Mongodb service support.
27#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
28#[serde(default)]
29#[non_exhaustive]
30pub struct MongodbConfig {
31    /// connection string of this backend
32    pub connection_string: Option<String>,
33    /// database of this backend
34    pub database: Option<String>,
35    /// collection of this backend
36    pub collection: Option<String>,
37    /// root of this backend
38    pub root: Option<String>,
39    /// key field of this backend
40    pub key_field: Option<String>,
41    /// value field of this backend
42    pub value_field: Option<String>,
43}
44
45impl Debug for MongodbConfig {
46    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
47        f.debug_struct("MongodbConfig")
48            .field("database", &self.database)
49            .field("collection", &self.collection)
50            .field("root", &self.root)
51            .field("key_field", &self.key_field)
52            .field("value_field", &self.value_field)
53            .finish()
54    }
55}
56
57impl crate::Configurator for MongodbConfig {
58    type Builder = MongodbBuilder;
59
60    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
61        let mut map = uri.options().clone();
62
63        if let Some(authority) = uri.authority() {
64            map.entry("connection_string".to_string())
65                .or_insert_with(|| format!("mongodb://{authority}"));
66        }
67
68        if let Some(path) = uri.root() {
69            if !path.is_empty() {
70                let mut segments = path.splitn(3, '/');
71                if let Some(db) = segments.next() {
72                    if !db.is_empty() {
73                        map.entry("database".to_string())
74                            .or_insert_with(|| db.to_string());
75                    }
76                }
77                if let Some(collection) = segments.next() {
78                    if !collection.is_empty() {
79                        map.entry("collection".to_string())
80                            .or_insert_with(|| collection.to_string());
81                    }
82                }
83                if let Some(rest) = segments.next() {
84                    if !rest.is_empty() {
85                        map.insert("root".to_string(), rest.to_string());
86                    }
87                }
88            }
89        }
90
91        Self::from_iter(map)
92    }
93
94    fn into_builder(self) -> Self::Builder {
95        MongodbBuilder { config: self }
96    }
97}
98
99#[cfg(test)]
100mod tests {
101    use super::*;
102    use crate::Configurator;
103    use crate::types::OperatorUri;
104
105    #[test]
106    fn from_uri_sets_connection_string_database_collection_and_root() {
107        let uri = OperatorUri::new(
108            "mongodb://mongo.internal:27017/analytics/events/session",
109            Vec::<(String, String)>::new(),
110        )
111        .unwrap();
112
113        let cfg = MongodbConfig::from_uri(&uri).unwrap();
114        assert_eq!(
115            cfg.connection_string.as_deref(),
116            Some("mongodb://mongo.internal:27017")
117        );
118        assert_eq!(cfg.database.as_deref(), Some("analytics"));
119        assert_eq!(cfg.collection.as_deref(), Some("events"));
120        assert_eq!(cfg.root.as_deref(), Some("session"));
121    }
122}
```
