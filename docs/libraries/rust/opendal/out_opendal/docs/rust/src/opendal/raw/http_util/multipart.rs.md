# 

opendal/raw/http_util/

multipart.rs

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
18use std::mem;
19use std::str::FromStr;
20
21use bytes::Bytes;
22use bytes::BytesMut;
23use http::HeaderMap;
24use http::HeaderName;
25use http::HeaderValue;
26use http::Method;
27use http::Request;
28use http::Response;
29use http::StatusCode;
30use http::Uri;
31use http::Version;
32use http::header::CONTENT_DISPOSITION;
33use http::header::CONTENT_LENGTH;
34use http::header::CONTENT_TYPE;
35use http::uri::PathAndQuery;
36
37use super::new_request_build_error;
38use crate::*;
39
40/// Multipart is a builder for multipart/form-data.
41#[derive(Debug)]
42pub struct Multipart<T: Part> {
43    boundary: String,
44    parts: Vec<T>,
45}
46
47impl<T: Part> Default for Multipart<T> {
48    fn default() -> Self {
49        Self::new()
50    }
51}
52
53impl<T: Part> Multipart<T> {
54    /// Create a new multipart with random boundary.
55    pub fn new() -> Self {
56        Multipart {
57            boundary: format!("opendal-{}", uuid::Uuid::new_v4()),
58            parts: Vec::default(),
59        }
60    }
61
62    /// Set the boundary with given string.
63    pub fn with_boundary(mut self, boundary: &str) -> Self {
64        self.boundary = boundary.to_string();
65        self
66    }
67
68    /// Insert a part into multipart.
69    pub fn part(mut self, part: T) -> Self {
70        self.parts.push(part);
71        self
72    }
73
74    /// Into parts.
75    pub fn into_parts(self) -> Vec<T> {
76        self.parts
77    }
78
79    /// Parse a response with multipart body into Multipart.
80    pub fn parse(mut self, bs: Bytes) -> Result<Self> {
81        let s = String::from_utf8(bs.to_vec()).map_err(|err| {
82            Error::new(
83                ErrorKind::Unexpected,
84                "multipart response contains invalid utf-8 chars",
85            )
86            .set_source(err)
87        })?;
88
89        let parts = s
90            .split(format!("--{}", self.boundary).as_str())
91            .collect::<Vec<&str>>();
92
93        for part in parts {
94            if part.is_empty() || part.starts_with("--") {
95                continue;
96            }
97
98            self.parts.push(T::parse(part)?);
99        }
100
101        Ok(self)
102    }
103
104    pub(crate) fn build(self) -> Buffer {
105        let mut bufs = Vec::with_capacity(self.parts.len() + 2);
106
107        // Build pre part.
108        let mut bs = BytesMut::new();
109        bs.extend_from_slice(b"--");
110        bs.extend_from_slice(self.boundary.as_bytes());
111        bs.extend_from_slice(b"\r\n");
112        let pre_part = Buffer::from(bs.freeze());
113
114        // Write all parts.
115        for part in self.parts {
116            bufs.push(pre_part.clone());
117            bufs.push(part.format());
118        }
119
120        // Write the last boundary
121        let mut bs = BytesMut::new();
122        bs.extend_from_slice(b"--");
123        bs.extend_from_slice(self.boundary.as_bytes());
124        bs.extend_from_slice(b"--");
125        bs.extend_from_slice(b"\r\n");
126
127        // Build final part.
128        bufs.push(Buffer::from(bs.freeze()));
129
130        bufs.into_iter().flatten().collect()
131    }
132
133    /// Consume the input and generate a request with multipart body.
134    ///
135    /// This function will make sure content_type and content_length set correctly.
136    pub fn apply(self, mut builder: http::request::Builder) -> Result<Request<Buffer>> {
137        let boundary = self.boundary.clone();
138        let buf = self.build();
139        let content_length = buf.len();
140
141        // Insert content type with correct boundary.
142        builder = builder.header(
143            CONTENT_TYPE,
144            format!("multipart/{}; boundary={}", T::TYPE, boundary).as_str(),
145        );
146        // Insert content length with calculated size.
147        builder = builder.header(CONTENT_LENGTH, content_length);
148
149        builder.body(buf).map_err(new_request_build_error)
150    }
151}
152
153/// Part is a trait for multipart part.
154pub trait Part: Sized + 'static {
155    /// TYPE is the type of multipart.
156    ///
157    /// Current available types are: `form-data` and `mixed`
158    const TYPE: &'static str;
159
160    /// format will generates the bytes.
161    fn format(self) -> Buffer;
162
163    /// parse will parse the bytes into a part.
164    fn parse(s: &str) -> Result<Self>;
165}
166
167/// FormDataPart is a builder for multipart/form-data part.
168pub struct FormDataPart {
169    headers: HeaderMap,
170
171    content: Buffer,
172}
173
174impl FormDataPart {
175    /// Create a new part builder
176    ///
177    /// # Panics
178    ///
179    /// Input name must be percent encoded.
180    pub fn new(name: &str) -> Self {
181        // Insert content disposition header for part.
182        let mut headers = HeaderMap::new();
183        headers.insert(
184            CONTENT_DISPOSITION,
185            format!("form-data; name=\"{name}\"").parse().unwrap(),
186        );
187
188        Self {
189            headers,
190            content: Buffer::new(),
191        }
192    }
193
194    /// Insert a header into part.
195    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
196        self.headers.insert(key, value);
197        self
198    }
199
200    /// Set the content for this part.
201    pub fn content(mut self, content: impl Into<Buffer>) -> Self {
202        self.content = content.into();
203        self
204    }
205}
206
207impl Part for FormDataPart {
208    const TYPE: &'static str = "form-data";
209
210    fn format(self) -> Buffer {
211        let mut bufs = Vec::with_capacity(3);
212
213        // Building pre-content.
214        let mut bs = BytesMut::new();
215        for (k, v) in self.headers.iter() {
216            // Trick!
217            //
218            // Seafile could not recognize header names like `content-disposition`
219            // and requires to use `Content-Disposition`. So we hardcode the part
220            // headers name here.
221            match k.as_str() {
222                "content-disposition" => {
223                    bs.extend_from_slice("Content-Disposition".as_bytes());
224                }
225                _ => {
226                    bs.extend_from_slice(k.as_str().as_bytes());
227                }
228            }
229            bs.extend_from_slice(b": ");
230            bs.extend_from_slice(v.as_bytes());
231            bs.extend_from_slice(b"\r\n");
232        }
233        bs.extend_from_slice(b"\r\n");
234        bufs.push(Buffer::from(bs.freeze()));
235
236        // Building content.
237        bufs.push(self.content);
238
239        // Building post-content.
240        bufs.push(Buffer::from("\r\n"));
241
242        bufs.into_iter().flatten().collect()
243    }
244
245    fn parse(_: &str) -> Result<Self> {
246        Err(Error::new(
247            ErrorKind::Unsupported,
248            "parse of form-data is not supported",
249        ))
250    }
251}
252
253/// MixedPart is a builder for multipart/mixed part.
254pub struct MixedPart {
255    part_headers: HeaderMap,
256
257    /// Common
258    version: Version,
259    headers: HeaderMap,
260    content: Buffer,
261
262    /// Request only
263    method: Option<Method>,
264    uri: Option<Uri>,
265
266    /// Response only
267    status_code: Option<StatusCode>,
268}
269
270impl MixedPart {
271    /// Create a new mixed part with given uri.
272    pub fn new(uri: &str) -> Self {
273        let mut part_headers = HeaderMap::new();
274        part_headers.insert(CONTENT_TYPE, "application/http".parse().unwrap());
275        part_headers.insert("content-transfer-encoding", "binary".parse().unwrap());
276
277        let uri = Uri::from_str(uri).expect("the uri used to build a mixed part must be valid");
278
279        Self {
280            part_headers,
281
282            version: Version::HTTP_11,
283            headers: HeaderMap::new(),
284            content: Buffer::new(),
285
286            uri: Some(uri),
287            method: None,
288
289            status_code: None,
290        }
291    }
292
293    /// Build a mixed part from a request.
294    pub fn from_request(req: Request<Buffer>) -> Self {
295        let mut part_headers = HeaderMap::new();
296        part_headers.insert(CONTENT_TYPE, "application/http".parse().unwrap());
297        part_headers.insert("content-transfer-encoding", "binary".parse().unwrap());
298
299        let (parts, content) = req.into_parts();
300
301        Self {
302            part_headers,
303            uri: Some(
304                Uri::from_str(
305                    parts
306                        .uri
307                        .path_and_query()
308                        .unwrap_or(&PathAndQuery::from_static("/"))
309                        .as_str(),
310                )
311                .expect("the uri used to build a mixed part must be valid"),
312            ),
313            version: parts.version,
314            headers: parts.headers,
315            content,
316
317            method: Some(parts.method),
318            status_code: None,
319        }
320    }
321
322    /// Consume a mixed part to build a response.
323    pub fn into_response(mut self) -> Response<Buffer> {
324        let mut builder = Response::builder();
325
326        builder = builder.status(self.status_code.unwrap_or(StatusCode::OK));
327        builder = builder.version(self.version);
328        // Swap headers directly instead of copy the entire map.
329        mem::swap(builder.headers_mut().unwrap(), &mut self.headers);
330
331        builder
332            .body(self.content)
333            .expect("mixed part must be valid response")
334    }
335
336    /// Insert a part header into part.
337    pub fn part_header(mut self, key: HeaderName, value: HeaderValue) -> Self {
338        self.part_headers.insert(key, value);
339        self
340    }
341
342    /// Set the method for request in this part.
343    pub fn method(mut self, method: Method) -> Self {
344        self.method = Some(method);
345        self
346    }
347
348    /// Set the version for request in this part.
349    pub fn version(mut self, version: Version) -> Self {
350        self.version = version;
351        self
352    }
353
354    /// Insert a header into part.
355    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
356        self.headers.insert(key, value);
357        self
358    }
359
360    /// Set the content for this part.
361    pub fn content(mut self, content: impl Into<Buffer>) -> Self {
362        self.content = content.into();
363        self
364    }
365}
366
367impl Part for MixedPart {
368    const TYPE: &'static str = "mixed";
369
370    fn format(self) -> Buffer {
371        let mut bufs = Vec::with_capacity(3);
372
373        // Write parts headers.
374        let mut bs = BytesMut::new();
375        for (k, v) in self.part_headers.iter() {
376            // Trick!
377            //
378            // Azblob could not recognize header names like `content-type`
379            // and requires to use `Content-Type`. So we hardcode the part
380            // headers name here.
381            match k.as_str() {
382                "content-type" => {
383                    bs.extend_from_slice("Content-Type".as_bytes());
384                }
385                "content-id" => {
386                    bs.extend_from_slice("Content-ID".as_bytes());
387                }
388                "content-transfer-encoding" => {
389                    bs.extend_from_slice("Content-Transfer-Encoding".as_bytes());
390                }
391                _ => {
392                    bs.extend_from_slice(k.as_str().as_bytes());
393                }
394            }
395            bs.extend_from_slice(b": ");
396            bs.extend_from_slice(v.as_bytes());
397            bs.extend_from_slice(b"\r\n");
398        }
399
400        // Write request line: `DELETE /container0/blob0 HTTP/1.1`
401        bs.extend_from_slice(b"\r\n");
402        bs.extend_from_slice(
403            self.method
404                .as_ref()
405                .expect("mixed part must be a valid request that contains method")
406                .as_str()
407                .as_bytes(),
408        );
409        bs.extend_from_slice(b" ");
410        bs.extend_from_slice(
411            self.uri
412                .as_ref()
413                .expect("mixed part must be a valid request that contains uri")
414                .path()
415                .as_bytes(),
416        );
417        bs.extend_from_slice(b" ");
418        bs.extend_from_slice(format!("{:?}", self.version).as_bytes());
419        bs.extend_from_slice(b"\r\n");
420
421        // Write request headers.
422        for (k, v) in self.headers.iter() {
423            bs.extend_from_slice(k.as_str().as_bytes());
424            bs.extend_from_slice(b": ");
425            bs.extend_from_slice(v.as_bytes());
426            bs.extend_from_slice(b"\r\n");
427        }
428        bs.extend_from_slice(b"\r\n");
429        bufs.push(Buffer::from(bs.freeze()));
430
431        if !self.content.is_empty() {
432            bufs.push(self.content);
433            bufs.push(Buffer::from("\r\n"))
434        }
435
436        bufs.into_iter().flatten().collect()
437    }
438
439    /// TODO
440    ///
441    /// This is a simple implementation and have a lot of space to improve.
442    fn parse(s: &str) -> Result<Self> {
443        let parts = s.splitn(2, "\r\n\r\n").collect::<Vec<&str>>();
444        let part_headers_content = parts[0];
445        let http_response = parts.get(1).unwrap_or(&"");
446
447        let mut part_headers = HeaderMap::new();
448        for line in part_headers_content.lines() {
449            let parts = line.splitn(2, ": ").collect::<Vec<&str>>();
450            if parts.len() == 2 {
451                let header_name = HeaderName::from_str(parts[0]).map_err(|err| {
452                    Error::new(
453                        ErrorKind::Unexpected,
454                        "multipart response contains invalid part header name",
455                    )
456                    .set_source(err)
457                })?;
458                let header_value = parts[1].parse().map_err(|err| {
459                    Error::new(
460                        ErrorKind::Unexpected,
461                        "multipart response contains invalid part header value",
462                    )
463                    .set_source(err)
464                })?;
465
466                part_headers.insert(header_name, header_value);
467            }
468        }
469
470        let parts = http_response.split("\r\n\r\n").collect::<Vec<&str>>();
471        let headers_content = parts[0];
472        let body_content = parts.get(1).unwrap_or(&"");
473        let body_bytes = Buffer::from(body_content.to_string());
474
475        let status_line = headers_content.lines().next().unwrap_or("");
476        let status_code = status_line
477            .split_whitespace()
478            .nth(1)
479            .unwrap_or("")
480            .parse::<u16>()
481            .unwrap_or(200);
482
483        let mut headers = HeaderMap::new();
484        for line in headers_content.lines().skip(1) {
485            let parts = line.splitn(2, ": ").collect::<Vec<&str>>();
486            if parts.len() == 2 {
487                let header_name = HeaderName::from_str(parts[0]).map_err(|err| {
488                    Error::new(
489                        ErrorKind::Unexpected,
490                        "multipart response contains invalid part header name",
491                    )
492                    .set_source(err)
493                })?;
494                let header_value = parts[1].parse().map_err(|err| {
495                    Error::new(
496                        ErrorKind::Unexpected,
497                        "multipart response contains invalid part header value",
498                    )
499                    .set_source(err)
500                })?;
501
502                headers.insert(header_name, header_value);
503            }
504        }
505
506        Ok(Self {
507            part_headers,
508            version: Version::HTTP_11,
509            headers,
510            content: body_bytes,
511
512            method: None,
513            uri: None,
514
515            status_code: Some(StatusCode::from_u16(status_code).map_err(|err| {
516                Error::new(
517                    ErrorKind::Unexpected,
518                    "multipart response contains invalid status code",
519                )
520                .set_source(err)
521            })?),
522        })
523    }
524}
525
526/// RelatedPart is a builder for multipart/related part.
527pub struct RelatedPart {
528    /// Common
529    headers: HeaderMap,
530    content: Buffer,
531}
532
533impl Default for RelatedPart {
534    fn default() -> Self {
535        Self::new()
536    }
537}
538
539impl RelatedPart {
540    /// Create a new related
541    pub fn new() -> Self {
542        Self {
543            headers: HeaderMap::new(),
544            content: Buffer::new(),
545        }
546    }
547
548    /// Build a mixed part from a request.
549    pub fn from_request(req: Request<Buffer>) -> Self {
550        let (parts, content) = req.into_parts();
551
552        Self {
553            headers: parts.headers,
554            content,
555        }
556    }
557
558    /// Consume a mixed part to build a response.
559    pub fn into_response(mut self) -> Response<Buffer> {
560        let mut builder = Response::builder();
561
562        // Swap headers directly instead of copy the entire map.
563        mem::swap(builder.headers_mut().unwrap(), &mut self.headers);
564
565        builder
566            .body(self.content)
567            .expect("a related part must be valid response")
568    }
569
570    /// Insert a header into part.
571    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
572        self.headers.insert(key, value);
573        self
574    }
575
576    /// Set the content for this part.
577    pub fn content(mut self, content: impl Into<Buffer>) -> Self {
578        self.content = content.into();
579        self
580    }
581}
582
583impl Part for RelatedPart {
584    const TYPE: &'static str = "related";
585
586    fn format(self) -> Buffer {
587        // This is what multipart/related body might look for an insert in GCS
588        // https://cloud.google.com/storage/docs/uploading-objects
589        /*
590        --separator_string
591        Content-Type: application/json; charset=UTF-8
592
593        {"name":"my-document.txt"}
594
595        --separator_string
596        Content-Type: text/plain
597
598        This is a text file.
599        --separator_string--
600        */
601
602        let mut bufs = Vec::with_capacity(3);
603        let mut bs = BytesMut::new();
604
605        // Write request headers.
606        for (k, v) in self.headers.iter() {
607            bs.extend_from_slice(k.as_str().as_bytes());
608            bs.extend_from_slice(b": ");
609            bs.extend_from_slice(v.as_bytes());
610            bs.extend_from_slice(b"\r\n");
611        }
612        bs.extend_from_slice(b"\r\n");
613        bufs.push(Buffer::from(bs.freeze()));
614
615        if !self.content.is_empty() {
616            bufs.push(self.content);
617            bufs.push(Buffer::from("\r\n"))
618        }
619
620        bufs.into_iter().flatten().collect()
621    }
622
623    fn parse(_s: &str) -> Result<Self> {
624        Err(Error::new(
625            ErrorKind::Unsupported,
626            "parsing multipart/related is not supported",
627        ))
628    }
629}
630
631#[cfg(test)]
632mod tests {
633    use http::header::CONTENT_TYPE;
634    use pretty_assertions::assert_eq;
635
636    use super::*;
637
638    #[test]
639    fn test_multipart_formdata_basic() -> Result<()> {
640        let multipart = Multipart::new()
641            .with_boundary("lalala")
642            .part(FormDataPart::new("foo").content(Bytes::from("bar")))
643            .part(FormDataPart::new("hello").content(Bytes::from("world")));
644
645        let bs = multipart.build();
646
647        let expected = "--lalala\r\n\
648             Content-Disposition: form-data; name=\"foo\"\r\n\
649             \r\n\
650             bar\r\n\
651             --lalala\r\n\
652             Content-Disposition: form-data; name=\"hello\"\r\n\
653             \r\n\
654             world\r\n\
655             --lalala--\r\n";
656
657        assert_eq!(Bytes::from(expected), bs.to_bytes());
658        Ok(())
659    }
660
661    /// This test is inspired by <https://docs.aws.amazon.com/AmazonS3/latest/userguide/HTTPPOSTExamples.html>
662    #[test]
663    fn test_multipart_formdata_s3_form_upload() -> Result<()> {
664        let multipart = Multipart::new()
665            .with_boundary("9431149156168")
666            .part(FormDataPart::new("key").content("user/eric/MyPicture.jpg"))
667            .part(FormDataPart::new("acl").content("public-read"))
668            .part(FormDataPart::new("success_action_redirect").content(
669                "https://awsexamplebucket1.s3.us-west-1.amazonaws.com/successful_upload.html",
670            ))
671            .part(FormDataPart::new("content-type").content("image/jpeg"))
672            .part(FormDataPart::new("x-amz-meta-uuid").content("14365123651274"))
673            .part(FormDataPart::new("x-amz-meta-tag").content("Some,Tag,For,Picture"))
674            .part(FormDataPart::new("AWSAccessKeyId").content("AKIAIOSFODNN7EXAMPLE"))
675            .part(FormDataPart::new("Policy").content("eyAiZXhwaXJhdGlvbiI6ICIyMDA3LTEyLTAxVDEyOjAwOjAwLjAwMFoiLAogICJjb25kaXRpb25zIjogWwogICAgeyJidWNrZXQiOiAiam9obnNtaXRoIn0sCiAgICBbInN0YXJ0cy13aXRoIiwgIiRrZXkiLCAidXNlci9lcmljLyJdLAogICAgeyJhY2wiOiAicHVibGljLXJlYWQifSwKICAgIHsic3VjY2Vzc19hY3Rpb25fcmVkaXJlY3QiOiAiaHR0cDovL2pvaG5zbWl0aC5zMy5hbWF6b25hd3MuY29tL3N1Y2Nlc3NmdWxfdXBsb2FkLmh0bWwifSwKICAgIFsic3RhcnRzLXdpdGgiLCAiJENvbnRlbnQtVHlwZSIsICJpbWFnZS8iXSwKICAgIHsieC1hbXotbWV0YS11dWlkIjogIjE0MzY1MTIzNjUxMjc0In0sCiAgICBbInN0YXJ0cy13aXRoIiwgIiR4LWFtei1tZXRhLXRhZyIsICIiXQogIF0KfQo="))
676            .part(FormDataPart::new("Signature").content("0RavWzkygo6QX9caELEqKi9kDbU="))
677            .part(FormDataPart::new("file").header(CONTENT_TYPE, "image/jpeg".parse().unwrap()).content("...file content...")).part(FormDataPart::new("submit").content("Upload to Amazon S3"));
678
679        let bs = multipart.build();
680
681        let expected = r#"--9431149156168
682Content-Disposition: form-data; name="key"
683
684user/eric/MyPicture.jpg
685--9431149156168
686Content-Disposition: form-data; name="acl"
687
688public-read
689--9431149156168
690Content-Disposition: form-data; name="success_action_redirect"
691
692https://awsexamplebucket1.s3.us-west-1.amazonaws.com/successful_upload.html
693--9431149156168
694Content-Disposition: form-data; name="content-type"
695
696image/jpeg
697--9431149156168
698Content-Disposition: form-data; name="x-amz-meta-uuid"
699
70014365123651274
701--9431149156168
702Content-Disposition: form-data; name="x-amz-meta-tag"
703
704Some,Tag,For,Picture
705--9431149156168
706Content-Disposition: form-data; name="AWSAccessKeyId"
707
708AKIAIOSFODNN7EXAMPLE
709--9431149156168
710Content-Disposition: form-data; name="Policy"
711
712eyAiZXhwaXJhdGlvbiI6ICIyMDA3LTEyLTAxVDEyOjAwOjAwLjAwMFoiLAogICJjb25kaXRpb25zIjogWwogICAgeyJidWNrZXQiOiAiam9obnNtaXRoIn0sCiAgICBbInN0YXJ0cy13aXRoIiwgIiRrZXkiLCAidXNlci9lcmljLyJdLAogICAgeyJhY2wiOiAicHVibGljLXJlYWQifSwKICAgIHsic3VjY2Vzc19hY3Rpb25fcmVkaXJlY3QiOiAiaHR0cDovL2pvaG5zbWl0aC5zMy5hbWF6b25hd3MuY29tL3N1Y2Nlc3NmdWxfdXBsb2FkLmh0bWwifSwKICAgIFsic3RhcnRzLXdpdGgiLCAiJENvbnRlbnQtVHlwZSIsICJpbWFnZS8iXSwKICAgIHsieC1hbXotbWV0YS11dWlkIjogIjE0MzY1MTIzNjUxMjc0In0sCiAgICBbInN0YXJ0cy13aXRoIiwgIiR4LWFtei1tZXRhLXRhZyIsICIiXQogIF0KfQo=
713--9431149156168
714Content-Disposition: form-data; name="Signature"
715
7160RavWzkygo6QX9caELEqKi9kDbU=
717--9431149156168
718Content-Disposition: form-data; name="file"
719content-type: image/jpeg
720
721...file content...
722--9431149156168
723Content-Disposition: form-data; name="submit"
724
725Upload to Amazon S3
726--9431149156168--
727"#;
728
729        assert_eq!(
730            expected,
731            // Rust can't represent `\r` in a string literal, so we
732            // replace `\r\n` with `\n` for comparison
733            String::from_utf8(bs.to_bytes().to_vec())
734                .unwrap()
735                .replace("\r\n", "\n")
736        );
737
738        Ok(())
739    }
740
741    /// This test is inspired by <https://cloud.google.com/storage/docs/batch>
742    #[test]
743    fn test_multipart_mixed_gcs_batch_metadata() -> Result<()> {
744        let multipart = Multipart::new()
745            .with_boundary("===============7330845974216740156==")
746            .part(
747                MixedPart::new("/storage/v1/b/example-bucket/o/obj1")
748                    .method(Method::PATCH)
749                    .part_header(
750                        "content-id".parse().unwrap(),
751                        "<b29c5de2-0db4-490b-b421-6a51b598bd22+1>".parse().unwrap(),
752                    )
753                    .header(
754                        "content-type".parse().unwrap(),
755                        "application/json".parse().unwrap(),
756                    )
757                    .header(
758                        "accept".parse().unwrap(),
759                        "application/json".parse().unwrap(),
760                    )
761                    .header("content-length".parse().unwrap(), "31".parse().unwrap())
762                    .content(r#"{"metadata": {"type": "tabby"}}"#),
763            )
764            .part(
765                MixedPart::new("/storage/v1/b/example-bucket/o/obj2")
766                    .method(Method::PATCH)
767                    .part_header(
768                        "content-id".parse().unwrap(),
769                        "<b29c5de2-0db4-490b-b421-6a51b598bd22+2>".parse().unwrap(),
770                    )
771                    .header(
772                        "content-type".parse().unwrap(),
773                        "application/json".parse().unwrap(),
774                    )
775                    .header(
776                        "accept".parse().unwrap(),
777                        "application/json".parse().unwrap(),
778                    )
779                    .header("content-length".parse().unwrap(), "32".parse().unwrap())
780                    .content(r#"{"metadata": {"type": "tuxedo"}}"#),
781            )
782            .part(
783                MixedPart::new("/storage/v1/b/example-bucket/o/obj3")
784                    .method(Method::PATCH)
785                    .part_header(
786                        "content-id".parse().unwrap(),
787                        "<b29c5de2-0db4-490b-b421-6a51b598bd22+3>".parse().unwrap(),
788                    )
789                    .header(
790                        "content-type".parse().unwrap(),
791                        "application/json".parse().unwrap(),
792                    )
793                    .header(
794                        "accept".parse().unwrap(),
795                        "application/json".parse().unwrap(),
796                    )
797                    .header("content-length".parse().unwrap(), "32".parse().unwrap())
798                    .content(r#"{"metadata": {"type": "calico"}}"#),
799            );
800
801        let bs = multipart.build();
802
803        let expected = r#"--===============7330845974216740156==
804Content-Type: application/http
805Content-Transfer-Encoding: binary
806Content-ID: <b29c5de2-0db4-490b-b421-6a51b598bd22+1>
807
808PATCH /storage/v1/b/example-bucket/o/obj1 HTTP/1.1
809content-type: application/json
810accept: application/json
811content-length: 31
812
813{"metadata": {"type": "tabby"}}
814--===============7330845974216740156==
815Content-Type: application/http
816Content-Transfer-Encoding: binary
817Content-ID: <b29c5de2-0db4-490b-b421-6a51b598bd22+2>
818
819PATCH /storage/v1/b/example-bucket/o/obj2 HTTP/1.1
820content-type: application/json
821accept: application/json
822content-length: 32
823
824{"metadata": {"type": "tuxedo"}}
825--===============7330845974216740156==
826Content-Type: application/http
827Content-Transfer-Encoding: binary
828Content-ID: <b29c5de2-0db4-490b-b421-6a51b598bd22+3>
829
830PATCH /storage/v1/b/example-bucket/o/obj3 HTTP/1.1
831content-type: application/json
832accept: application/json
833content-length: 32
834
835{"metadata": {"type": "calico"}}
836--===============7330845974216740156==--
837"#;
838
839        assert_eq!(
840            expected,
841            // Rust can't represent `\r` in a string literal, so we
842            // replace `\r\n` with `\n` for comparison
843            String::from_utf8(bs.to_bytes().to_vec())
844                .unwrap()
845                .replace("\r\n", "\n")
846        );
847
848        Ok(())
849    }
850
851    /// This test is inspired by <https://learn.microsoft.com/en-us/rest/api/storageservices/blob-batch?tabs=azure-ad>
852    #[test]
853    fn test_multipart_mixed_azblob_batch_delete() -> Result<()> {
854        let multipart = Multipart::new()
855            .with_boundary("batch_357de4f7-6d0b-4e02-8cd2-6361411a9525")
856            .part(
857                MixedPart::new("/container0/blob0")
858                    .method(Method::DELETE)
859                    .part_header("content-id".parse().unwrap(), "0".parse().unwrap())
860                    .header(
861                        "x-ms-date".parse().unwrap(),
862                        "Thu, 14 Jun 2018 16:46:54 GMT".parse().unwrap(),
863                    )
864                    .header(
865                        "authorization".parse().unwrap(),
866                        "SharedKey account:G4jjBXA7LI/RnWKIOQ8i9xH4p76pAQ+4Fs4R1VxasaE="
867                            .parse()
868                            .unwrap(),
869                    )
870                    .header("content-length".parse().unwrap(), "0".parse().unwrap()),
871            )
872            .part(
873                MixedPart::new("/container1/blob1")
874                    .method(Method::DELETE)
875                    .part_header("content-id".parse().unwrap(), "1".parse().unwrap())
876                    .header(
877                        "x-ms-date".parse().unwrap(),
878                        "Thu, 14 Jun 2018 16:46:54 GMT".parse().unwrap(),
879                    )
880                    .header(
881                        "authorization".parse().unwrap(),
882                        "SharedKey account:IvCoYDQ+0VcaA/hKFjUmQmIxXv2RT3XwwTsOTHL39HI="
883                            .parse()
884                            .unwrap(),
885                    )
886                    .header("content-length".parse().unwrap(), "0".parse().unwrap()),
887            )
888            .part(
889                MixedPart::new("/container2/blob2")
890                    .method(Method::DELETE)
891                    .part_header("content-id".parse().unwrap(), "2".parse().unwrap())
892                    .header(
893                        "x-ms-date".parse().unwrap(),
894                        "Thu, 14 Jun 2018 16:46:54 GMT".parse().unwrap(),
895                    )
896                    .header(
897                        "authorization".parse().unwrap(),
898                        "SharedKey account:S37N2JTjcmOQVLHLbDmp2johz+KpTJvKhbVc4M7+UqI="
899                            .parse()
900                            .unwrap(),
901                    )
902                    .header("content-length".parse().unwrap(), "0".parse().unwrap()),
903            );
904
905        let bs = multipart.build();
906
907        let expected = r#"--batch_357de4f7-6d0b-4e02-8cd2-6361411a9525
908Content-Type: application/http
909Content-Transfer-Encoding: binary
910Content-ID: 0
911
912DELETE /container0/blob0 HTTP/1.1
913x-ms-date: Thu, 14 Jun 2018 16:46:54 GMT
914authorization: SharedKey account:G4jjBXA7LI/RnWKIOQ8i9xH4p76pAQ+4Fs4R1VxasaE=
915content-length: 0
916
917--batch_357de4f7-6d0b-4e02-8cd2-6361411a9525
918Content-Type: application/http
919Content-Transfer-Encoding: binary
920Content-ID: 1
921
922DELETE /container1/blob1 HTTP/1.1
923x-ms-date: Thu, 14 Jun 2018 16:46:54 GMT
924authorization: SharedKey account:IvCoYDQ+0VcaA/hKFjUmQmIxXv2RT3XwwTsOTHL39HI=
925content-length: 0
926
927--batch_357de4f7-6d0b-4e02-8cd2-6361411a9525
928Content-Type: application/http
929Content-Transfer-Encoding: binary
930Content-ID: 2
931
932DELETE /container2/blob2 HTTP/1.1
933x-ms-date: Thu, 14 Jun 2018 16:46:54 GMT
934authorization: SharedKey account:S37N2JTjcmOQVLHLbDmp2johz+KpTJvKhbVc4M7+UqI=
935content-length: 0
936
937--batch_357de4f7-6d0b-4e02-8cd2-6361411a9525--
938"#;
939
940        assert_eq!(
941            expected,
942            // Rust can't represent `\r` in a string literal, so we
943            // replace `\r\n` with `\n` for comparison
944            String::from_utf8(bs.to_bytes().to_vec())
945                .unwrap()
946                .replace("\r\n", "\n")
947        );
948
949        Ok(())
950    }
951
952    /// This test is inspired by <https://cloud.google.com/storage/docs/batch>
953    #[test]
954    fn test_multipart_mixed_gcs_batch_metadata_response() {
955        let response = r#"--batch_pK7JBAk73-E=_AA5eFwv4m2Q=
956Content-Type: application/http
957Content-ID: <response-b29c5de2-0db4-490b-b421-6a51b598bd22+1>
958
959HTTP/1.1 200 OK
960ETag: "lGaP-E0memYDumK16YuUDM_6Gf0/V43j6azD55CPRGb9b6uytDYl61Y"
961Content-Type: application/json; charset=UTF-8
962Date: Mon, 22 Jan 2018 18:56:00 GMT
963Expires: Mon, 22 Jan 2018 18:56:00 GMT
964Cache-Control: private, max-age=0
965Content-Length: 846
966
967{"kind": "storage#object","id": "example-bucket/obj1/1495822576643790","metadata": {"type": "tabby"}}
968
969--batch_pK7JBAk73-E=_AA5eFwv4m2Q=
970Content-Type: application/http
971Content-ID: <response-b29c5de2-0db4-490b-b421-6a51b598bd22+2>
972
973HTTP/1.1 200 OK
974ETag: "lGaP-E0memYDumK16YuUDM_6Gf0/91POdd-sxSAkJnS8Dm7wMxBSDKk"
975Content-Type: application/json; charset=UTF-8
976Date: Mon, 22 Jan 2018 18:56:00 GMT
977Expires: Mon, 22 Jan 2018 18:56:00 GMT
978Cache-Control: private, max-age=0
979Content-Length: 846
980
981{"kind": "storage#object","id": "example-bucket/obj2/1495822576643790","metadata": {"type": "tuxedo"}}
982
983--batch_pK7JBAk73-E=_AA5eFwv4m2Q=
984Content-Type: application/http
985Content-ID: <response-b29c5de2-0db4-490b-b421-6a51b598bd22+3>
986
987HTTP/1.1 200 OK
988ETag: "lGaP-E0memYDumK16YuUDM_6Gf0/d2Z1F1_ZVbB1dC0YKM9rX5VAgIQ"
989Content-Type: application/json; charset=UTF-8
990Date: Mon, 22 Jan 2018 18:56:00 GMT
991Expires: Mon, 22 Jan 2018 18:56:00 GMT
992Cache-Control: private, max-age=0
993Content-Length: 846
994
995{"kind": "storage#object","id": "example-bucket/obj3/1495822576643790","metadata": {"type": "calico"}}
996
997--batch_pK7JBAk73-E=_AA5eFwv4m2Q=--"#.replace('\n', "\r\n");
998
999        let multipart: Multipart<MixedPart> = Multipart::new()
1000            .with_boundary("batch_pK7JBAk73-E=_AA5eFwv4m2Q=")
1001            .parse(Bytes::from(response))
1002            .unwrap();
1003
1004        let part0_bs = Bytes::from_static(
1005            r#"{"kind": "storage#object","id": "example-bucket/obj1/1495822576643790","metadata": {"type": "tabby"}}"#.as_bytes());
1006        let part1_bs = Bytes::from_static(
1007            r#"{"kind": "storage#object","id": "example-bucket/obj2/1495822576643790","metadata": {"type": "tuxedo"}}"#
1008                .as_bytes()
1009        );
1010        let part2_bs = Bytes::from_static(
1011            r#"{"kind": "storage#object","id": "example-bucket/obj3/1495822576643790","metadata": {"type": "calico"}}"#
1012                .as_bytes()
1013        );
1014
1015        assert_eq!(multipart.parts.len(), 3);
1016
1017        assert_eq!(multipart.parts[0].part_headers, {
1018            let mut h = HeaderMap::new();
1019            h.insert("Content-Type", "application/http".parse().unwrap());
1020            h.insert(
1021                "Content-ID",
1022                "<response-b29c5de2-0db4-490b-b421-6a51b598bd22+1>"
1023                    .parse()
1024                    .unwrap(),
1025            );
1026
1027            h
1028        });
1029        assert_eq!(multipart.parts[0].version, Version::HTTP_11);
1030        assert_eq!(multipart.parts[0].headers, {
1031            let mut h = HeaderMap::new();
1032            h.insert(
1033                "ETag",
1034                "\"lGaP-E0memYDumK16YuUDM_6Gf0/V43j6azD55CPRGb9b6uytDYl61Y\""
1035                    .parse()
1036                    .unwrap(),
1037            );
1038            h.insert(
1039                "Content-Type",
1040                "application/json; charset=UTF-8".parse().unwrap(),
1041            );
1042            h.insert("Date", "Mon, 22 Jan 2018 18:56:00 GMT".parse().unwrap());
1043            h.insert("Expires", "Mon, 22 Jan 2018 18:56:00 GMT".parse().unwrap());
1044            h.insert("Cache-Control", "private, max-age=0".parse().unwrap());
1045            h.insert("Content-Length", "846".parse().unwrap());
1046
1047            h
1048        });
1049        assert_eq!(multipart.parts[0].content.len(), part0_bs.len());
1050        assert_eq!(multipart.parts[0].uri, None);
1051        assert_eq!(multipart.parts[0].method, None);
1052        assert_eq!(
1053            multipart.parts[0].status_code,
1054            Some(StatusCode::from_u16(200).unwrap())
1055        );
1056
1057        assert_eq!(multipart.parts[1].part_headers, {
1058            let mut h = HeaderMap::new();
1059            h.insert("Content-Type", "application/http".parse().unwrap());
1060            h.insert(
1061                "Content-ID",
1062                "<response-b29c5de2-0db4-490b-b421-6a51b598bd22+2>"
1063                    .parse()
1064                    .unwrap(),
1065            );
1066
1067            h
1068        });
1069        assert_eq!(multipart.parts[1].version, Version::HTTP_11);
1070        assert_eq!(multipart.parts[1].headers, {
1071            let mut h = HeaderMap::new();
1072            h.insert(
1073                "ETag",
1074                "\"lGaP-E0memYDumK16YuUDM_6Gf0/91POdd-sxSAkJnS8Dm7wMxBSDKk\""
1075                    .parse()
1076                    .unwrap(),
1077            );
1078            h.insert(
1079                "Content-Type",
1080                "application/json; charset=UTF-8".parse().unwrap(),
1081            );
1082            h.insert("Date", "Mon, 22 Jan 2018 18:56:00 GMT".parse().unwrap());
1083            h.insert("Expires", "Mon, 22 Jan 2018 18:56:00 GMT".parse().unwrap());
1084            h.insert("Cache-Control", "private, max-age=0".parse().unwrap());
1085            h.insert("Content-Length", "846".parse().unwrap());
1086
1087            h
1088        });
1089        assert_eq!(multipart.parts[1].content.len(), part1_bs.len());
1090        assert_eq!(multipart.parts[1].uri, None);
1091        assert_eq!(multipart.parts[1].method, None);
1092        assert_eq!(
1093            multipart.parts[1].status_code,
1094            Some(StatusCode::from_u16(200).unwrap())
1095        );
1096
1097        assert_eq!(multipart.parts[2].part_headers, {
1098            let mut h = HeaderMap::new();
1099            h.insert("Content-Type", "application/http".parse().unwrap());
1100            h.insert(
1101                "Content-ID",
1102                "<response-b29c5de2-0db4-490b-b421-6a51b598bd22+3>"
1103                    .parse()
1104                    .unwrap(),
1105            );
1106
1107            h
1108        });
1109        assert_eq!(multipart.parts[2].version, Version::HTTP_11);
1110        assert_eq!(multipart.parts[2].headers, {
1111            let mut h = HeaderMap::new();
1112            h.insert(
1113                "ETag",
1114                "\"lGaP-E0memYDumK16YuUDM_6Gf0/d2Z1F1_ZVbB1dC0YKM9rX5VAgIQ\""
1115                    .parse()
1116                    .unwrap(),
1117            );
1118            h.insert(
1119                "Content-Type",
1120                "application/json; charset=UTF-8".parse().unwrap(),
1121            );
1122            h.insert("Date", "Mon, 22 Jan 2018 18:56:00 GMT".parse().unwrap());
1123            h.insert("Expires", "Mon, 22 Jan 2018 18:56:00 GMT".parse().unwrap());
1124            h.insert("Cache-Control", "private, max-age=0".parse().unwrap());
1125            h.insert("Content-Length", "846".parse().unwrap());
1126
1127            h
1128        });
1129        assert_eq!(multipart.parts[2].content.len(), part2_bs.len());
1130        assert_eq!(multipart.parts[2].uri, None);
1131        assert_eq!(multipart.parts[2].method, None);
1132        assert_eq!(
1133            multipart.parts[2].status_code,
1134            Some(StatusCode::from_u16(200).unwrap())
1135        );
1136    }
1137
1138    #[test]
1139    fn test_multipart_related_gcs_simple() {
1140        // This is what multipart/related body might look for an insert in GCS
1141        // https://cloud.google.com/storage/docs/uploading-objects
1142        let expected = r#"--separator_string
1143content-type: application/json; charset=UTF-8
1144
1145{"name":"my-document.txt"}
1146--separator_string
1147content-type: text/plain
1148
1149This is a text file.
1150--separator_string--
1151"#;
1152
1153        let multipart = Multipart::new()
1154            .with_boundary("separator_string")
1155            .part(
1156                RelatedPart::new()
1157                    .header(
1158                        "Content-Type".parse().unwrap(),
1159                        "application/json; charset=UTF-8".parse().unwrap(),
1160                    )
1161                    .content(r#"{"name":"my-document.txt"}"#),
1162            )
1163            .part(
1164                RelatedPart::new()
1165                    .header(
1166                        "Content-Type".parse().unwrap(),
1167                        "text/plain".parse().unwrap(),
1168                    )
1169                    .content("This is a text file."),
1170            );
1171
1172        let bs = multipart.build();
1173
1174        let output = String::from_utf8(bs.to_bytes().to_vec())
1175            .unwrap()
1176            .replace("\r\n", "\n");
1177
1178        assert_eq!(output, expected);
1179    }
1180}
```
