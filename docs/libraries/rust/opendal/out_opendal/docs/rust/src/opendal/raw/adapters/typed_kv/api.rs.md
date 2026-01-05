# 

opendal/raw/adapters/typed_kv/

api.rs

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
18use crate::Buffer;
19use crate::EntryMode;
20use crate::Error;
21use crate::ErrorKind;
22use crate::Metadata;
23use crate::Result;
24use crate::Scheme;
25use crate::raw::{MaybeSend, Timestamp};
26use std::fmt::Debug;
27use std::future::Future;
28use std::future::ready;
29use std::mem::size_of;
30
31/// Adapter is the typed adapter to underlying kv services.
32///
33/// By implement this trait, any kv service can work as an OpenDAL Service.
34///
35/// # Notes
36///
37/// `typed_kv::Adapter` is the typed version of `kv::Adapter`. It's more
38/// efficient if the underlying kv service can store data with its type. For
39/// example, we can store `Bytes` along with its metadata so that we don't
40/// need to serialize/deserialize it when we get it from the service.
41///
42/// Ideally, we should use `typed_kv::Adapter` instead of `kv::Adapter` for
43/// in-memory rust libs like moka and dashmap.
44pub trait Adapter: Send + Sync + Debug + Unpin + 'static {
45    /// Return the info of this key value accessor.
46    fn info(&self) -> Info;
47
48    /// Get a value from adapter.
49    fn get(&self, path: &str) -> impl Future<Output = Result<Option<Value>>> + MaybeSend;
50
51    /// Set a value into adapter.
52    fn set(&self, path: &str, value: Value) -> impl Future<Output = Result<()>> + MaybeSend;
53
54    /// Delete a value from adapter.
55    fn delete(&self, path: &str) -> impl Future<Output = Result<()>> + MaybeSend;
56
57    /// Scan a key prefix to get all keys that start with this key.
58    fn scan(&self, path: &str) -> impl Future<Output = Result<Vec<String>>> + MaybeSend {
59        let _ = path;
60
61        ready(Err(Error::new(
62            ErrorKind::Unsupported,
63            "typed_kv adapter doesn't support this operation",
64        )
65        .with_operation("typed_kv::Adapter::scan")))
66    }
67}
68
69/// Value is the typed value stored in adapter.
70///
71/// It's cheap to clone so that users can read data without extra copy.
72#[derive(Debug, Clone)]
73pub struct Value {
74    /// Metadata of this value.
75    pub metadata: Metadata,
76    /// The corresponding content of this value.
77    pub value: Buffer,
78}
79
80impl Value {
81    /// Create a new dir of value.
82    pub fn new_dir() -> Self {
83        Self {
84            metadata: Metadata::new(EntryMode::DIR)
85                .with_content_length(0)
86                .with_last_modified(Timestamp::now()),
87            value: Buffer::new(),
88        }
89    }
90
91    /// Size returns the in-memory size of Value.
92    pub fn size(&self) -> usize {
93        size_of::<Metadata>() + self.value.len()
94    }
95}
96
97/// Capability is used to describe what operations are supported
98/// by Typed KV Operator.
99#[derive(Copy, Clone, Default)]
100pub struct Capability {
101    /// If typed_kv operator supports get natively.
102    pub get: bool,
103    /// If typed_kv operator supports set natively.
104    pub set: bool,
105    /// If typed_kv operator supports delete natively.
106    pub delete: bool,
107    /// If typed_kv operator supports scan natively.
108    pub scan: bool,
109    /// If typed_kv operator supports shared access.
110    pub shared: bool,
111}
112
113impl Debug for Capability {
114    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
115        let mut s = vec![];
116
117        if self.get {
118            s.push("Get")
119        }
120        if self.set {
121            s.push("Set");
122        }
123        if self.delete {
124            s.push("Delete");
125        }
126        if self.scan {
127            s.push("Scan");
128        }
129        if self.shared {
130            s.push("Shared");
131        }
132
133        write!(f, "{{ {} }}", s.join(" | "))
134    }
135}
136
137/// Info for this key value accessor.
138pub struct Info {
139    scheme: Scheme,
140    name: String,
141    capabilities: Capability,
142}
143
144impl Info {
145    /// Create a new KeyValueAccessorInfo.
146    pub fn new(scheme: Scheme, name: &str, capabilities: Capability) -> Self {
147        Self {
148            scheme,
149            name: name.to_string(),
150            capabilities,
151        }
152    }
153
154    /// Get the scheme.
155    pub fn scheme(&self) -> Scheme {
156        self.scheme
157    }
158
159    /// Get the name.
160    pub fn name(&self) -> &str {
161        &self.name
162    }
163
164    /// Get the capabilities.
165    pub fn capabilities(&self) -> Capability {
166        self.capabilities
167    }
168}
```
