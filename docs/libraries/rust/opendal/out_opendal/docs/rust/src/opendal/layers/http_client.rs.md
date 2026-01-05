# 

opendal/layers/

http_client.rs

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
18use crate::raw::*;
19use crate::*;
20
21/// Layer for replacing the default HTTP client with a custom one.
22///
23/// This layer allows users to provide their own HTTP client implementation
24/// by implementing the [`HttpFetch`] trait. This is useful when you need
25/// to customize HTTP behavior, add authentication, use a different HTTP
26/// client library, or apply custom middleware.
27///
28/// # Examples
29///
30/// ```no_run
31/// use opendal::layers::HttpClientLayer;
32/// use opendal::services;
33/// use opendal::Operator;
34/// use opendal::Result;
35/// use opendal::raw::HttpClient;
36///
37/// # fn main() -> Result<()> {
38/// // Create a custom HTTP client
39/// let custom_client = HttpClient::new()?;
40///
41/// let op = Operator::new(services::S3::default())?
42///     .layer(HttpClientLayer::new(custom_client))
43///     .finish();
44/// # Ok(())
45/// # }
46/// ```
47///
48/// # Custom HTTP Client Implementation
49///
50/// ```no_run
51/// use opendal::raw::{HttpFetch, HttpBody};
52/// use opendal::Buffer;
53/// use http::{Request, Response};
54/// use opendal::Result;
55///
56/// struct CustomHttpClient {
57///     // Your custom HTTP client fields
58/// }
59///
60/// impl HttpFetch for CustomHttpClient {
61///     async fn fetch(&self, req: Request<Buffer>) -> Result<Response<HttpBody>> {
62///         // Your custom HTTP client implementation
63///         todo!()
64///     }
65/// }
66/// ```
67#[derive(Clone)]
68pub struct HttpClientLayer {
69    client: HttpClient,
70}
71
72impl HttpClientLayer {
73    /// Create a new `HttpClientLayer` with the given HTTP client.
74    ///
75    /// # Arguments
76    ///
77    /// * `client` - The HTTP client to use for all HTTP requests
78    ///
79    /// # Examples
80    ///
81    /// ```no_run
82    /// use opendal::layers::HttpClientLayer;
83    /// use opendal::raw::HttpClient;
84    /// use opendal::Result;
85    ///
86    /// # fn main() -> Result<()> {
87    /// let client = HttpClient::new()?;
88    /// let layer = HttpClientLayer::new(client);
89    /// # Ok(())
90    /// # }
91    /// ```
92    pub fn new(client: HttpClient) -> Self {
93        Self { client }
94    }
95}
96
97impl<A: Access> Layer<A> for HttpClientLayer {
98    type LayeredAccess = HttpClientAccessor<A>;
99
100    fn layer(&self, inner: A) -> Self::LayeredAccess {
101        let info = inner.info();
102
103        // Replace the HTTP client with our custom one
104        info.update_http_client(|_| self.client.clone());
105
106        HttpClientAccessor { inner }
107    }
108}
109
110/// The accessor returned by [`HttpClientLayer`].
111///
112/// This accessor simply passes through all operations to the inner accessor,
113/// while the HTTP client replacement is handled at the layer level.
114#[derive(Debug, Clone)]
115pub struct HttpClientAccessor<A: Access> {
116    inner: A,
117}
118
119impl<A: Access> LayeredAccess for HttpClientAccessor<A> {
120    type Inner = A;
121    type Reader = A::Reader;
122    type Writer = A::Writer;
123    type Lister = A::Lister;
124    type Deleter = A::Deleter;
125
126    fn inner(&self) -> &Self::Inner {
127        &self.inner
128    }
129
130    async fn create_dir(&self, path: &str, args: OpCreateDir) -> Result<RpCreateDir> {
131        self.inner.create_dir(path, args).await
132    }
133
134    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
135        self.inner.read(path, args).await
136    }
137
138    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
139        self.inner.write(path, args).await
140    }
141
142    async fn copy(&self, from: &str, to: &str, args: OpCopy) -> Result<RpCopy> {
143        self.inner.copy(from, to, args).await
144    }
145
146    async fn rename(&self, from: &str, to: &str, args: OpRename) -> Result<RpRename> {
147        self.inner.rename(from, to, args).await
148    }
149
150    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
151        self.inner.stat(path, args).await
152    }
153
154    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
155        self.inner.delete().await
156    }
157
158    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
159        self.inner.list(path, args).await
160    }
161
162    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
163        self.inner.presign(path, args).await
164    }
165}
166
167#[cfg(test)]
168mod tests {
169    use super::*;
170    use crate::services;
171
172    #[tokio::test]
173    async fn test_http_client_layer() {
174        let layer = HttpClientLayer::new(HttpClient::new().unwrap());
175        let op = Operator::new(services::Memory::default())
176            .unwrap()
177            .layer(layer)
178            .finish();
179
180        // Basic test to ensure the layer doesn't break functionality
181        op.write("test", "data").await.unwrap();
182        let content = op.read("test").await.unwrap();
183        assert_eq!(content.to_bytes(), "data");
184    }
185
186    #[tokio::test]
187    async fn test_http_client_layer_with_fetcher() {
188        struct MockFetcher;
189
190        impl HttpFetch for MockFetcher {
191            async fn fetch(&self, _req: http::Request<Buffer>) -> Result<http::Response<HttpBody>> {
192                // For testing purposes, we just return an error since Memory service doesn't use HTTP
193                Err(Error::new(ErrorKind::Unexpected, "mock fetcher"))
194            }
195        }
196
197        let client = HttpClient::with(MockFetcher);
198        let layer = HttpClientLayer::new(client);
199        let _op = Operator::new(services::Memory::default())
200            .unwrap()
201            .layer(layer)
202            .finish();
203
204        // The layer should be created successfully even with a custom fetcher
205    }
206}
```
