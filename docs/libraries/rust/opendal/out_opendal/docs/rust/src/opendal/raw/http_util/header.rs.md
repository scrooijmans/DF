# 

opendal/raw/http_util/

header.rs

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
18use std::collections::HashMap;
19
20use base64::Engine;
21use base64::engine::general_purpose;
22use http::HeaderMap;
23use http::HeaderName;
24use http::HeaderValue;
25use http::header::CACHE_CONTROL;
26use http::header::CONTENT_DISPOSITION;
27use http::header::CONTENT_ENCODING;
28use http::header::CONTENT_LENGTH;
29use http::header::CONTENT_RANGE;
30use http::header::CONTENT_TYPE;
31use http::header::ETAG;
32use http::header::LAST_MODIFIED;
33use http::header::LOCATION;
34use md5::Digest;
35
36use crate::EntryMode;
37use crate::Error;
38use crate::ErrorKind;
39use crate::Metadata;
40use crate::Result;
41use crate::raw::*;
42
43/// Parse redirect location from header map
44///
45/// # Note
46/// The returned value maybe a relative path, like `/index.html`, `/robots.txt`, etc.
47pub fn parse_location(headers: &HeaderMap) -> Result<Option<&str>> {
48    parse_header_to_str(headers, LOCATION)
49}
50
51/// Parse cache control from header map.
52///
53/// # Note
54///
55/// The returned value is the raw string of `cache-control` header,
56/// maybe `no-cache`, `max-age=3600`, etc.
57pub fn parse_cache_control(headers: &HeaderMap) -> Result<Option<&str>> {
58    parse_header_to_str(headers, CACHE_CONTROL)
59}
60
61/// Parse content length from header map.
62pub fn parse_content_length(headers: &HeaderMap) -> Result<Option<u64>> {
63    parse_header_to_str(headers, CONTENT_LENGTH)?
64        .map(|v| {
65            v.parse::<u64>().map_err(|e| {
66                Error::new(ErrorKind::Unexpected, "header value is not valid integer").set_source(e)
67            })
68        })
69        .transpose()
70}
71
72/// Parse content md5 from header map.
73pub fn parse_content_md5(headers: &HeaderMap) -> Result<Option<&str>> {
74    parse_header_to_str(headers, "content-md5")
75}
76
77/// Parse content type from header map.
78pub fn parse_content_type(headers: &HeaderMap) -> Result<Option<&str>> {
79    parse_header_to_str(headers, CONTENT_TYPE)
80}
81
82/// Parse content encoding from header map.
83pub fn parse_content_encoding(headers: &HeaderMap) -> Result<Option<&str>> {
84    parse_header_to_str(headers, CONTENT_ENCODING)
85}
86
87/// Parse content range from header map.
88pub fn parse_content_range(headers: &HeaderMap) -> Result<Option<BytesContentRange>> {
89    parse_header_to_str(headers, CONTENT_RANGE)?
90        .map(|v| v.parse())
91        .transpose()
92}
93
94/// Parse last modified from header map.
95pub fn parse_last_modified(headers: &HeaderMap) -> Result<Option<Timestamp>> {
96    parse_header_to_str(headers, LAST_MODIFIED)?
97        .map(Timestamp::parse_rfc2822)
98        .transpose()
99}
100
101/// Parse etag from header map.
102pub fn parse_etag(headers: &HeaderMap) -> Result<Option<&str>> {
103    parse_header_to_str(headers, ETAG)
104}
105
106/// Parse Content-Disposition for header map
107pub fn parse_content_disposition(headers: &HeaderMap) -> Result<Option<&str>> {
108    parse_header_to_str(headers, CONTENT_DISPOSITION)
109}
110
111/// Parse multipart boundary from header map.
112pub fn parse_multipart_boundary(headers: &HeaderMap) -> Result<Option<&str>> {
113    parse_header_to_str(headers, CONTENT_TYPE).map(|v| v.and_then(|v| v.split("boundary=").nth(1)))
114}
115
116/// Parse header value to string according to name.
117#[inline]
118pub fn parse_header_to_str<K>(headers: &HeaderMap, name: K) -> Result<Option<&str>>
119where
120    HeaderName: TryFrom<K>,
121{
122    let name = HeaderName::try_from(name).map_err(|_| {
123        Error::new(
124            ErrorKind::Unexpected,
125            "header name must be valid http header name but not",
126        )
127        .with_operation("http_util::parse_header_to_str")
128    })?;
129
130    let value = if let Some(v) = headers.get(&name) {
131        v
132    } else {
133        return Ok(None);
134    };
135
136    Ok(Some(value.to_str().map_err(|e| {
137        Error::new(
138            ErrorKind::Unexpected,
139            "header value must be valid utf-8 string but not",
140        )
141        .with_operation("http_util::parse_header_to_str")
142        .with_context("header_name", name.as_str())
143        .set_source(e)
144    })?))
145}
146
147/// parse_into_metadata will parse standards http headers into Metadata.
148///
149/// # Notes
150///
151/// parse_into_metadata only handles the standard behavior of http
152/// headers. If services have their own logic, they should update the parsed
153/// metadata on demand.
154pub fn parse_into_metadata(path: &str, headers: &HeaderMap) -> Result<Metadata> {
155    let mode = if path.ends_with('/') {
156        EntryMode::DIR
157    } else {
158        EntryMode::FILE
159    };
160    let mut m = Metadata::new(mode);
161
162    if let Some(v) = parse_cache_control(headers)? {
163        m.set_cache_control(v);
164    }
165
166    if let Some(v) = parse_content_length(headers)? {
167        m.set_content_length(v);
168    }
169
170    if let Some(v) = parse_content_type(headers)? {
171        m.set_content_type(v);
172    }
173
174    if let Some(v) = parse_content_encoding(headers)? {
175        m.set_content_encoding(v);
176    }
177
178    if let Some(v) = parse_content_range(headers)? {
179        m.set_content_range(v);
180    }
181
182    if let Some(v) = parse_etag(headers)? {
183        m.set_etag(v);
184    }
185
186    if let Some(v) = parse_content_md5(headers)? {
187        m.set_content_md5(v);
188    }
189
190    if let Some(v) = parse_last_modified(headers)? {
191        m.set_last_modified(v);
192    }
193
194    if let Some(v) = parse_content_disposition(headers)? {
195        m.set_content_disposition(v);
196    }
197
198    Ok(m)
199}
200
201/// Parse prefixed headers and return a map with the prefix of each header removed.
202pub fn parse_prefixed_headers(headers: &HeaderMap, prefix: &str) -> HashMap<String, String> {
203    headers
204        .iter()
205        .filter_map(|(name, value)| {
206            name.as_str().strip_prefix(prefix).and_then(|stripped_key| {
207                value
208                    .to_str()
209                    .ok()
210                    .map(|parsed_value| (stripped_key.to_string(), parsed_value.to_string()))
211            })
212        })
213        .collect()
214}
215
216/// format content md5 header by given input.
217pub fn format_content_md5(bs: &[u8]) -> String {
218    let mut hasher = md5::Md5::new();
219    hasher.update(bs);
220
221    general_purpose::STANDARD.encode(hasher.finalize())
222}
223
224/// format authorization header by basic auth.
225///
226/// # Errors
227///
228/// If input username is empty, function will return an unexpected error.
229pub fn format_authorization_by_basic(username: &str, password: &str) -> Result<String> {
230    if username.is_empty() {
231        return Err(Error::new(
232            ErrorKind::Unexpected,
233            "can't build authorization header with empty username",
234        ));
235    }
236
237    let value = general_purpose::STANDARD.encode(format!("{username}:{password}"));
238
239    Ok(format!("Basic {value}"))
240}
241
242/// format authorization header by bearer token.
243///
244/// # Errors
245///
246/// If input token is empty, function will return an unexpected error.
247pub fn format_authorization_by_bearer(token: &str) -> Result<String> {
248    if token.is_empty() {
249        return Err(Error::new(
250            ErrorKind::Unexpected,
251            "can't build authorization header with empty token",
252        ));
253    }
254
255    Ok(format!("Bearer {token}"))
256}
257
258/// Build header value from given string.
259pub fn build_header_value(v: &str) -> Result<HeaderValue> {
260    HeaderValue::from_str(v).map_err(|e| {
261        Error::new(
262            ErrorKind::ConfigInvalid,
263            "header value contains invalid characters",
264        )
265        .with_operation("http_util::build_header_value")
266        .set_source(e)
267    })
268}
269
270#[cfg(test)]
271mod tests {
272    use super::*;
273
274    /// Test cases is from https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteObjects.html
275    #[test]
276    fn test_format_content_md5() {
277        let cases = vec![(
278            r#"<Delete>
279<Object>
280 <Key>sample1.txt</Key>
281 </Object>
282 <Object>
283   <Key>sample2.txt</Key>
284 </Object>
285 </Delete>"#,
286            "WOctCY1SS662e7ziElh4cw==",
287        )];
288
289        for (input, expected) in cases {
290            let actual = format_content_md5(input.as_bytes());
291
292            assert_eq!(actual, expected)
293        }
294    }
295
296    /// Test cases is borrowed from
297    ///
298    /// - RFC2617: https://datatracker.ietf.org/doc/html/rfc2617#section-2
299    /// - MDN: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization
300    #[test]
301    fn test_format_authorization_by_basic() {
302        let cases = vec![
303            ("aladdin", "opensesame", "Basic YWxhZGRpbjpvcGVuc2VzYW1l"),
304            ("aladdin", "", "Basic YWxhZGRpbjo="),
305            (
306                "Aladdin",
307                "open sesame",
308                "Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==",
309            ),
310            ("Aladdin", "", "Basic QWxhZGRpbjo="),
311        ];
312
313        for (username, password, expected) in cases {
314            let actual =
315                format_authorization_by_basic(username, password).expect("format must success");
316
317            assert_eq!(actual, expected)
318        }
319    }
320
321    /// Test cases is borrowed from
322    ///
323    /// - RFC6750: https://datatracker.ietf.org/doc/html/rfc6750
324    #[test]
325    fn test_format_authorization_by_bearer() {
326        let cases = vec![("mF_9.B5f-4.1JqM", "Bearer mF_9.B5f-4.1JqM")];
327
328        for (token, expected) in cases {
329            let actual = format_authorization_by_bearer(token).expect("format must success");
330
331            assert_eq!(actual, expected)
332        }
333    }
334
335    #[test]
336    fn test_parse_multipart_boundary() {
337        let cases = vec![
338            (
339                "multipart/mixed; boundary=gc0p4Jq0M2Yt08jU534c0p",
340                Some("gc0p4Jq0M2Yt08jU534c0p"),
341            ),
342            ("multipart/mixed", None),
343        ];
344
345        for (input, expected) in cases {
346            let mut headers = HeaderMap::new();
347            headers.insert(CONTENT_TYPE, HeaderValue::from_str(input).unwrap());
348
349            let actual = parse_multipart_boundary(&headers).expect("parse must success");
350
351            assert_eq!(actual, expected)
352        }
353    }
354}
```
