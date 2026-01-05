# 

opendal/raw/

rps.rs

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
18use http::Request;
19
20use crate::raw::*;
21use crate::*;
22
23/// Reply for `create_dir` operation
24#[derive(Debug, Clone, Default)]
25pub struct RpCreateDir {}
26
27/// Reply for `delete` operation
28#[derive(Debug, Clone, Default)]
29pub struct RpDelete {}
30
31/// Reply for `list` operation.
32#[derive(Debug, Clone, Default)]
33pub struct RpList {}
34
35/// Reply for `presign` operation.
36#[derive(Debug, Clone)]
37pub struct RpPresign {
38    req: PresignedRequest,
39}
40
41impl RpPresign {
42    /// Create a new reply for `presign`.
43    pub fn new(req: PresignedRequest) -> Self {
44        RpPresign { req }
45    }
46
47    /// Consume reply to build a presigned request.
48    pub fn into_presigned_request(self) -> PresignedRequest {
49        self.req
50    }
51}
52
53/// PresignedRequest is a presigned request return by `presign`.
54#[derive(Debug, Clone)]
55pub struct PresignedRequest {
56    method: http::Method,
57    uri: http::Uri,
58    headers: http::HeaderMap,
59}
60
61impl PresignedRequest {
62    /// Create a new PresignedRequest
63    pub fn new(method: http::Method, uri: http::Uri, headers: http::HeaderMap) -> Self {
64        Self {
65            method,
66            uri,
67            headers,
68        }
69    }
70
71    /// Return request's method.
72    pub fn method(&self) -> &http::Method {
73        &self.method
74    }
75
76    /// Return request's uri.
77    pub fn uri(&self) -> &http::Uri {
78        &self.uri
79    }
80
81    /// Return request's header.
82    pub fn header(&self) -> &http::HeaderMap {
83        &self.headers
84    }
85}
86
87impl<T: Default> From<PresignedRequest> for Request<T> {
88    fn from(v: PresignedRequest) -> Self {
89        let mut builder = Request::builder().method(v.method).uri(v.uri);
90
91        let headers = builder.headers_mut().expect("header map must be valid");
92        headers.extend(v.headers);
93
94        builder
95            .body(T::default())
96            .expect("request must build succeed")
97    }
98}
99
100/// Reply for `read` operation.
101#[derive(Debug, Clone, Default)]
102pub struct RpRead {
103    /// Size is the size of the reader returned by this read operation.
104    ///
105    /// - `Some(size)` means the reader has at most size bytes.
106    /// - `None` means the reader has unknown size.
107    ///
108    /// It's ok to leave size as empty, but it's recommended to set size if possible. We will use
109    /// this size as hint to do some optimization like avoid an extra stat or read.
110    size: Option<u64>,
111    /// Range is the range of the reader returned by this read operation.
112    ///
113    /// - `Some(range)` means the reader's content range inside the whole file.
114    /// - `None` means the reader's content range is unknown.
115    ///
116    /// It's ok to leave range as empty, but it's recommended to set range if possible. We will use
117    /// this range as hint to do some optimization like avoid an extra stat or read.
118    range: Option<BytesContentRange>,
119}
120
121impl RpRead {
122    /// Create a new reply for `read`.
123    pub fn new() -> Self {
124        RpRead::default()
125    }
126
127    /// Got the size of the reader returned by this read operation.
128    ///
129    /// - `Some(size)` means the reader has at most size bytes.
130    /// - `None` means the reader has unknown size.
131    pub fn size(&self) -> Option<u64> {
132        self.size
133    }
134
135    /// Set the size of the reader returned by this read operation.
136    pub fn with_size(mut self, size: Option<u64>) -> Self {
137        self.size = size;
138        self
139    }
140
141    /// Got the range of the reader returned by this read operation.
142    ///
143    /// - `Some(range)` means the reader has content range inside the whole file.
144    /// - `None` means the reader has unknown size.
145    pub fn range(&self) -> Option<BytesContentRange> {
146        self.range
147    }
148
149    /// Set the range of the reader returned by this read operation.
150    pub fn with_range(mut self, range: Option<BytesContentRange>) -> Self {
151        self.range = range;
152        self
153    }
154}
155
156/// Reply for `stat` operation.
157#[derive(Debug, Clone)]
158pub struct RpStat {
159    meta: Metadata,
160}
161
162impl RpStat {
163    /// Create a new reply for `stat`.
164    pub fn new(meta: Metadata) -> Self {
165        RpStat { meta }
166    }
167
168    /// Operate on inner metadata.
169    pub fn map_metadata(mut self, f: impl FnOnce(Metadata) -> Metadata) -> Self {
170        self.meta = f(self.meta);
171        self
172    }
173
174    /// Consume RpStat to get the inner metadata.
175    pub fn into_metadata(self) -> Metadata {
176        self.meta
177    }
178}
179
180/// Reply for `write` operation.
181#[derive(Debug, Clone, Default)]
182pub struct RpWrite {}
183
184impl RpWrite {
185    /// Create a new reply for `write`.
186    pub fn new() -> Self {
187        Self {}
188    }
189}
190
191/// Reply for `copy` operation.
192#[derive(Debug, Clone, Default)]
193pub struct RpCopy {}
194
195impl RpCopy {
196    /// Create a new reply for `copy`.
197    pub fn new() -> Self {
198        Self {}
199    }
200}
201
202/// Reply for `rename` operation.
203#[derive(Debug, Clone, Default)]
204pub struct RpRename {}
205
206impl RpRename {
207    /// Create a new reply for `rename`.
208    pub fn new() -> Self {
209        Self {}
210    }
211}
212
213#[cfg(test)]
214mod tests {
215    use anyhow::Result;
216    use http::HeaderMap;
217    use http::Method;
218    use http::Uri;
219    use http::header::CONTENT_LENGTH;
220    use http::header::CONTENT_TYPE;
221
222    use super::*;
223
224    #[test]
225    fn test_presigned_request_convert() -> Result<()> {
226        let pr = PresignedRequest {
227            method: Method::PATCH,
228            uri: Uri::from_static("https://opendal.apache.org/path/to/file"),
229            headers: {
230                let mut headers = HeaderMap::new();
231                headers.insert(CONTENT_LENGTH, "123".parse()?);
232                headers.insert(CONTENT_TYPE, "application/json".parse()?);
233
234                headers
235            },
236        };
237
238        let req: Request<Buffer> = pr.into();
239        assert_eq!(Method::PATCH, req.method());
240        assert_eq!(
241            "https://opendal.apache.org/path/to/file",
242            req.uri().to_string()
243        );
244        assert_eq!("123", req.headers().get(CONTENT_LENGTH).unwrap());
245        assert_eq!("application/json", req.headers().get(CONTENT_TYPE).unwrap());
246
247        Ok(())
248    }
249}
```
