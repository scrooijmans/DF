# 

opendal/services/moka/

core.rs

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
21use moka::future::Cache;
22
23use crate::*;
24
25/// Value stored in moka cache containing both metadata and content
26#[derive(Clone)]
27pub struct MokaValue {
28    /// Stored metadata in moka cache.
29    pub metadata: Metadata,
30    /// Stored content in moka cache.
31    pub content: Buffer,
32}
33
34#[derive(Clone)]
35pub struct MokaCore {
36    pub cache: Cache<String, MokaValue>,
37}
38
39impl Debug for MokaCore {
40    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
41        f.debug_struct("MokaCore")
42            .field("size", &self.cache.weighted_size())
43            .field("count", &self.cache.entry_count())
44            .finish()
45    }
46}
47
48impl MokaCore {
49    pub async fn get(&self, key: &str) -> Result<Option<MokaValue>> {
50        Ok(self.cache.get(key).await)
51    }
52
53    pub async fn set(&self, key: &str, value: MokaValue) -> Result<()> {
54        self.cache.insert(key.to_string(), value).await;
55        Ok(())
56    }
57
58    pub async fn delete(&self, key: &str) -> Result<()> {
59        self.cache.invalidate(key).await;
60        Ok(())
61    }
62}
```
