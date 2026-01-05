# 

opendal/raw/adapters/kv/

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
18use std::fmt::Debug;
19use std::future::ready;
20use std::ops::DerefMut;
21
22use futures::Future;
23
24use crate::Capability;
25use crate::Scheme;
26use crate::raw::*;
27use crate::*;
28
29/// Scan is the async iterator returned by `Adapter::scan`.
30pub trait Scan: Send + Sync + Unpin {
31    /// Fetch the next key in the current key prefix
32    ///
33    /// `Ok(None)` means no further key will be returned
34    fn next(&mut self) -> impl Future<Output = Result<Option<String>>> + MaybeSend;
35}
36
37/// A noop implementation of Scan
38impl Scan for () {
39    async fn next(&mut self) -> Result<Option<String>> {
40        Ok(None)
41    }
42}
43
44/// A Scan implementation for all trivial non-async iterators
45#[allow(dead_code)]
46pub struct ScanStdIter<I>(I);
47
48#[cfg(any(feature = "services-rocksdb", feature = "services-sled"))]
49impl<I> ScanStdIter<I>
50where
51    I: Iterator<Item = Result<String>> + Unpin + Send + Sync,
52{
53    /// Create a new ScanStdIter from an Iterator
54    pub(crate) fn new(inner: I) -> Self {
55        Self(inner)
56    }
57}
58
59impl<I> Scan for ScanStdIter<I>
60where
61    I: Iterator<Item = Result<String>> + Unpin + Send + Sync,
62{
63    async fn next(&mut self) -> Result<Option<String>> {
64        self.0.next().transpose()
65    }
66}
67
68/// A type-erased wrapper of Scan
69pub type Scanner = Box<dyn ScanDyn>;
70
71pub trait ScanDyn: Unpin + Send + Sync {
72    fn next_dyn(&mut self) -> BoxedFuture<'_, Result<Option<String>>>;
73}
74
75impl<T: Scan + ?Sized> ScanDyn for T {
76    fn next_dyn(&mut self) -> BoxedFuture<'_, Result<Option<String>>> {
77        Box::pin(self.next())
78    }
79}
80
81impl<T: ScanDyn + ?Sized> Scan for Box<T> {
82    async fn next(&mut self) -> Result<Option<String>> {
83        self.deref_mut().next_dyn().await
84    }
85}
86
87/// KvAdapter is the adapter to underlying kv services.
88///
89/// By implement this trait, any kv service can work as an OpenDAL Service.
90pub trait Adapter: Send + Sync + Debug + Unpin + 'static {
91    /// TODO: use default associate type `= ()` after stabilized
92    type Scanner: Scan;
93
94    /// Return the info of this key value accessor.
95    fn info(&self) -> Info;
96
97    /// Get a key from service.
98    ///
99    /// - return `Ok(None)` if this key is not exist.
100    fn get(&self, path: &str) -> impl Future<Output = Result<Option<Buffer>>> + MaybeSend;
101
102    /// Set a key into service.
103    fn set(&self, path: &str, value: Buffer) -> impl Future<Output = Result<()>> + MaybeSend;
104
105    /// Delete a key from service.
106    ///
107    /// - return `Ok(())` even if this key is not exist.
108    fn delete(&self, path: &str) -> impl Future<Output = Result<()>> + MaybeSend;
109
110    /// Scan a key prefix to get all keys that start with this key.
111    fn scan(&self, path: &str) -> impl Future<Output = Result<Self::Scanner>> + MaybeSend {
112        let _ = path;
113
114        ready(Err(Error::new(
115            ErrorKind::Unsupported,
116            "kv adapter doesn't support this operation",
117        )
118        .with_operation("kv::Adapter::scan")))
119    }
120
121    /// Append a key into service
122    fn append(&self, path: &str, value: &[u8]) -> impl Future<Output = Result<()>> + MaybeSend {
123        let _ = path;
124        let _ = value;
125
126        ready(Err(Error::new(
127            ErrorKind::Unsupported,
128            "kv adapter doesn't support this operation",
129        )
130        .with_operation("kv::Adapter::append")))
131    }
132}
133
134/// Info for this key value accessor.
135pub struct Info {
136    scheme: Scheme,
137    name: String,
138    capabilities: Capability,
139}
140
141impl Info {
142    /// Create a new KeyValueAccessorInfo.
143    pub fn new(scheme: Scheme, name: &str, capabilities: Capability) -> Self {
144        Self {
145            scheme,
146            name: name.to_string(),
147            capabilities,
148        }
149    }
150
151    /// Get the scheme.
152    pub fn scheme(&self) -> Scheme {
153        self.scheme
154    }
155
156    /// Get the name.
157    pub fn name(&self) -> &str {
158        &self.name
159    }
160
161    /// Get the capabilities.
162    pub fn capabilities(&self) -> Capability {
163        self.capabilities
164    }
165}
```
