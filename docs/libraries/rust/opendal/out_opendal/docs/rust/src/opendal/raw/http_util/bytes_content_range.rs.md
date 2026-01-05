# 

opendal/raw/http_util/

bytes_content_range.rs

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
18use std::fmt::Display;
19use std::fmt::Formatter;
20use std::ops::Range;
21use std::ops::RangeInclusive;
22use std::str::FromStr;
23
24use crate::*;
25
26/// BytesContentRange is the content range of bytes.
27///
28/// `<unit>` should always be `bytes`.
29///
30/// ```text
31/// Content-Range: bytes <range-start>-<range-end>/<size>
32/// Content-Range: bytes <range-start>-<range-end>/*
33/// Content-Range: bytes */<size>
34/// ```
35///
36/// # Notes
37///
38/// ## Usage of the default.
39///
40/// `BytesContentRange::default` is not a valid content range.
41/// Please make sure their comes up with `with_range` or `with_size` call.
42///
43/// ## Allow clippy::len_without_is_empty
44///
45/// BytesContentRange implements `len()` but not `is_empty()` because it's useless.
46/// - When BytesContentRange's range is known, it must be non-empty.
47/// - When BytesContentRange's range is no known, we don't know whether it's empty.
48#[allow(clippy::len_without_is_empty)]
49#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
50pub struct BytesContentRange(
51    /// Start position of the range. `None` means unknown.
52    Option<u64>,
53    /// End position of the range. `None` means unknown.
54    Option<u64>,
55    /// Size of the whole content. `None` means unknown.
56    Option<u64>,
57);
58
59impl BytesContentRange {
60    /// Update BytesContentRange with range.
61    ///
62    /// The range is inclusive: `[start..=end]` as described in `content-range`.
63    pub fn with_range(mut self, start: u64, end: u64) -> Self {
64        self.0 = Some(start);
65        self.1 = Some(end);
66        self
67    }
68
69    /// Update BytesContentRange with size.
70    pub fn with_size(mut self, size: u64) -> Self {
71        self.2 = Some(size);
72        self
73    }
74
75    /// Get the length that specified by this BytesContentRange, return `None` if range is not known.
76    pub fn len(&self) -> Option<u64> {
77        if let (Some(start), Some(end)) = (self.0, self.1) {
78            Some(end - start + 1)
79        } else {
80            None
81        }
82    }
83
84    /// Get the size of this BytesContentRange, return `None` if size is not known.
85    pub fn size(&self) -> Option<u64> {
86        self.2
87    }
88
89    /// Get the range inclusive of this BytesContentRange, return `None` if range is not known.
90    pub fn range(&self) -> Option<Range<u64>> {
91        if let (Some(start), Some(end)) = (self.0, self.1) {
92            Some(start..end + 1)
93        } else {
94            None
95        }
96    }
97
98    /// Get the range inclusive of this BytesContentRange, return `None` if range is not known.
99    pub fn range_inclusive(&self) -> Option<RangeInclusive<u64>> {
100        if let (Some(start), Some(end)) = (self.0, self.1) {
101            Some(start..=end)
102        } else {
103            None
104        }
105    }
106
107    /// Convert bytes content range into Content-Range header.
108    pub fn to_header(&self) -> String {
109        format!("bytes {self}")
110    }
111}
112
113impl Display for BytesContentRange {
114    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
115        match (self.0, self.1, self.2) {
116            (Some(start), Some(end), Some(size)) => write!(f, "{start}-{end}/{size}"),
117            (Some(start), Some(end), None) => write!(f, "{start}-{end}/*"),
118            (None, None, Some(size)) => write!(f, "*/{size}"),
119            _ => unreachable!("invalid bytes range: {:?}", self),
120        }
121    }
122}
123
124impl FromStr for BytesContentRange {
125    type Err = Error;
126
127    fn from_str(value: &str) -> Result<Self> {
128        let s = value.strip_prefix("bytes ").ok_or_else(|| {
129            Error::new(ErrorKind::Unexpected, "header content range is invalid")
130                .with_operation("BytesContentRange::from_str")
131                .with_context("value", value)
132        })?;
133
134        let parse_int_error = |e: std::num::ParseIntError| {
135            Error::new(ErrorKind::Unexpected, "header content range is invalid")
136                .with_operation("BytesContentRange::from_str")
137                .with_context("value", value)
138                .set_source(e)
139        };
140
141        if let Some(size) = s.strip_prefix("*/") {
142            return Ok(
143                BytesContentRange::default().with_size(size.parse().map_err(parse_int_error)?)
144            );
145        }
146
147        let s: Vec<_> = s.split('/').collect();
148        if s.len() != 2 {
149            return Err(
150                Error::new(ErrorKind::Unexpected, "header content range is invalid")
151                    .with_operation("BytesContentRange::from_str")
152                    .with_context("value", value),
153            );
154        }
155
156        let v: Vec<_> = s[0].split('-').collect();
157        if v.len() != 2 {
158            return Err(
159                Error::new(ErrorKind::Unexpected, "header content range is invalid")
160                    .with_operation("BytesContentRange::from_str")
161                    .with_context("value", value),
162            );
163        }
164        let start: u64 = v[0].parse().map_err(parse_int_error)?;
165        let end: u64 = v[1].parse().map_err(parse_int_error)?;
166        let mut bcr = BytesContentRange::default().with_range(start, end);
167
168        // Handle size part first.
169        if s[1] != "*" {
170            bcr = bcr.with_size(s[1].parse().map_err(parse_int_error)?)
171        };
172
173        Ok(bcr)
174    }
175}
176
177#[cfg(test)]
178mod tests {
179    use super::*;
180
181    #[test]
182    fn test_bytes_content_range_from_str() -> Result<()> {
183        let cases = vec![
184            (
185                "range start with unknown size",
186                "bytes 123-123/*",
187                BytesContentRange::default().with_range(123, 123),
188            ),
189            (
190                "range start with known size",
191                "bytes 123-123/1024",
192                BytesContentRange::default()
193                    .with_range(123, 123)
194                    .with_size(1024),
195            ),
196            (
197                "only have size",
198                "bytes */1024",
199                BytesContentRange::default().with_size(1024),
200            ),
201        ];
202
203        for (name, input, expected) in cases {
204            let actual = input.parse()?;
205
206            assert_eq!(expected, actual, "{name}")
207        }
208
209        Ok(())
210    }
211
212    #[test]
213    fn test_bytes_content_range_to_string() {
214        let h = BytesContentRange::default().with_size(1024);
215        assert_eq!(h.to_string(), "*/1024");
216
217        let h = BytesContentRange::default().with_range(0, 1023);
218        assert_eq!(h.to_string(), "0-1023/*");
219
220        let h = BytesContentRange::default()
221            .with_range(0, 1023)
222            .with_size(1024);
223        assert_eq!(h.to_string(), "0-1023/1024");
224    }
225
226    #[test]
227    fn test_bytes_content_range_to_header() {
228        let h = BytesContentRange::default().with_size(1024);
229        assert_eq!(h.to_header(), "bytes */1024");
230
231        let h = BytesContentRange::default().with_range(0, 1023);
232        assert_eq!(h.to_header(), "bytes 0-1023/*");
233
234        let h = BytesContentRange::default()
235            .with_range(0, 1023)
236            .with_size(1024);
237        assert_eq!(h.to_header(), "bytes 0-1023/1024");
238    }
239}
```
