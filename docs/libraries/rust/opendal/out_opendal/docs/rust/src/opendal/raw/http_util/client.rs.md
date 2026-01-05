# 

opendal/raw/http_util/

client.rs

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
18use std::convert::Infallible;
19use std::fmt::Debug;
20use std::fmt::Formatter;
21use std::future;
22use std::mem;
23use std::ops::Deref;
24use std::pin::Pin;
25use std::str::FromStr;
26use std::sync::Arc;
27use std::sync::LazyLock;
28use std::task::Context;
29use std::task::Poll;
30
31use bytes::Bytes;
32use futures::Future;
33use futures::TryStreamExt;
34use http::Request;
35use http::Response;
36use http_body::Frame;
37use http_body::SizeHint;
38use raw::oio::Read;
39
40use super::HttpBody;
41use super::parse_content_encoding;
42use super::parse_content_length;
43use crate::raw::*;
44use crate::*;
45
46/// Http client used across opendal for loading credentials.
47/// This is merely a temporary solution because reqsign requires a reqwest client to be passed.
48/// We will remove it after the next major version of reqsign, which will enable users to provide their own client.
49#[allow(dead_code)]
50pub(crate) static GLOBAL_REQWEST_CLIENT: LazyLock<reqwest::Client> =
51    LazyLock::new(reqwest::Client::new);
52
53/// HttpFetcher is a type erased [`HttpFetch`].
54pub type HttpFetcher = Arc<dyn HttpFetchDyn>;
55
56/// A HTTP client instance for OpenDAL's services.
57///
58/// # Notes
59///
60/// * A http client must support redirections that follows 3xx response.
61#[derive(Clone)]
62pub struct HttpClient {
63    fetcher: HttpFetcher,
64}
65
66/// We don't want users to know details about our clients.
67impl Debug for HttpClient {
68    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
69        f.debug_struct("HttpClient").finish()
70    }
71}
72
73impl Default for HttpClient {
74    fn default() -> Self {
75        Self {
76            fetcher: Arc::new(GLOBAL_REQWEST_CLIENT.clone()),
77        }
78    }
79}
80
81impl HttpClient {
82    /// Create a new http client in async context.
83    pub fn new() -> Result<Self> {
84        Ok(Self::default())
85    }
86
87    /// Construct `Self` with given [`reqwest::Client`]
88    pub fn with(client: impl HttpFetch) -> Self {
89        let fetcher = Arc::new(client);
90        Self { fetcher }
91    }
92
93    /// Get the inner http client.
94    pub(crate) fn into_inner(self) -> HttpFetcher {
95        self.fetcher
96    }
97
98    /// Build a new http client in async context.
99    #[deprecated]
100    pub fn build(builder: reqwest::ClientBuilder) -> Result<Self> {
101        let client = builder.build().map_err(|err| {
102            Error::new(ErrorKind::Unexpected, "http client build failed").set_source(err)
103        })?;
104        let fetcher = Arc::new(client);
105        Ok(Self { fetcher })
106    }
107
108    /// Send a request and consume response.
109    pub async fn send(&self, req: Request<Buffer>) -> Result<Response<Buffer>> {
110        let (parts, mut body) = self.fetch(req).await?.into_parts();
111        let buffer = body.read_all().await?;
112        Ok(Response::from_parts(parts, buffer))
113    }
114
115    /// Fetch a request and return a streamable [`HttpBody`].
116    ///
117    /// Services can use [`HttpBody`] as [`Access::Read`].
118    pub async fn fetch(&self, req: Request<Buffer>) -> Result<Response<HttpBody>> {
119        self.fetcher.fetch(req).await
120    }
121}
122
123/// HttpFetch is the trait to fetch a request in async way.
124/// User should implement this trait to provide their own http client.
125pub trait HttpFetch: Send + Sync + Unpin + 'static {
126    /// Fetch a request in async way.
127    fn fetch(
128        &self,
129        req: Request<Buffer>,
130    ) -> impl Future<Output = Result<Response<HttpBody>>> + MaybeSend;
131}
132
133/// HttpFetchDyn is the dyn version of [`HttpFetch`]
134/// which make it possible to use as `Arc<dyn HttpFetchDyn>`.
135/// User should never implement this trait, but use `HttpFetch` instead.
136pub trait HttpFetchDyn: Send + Sync + Unpin + 'static {
137    /// The dyn version of [`HttpFetch::fetch`].
138    ///
139    /// This function returns a boxed future to make it object safe.
140    fn fetch_dyn(&self, req: Request<Buffer>) -> BoxedFuture<'_, Result<Response<HttpBody>>>;
141}
142
143impl<T: HttpFetch + ?Sized> HttpFetchDyn for T {
144    fn fetch_dyn(&self, req: Request<Buffer>) -> BoxedFuture<'_, Result<Response<HttpBody>>> {
145        Box::pin(self.fetch(req))
146    }
147}
148
149impl<T: HttpFetchDyn + ?Sized> HttpFetch for Arc<T> {
150    async fn fetch(&self, req: Request<Buffer>) -> Result<Response<HttpBody>> {
151        self.deref().fetch_dyn(req).await
152    }
153}
154
155impl HttpFetch for reqwest::Client {
156    async fn fetch(&self, req: Request<Buffer>) -> Result<Response<HttpBody>> {
157        // Uri stores all string alike data in `Bytes` which means
158        // the clone here is cheap.
159        let uri = req.uri().clone();
160        let is_head = req.method() == http::Method::HEAD;
161
162        let (parts, body) = req.into_parts();
163
164        let url = reqwest::Url::from_str(&uri.to_string()).map_err(|err| {
165            Error::new(ErrorKind::Unexpected, "request url is invalid")
166                .with_operation("http_util::Client::send::fetch")
167                .with_context("url", uri.to_string())
168                .set_source(err)
169        })?;
170
171        let mut req_builder = self.request(parts.method, url).headers(parts.headers);
172
173        // Client under wasm doesn't support set version.
174        #[cfg(not(target_arch = "wasm32"))]
175        {
176            req_builder = req_builder.version(parts.version);
177        }
178
179        // Don't set body if body is empty.
180        if !body.is_empty() {
181            #[cfg(not(target_arch = "wasm32"))]
182            {
183                req_builder = req_builder.body(reqwest::Body::wrap(HttpBufferBody(body)))
184            }
185            #[cfg(target_arch = "wasm32")]
186            {
187                req_builder = req_builder.body(reqwest::Body::from(body.to_bytes()))
188            }
189        }
190
191        let mut resp = req_builder.send().await.map_err(|err| {
192            Error::new(ErrorKind::Unexpected, "send http request")
193                .with_operation("http_util::Client::send")
194                .with_context("url", uri.to_string())
195                .with_temporary(is_temporary_error(&err))
196                .set_source(err)
197        })?;
198
199        // Get content length from header so that we can check it.
200        //
201        // - If the request method is HEAD, we will ignore content length.
202        // - If response contains content_encoding, we should omit its content length.
203        let content_length = if is_head || parse_content_encoding(resp.headers())?.is_some() {
204            None
205        } else {
206            parse_content_length(resp.headers())?
207        };
208
209        let mut hr = Response::builder()
210            .status(resp.status())
211            // Insert uri into response extension so that we can fetch
212            // it later.
213            .extension(uri.clone());
214
215        // Response builder under wasm doesn't support set version.
216        #[cfg(not(target_arch = "wasm32"))]
217        {
218            hr = hr.version(resp.version());
219        }
220
221        // Swap headers directly instead of copy the entire map.
222        mem::swap(hr.headers_mut().unwrap(), resp.headers_mut());
223
224        let bs = HttpBody::new(
225            resp.bytes_stream()
226                .try_filter(|v| future::ready(!v.is_empty()))
227                .map_ok(Buffer::from)
228                .map_err(move |err| {
229                    Error::new(ErrorKind::Unexpected, "read data from http response")
230                        .with_operation("http_util::Client::send")
231                        .with_context("url", uri.to_string())
232                        .with_temporary(is_temporary_error(&err))
233                        .set_source(err)
234                }),
235            content_length,
236        );
237
238        let resp = hr.body(bs).expect("response must build succeed");
239        Ok(resp)
240    }
241}
242
243#[inline]
244fn is_temporary_error(err: &reqwest::Error) -> bool {
245    // error sending request
246    err.is_request()||
247    // request or response body error
248    err.is_body() ||
249    // error decoding response body, for example, connection reset.
250    err.is_decode()
251}
252
253struct HttpBufferBody(Buffer);
254
255impl http_body::Body for HttpBufferBody {
256    type Data = Bytes;
257    type Error = Infallible;
258
259    fn poll_frame(
260        mut self: Pin<&mut Self>,
261        _: &mut Context<'_>,
262    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
263        match self.0.next() {
264            Some(bs) => Poll::Ready(Some(Ok(Frame::data(bs)))),
265            None => Poll::Ready(None),
266        }
267    }
268
269    fn is_end_stream(&self) -> bool {
270        self.0.is_empty()
271    }
272
273    fn size_hint(&self) -> SizeHint {
274        SizeHint::with_exact(self.0.len() as u64)
275    }
276}
```
