# 

opendal/raw/http_util/

bytes_range.rs

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
19use std::fmt::Display;
20use std::fmt::Formatter;
21use std::ops::Bound;
22use std::ops::RangeBounds;
23use std::str::FromStr;
24
25use crate::*;
26
27/// BytesRange(offset, size) carries a range of content.
28///
29/// BytesRange implements `ToString` which can be used as `Range` HTTP header directly.
30///
31/// `<unit>` should always be `bytes`.
32///
33/// ```text
34/// Range: bytes=<range-start>-
35/// Range: bytes=<range-start>-<range-end>
36/// ```
37///
38/// # Notes
39///
40/// We don't support tailing read like `Range: bytes=-<range-end>`
41#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
42pub struct BytesRange(
43    /// Offset of the range.
44    u64,
45    /// Size of the range.
46    Option<u64>,
47);
48
49impl BytesRange {
50    /// Create a new `BytesRange`
51    ///
52    /// It better to use `BytesRange::from(1024..2048)` to construct.
53    ///
54    /// # Note
55    ///
56    /// The behavior for `None` and `Some` of `size` is different.
57    ///
58    /// - size=None => `bytes=<offset>-`, read from `<offset>` until the end
59    /// - size=Some(1024) => `bytes=<offset>-<offset + 1024>`, read 1024 bytes starting from the `<offset>`
60    pub fn new(offset: u64, size: Option<u64>) -> Self {
61        BytesRange(offset, size)
62    }
63
64    /// Get offset of BytesRange.
65    pub fn offset(&self) -> u64 {
66        self.0
67    }
68
69    /// Get size of BytesRange.
70    pub fn size(&self) -> Option<u64> {
71        self.1
72    }
73
74    /// Advance the range by `n` bytes.
75    ///
76    /// # Panics
77    ///
78    /// Panic if input `n` is larger than the size of the range.
79    pub fn advance(&mut self, n: u64) {
80        self.0 += n;
81        self.1 = self.1.map(|size| size - n);
82    }
83
84    /// Check if this range is full of this content.
85    ///
86    /// If this range is full, we don't need to specify it in http request.
87    pub fn is_full(&self) -> bool {
88        self.0 == 0 && self.1.is_none()
89    }
90
91    /// Convert bytes range into Range header.
92    pub fn to_header(&self) -> String {
93        format!("bytes={self}")
94    }
95
96    /// Convert bytes range into rust range.
97    pub fn to_range(&self) -> impl RangeBounds<u64> {
98        (
99            Bound::Included(self.0),
100            match self.1 {
101                Some(size) => Bound::Excluded(self.0 + size),
102                None => Bound::Unbounded,
103            },
104        )
105    }
106
107    /// Convert bytes range into rust range with usize.
108    pub(crate) fn to_range_as_usize(self) -> impl RangeBounds<usize> {
109        (
110            Bound::Included(self.0 as usize),
111            match self.1 {
112                Some(size) => Bound::Excluded((self.0 + size) as usize),
113                None => Bound::Unbounded,
114            },
115        )
116    }
117}
118
119impl Display for BytesRange {
120    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
121        match self.1 {
122            None => write!(f, "{}-", self.0),
123            Some(size) => write!(f, "{}-{}", self.0, self.0 + size - 1),
124        }
125    }
126}
127
128impl FromStr for BytesRange {
129    type Err = Error;
130
131    fn from_str(value: &str) -> Result<Self> {
132        let s = value.strip_prefix("bytes=").ok_or_else(|| {
133            Error::new(ErrorKind::Unexpected, "header range is invalid")
134                .with_operation("BytesRange::from_str")
135                .with_context("value", value)
136        })?;
137
138        if s.contains(',') {
139            return Err(Error::new(ErrorKind::Unexpected, "header range is invalid")
140                .with_operation("BytesRange::from_str")
141                .with_context("value", value));
142        }
143
144        let v = s.split('-').collect::<Vec<_>>();
145        if v.len() != 2 {
146            return Err(Error::new(ErrorKind::Unexpected, "header range is invalid")
147                .with_operation("BytesRange::from_str")
148                .with_context("value", value));
149        }
150
151        let parse_int_error = |e: std::num::ParseIntError| {
152            Error::new(ErrorKind::Unexpected, "header range is invalid")
153                .with_operation("BytesRange::from_str")
154                .with_context("value", value)
155                .set_source(e)
156        };
157
158        if v[1].is_empty() {
159            // <range-start>-
160            Ok(BytesRange::new(
161                v[0].parse().map_err(parse_int_error)?,
162                None,
163            ))
164        } else if v[0].is_empty() {
165            // -<suffix-length>
166            Err(Error::new(
167                ErrorKind::Unexpected,
168                "header range with tailing is not supported",
169            )
170            .with_operation("BytesRange::from_str")
171            .with_context("value", value))
172        } else {
173            // <range-start>-<range-end>
174            let start: u64 = v[0].parse().map_err(parse_int_error)?;
175            let end: u64 = v[1].parse().map_err(parse_int_error)?;
176            Ok(BytesRange::new(start, Some(end - start + 1)))
177        }
178    }
179}
180
181impl<T> From<T> for BytesRange
182where
183    T: RangeBounds<u64>,
184{
185    fn from(range: T) -> Self {
186        let offset = match range.start_bound().cloned() {
187            Bound::Included(n) => n,
188            Bound::Excluded(n) => n + 1,
189            Bound::Unbounded => 0,
190        };
191        let size = match range.end_bound().cloned() {
192            Bound::Included(n) => Some(n + 1 - offset),
193            Bound::Excluded(n) => Some(n - offset),
194            Bound::Unbounded => None,
195        };
196
197        BytesRange(offset, size)
198    }
199}
200
201#[cfg(test)]
202mod tests {
203    use super::*;
204
205    #[test]
206    fn test_bytes_range_to_string() {
207        let h = BytesRange::new(0, Some(1024));
208        assert_eq!(h.to_string(), "0-1023");
209
210        let h = BytesRange::new(1024, None);
211        assert_eq!(h.to_string(), "1024-");
212
213        let h = BytesRange::new(1024, Some(1024));
214        assert_eq!(h.to_string(), "1024-2047");
215    }
216
217    #[test]
218    fn test_bytes_range_to_header() {
219        let h = BytesRange::new(0, Some(1024));
220        assert_eq!(h.to_header(), "bytes=0-1023");
221
222        let h = BytesRange::new(1024, None);
223        assert_eq!(h.to_header(), "bytes=1024-");
224
225        let h = BytesRange::new(1024, Some(1024));
226        assert_eq!(h.to_header(), "bytes=1024-2047");
227    }
228
229    #[test]
230    fn test_bytes_range_from_range_bounds() {
231        assert_eq!(BytesRange::new(0, None), BytesRange::from(..));
232        assert_eq!(BytesRange::new(10, None), BytesRange::from(10..));
233        assert_eq!(BytesRange::new(0, Some(11)), BytesRange::from(..=10));
234        assert_eq!(BytesRange::new(0, Some(10)), BytesRange::from(..10));
235        assert_eq!(BytesRange::new(10, Some(10)), BytesRange::from(10..20));
236        assert_eq!(BytesRange::new(10, Some(11)), BytesRange::from(10..=20));
237    }
238
239    #[test]
240    fn test_bytes_range_from_str() -> Result<()> {
241        let cases = vec![
242            ("range-start", "bytes=123-", BytesRange::new(123, None)),
243            ("range", "bytes=123-124", BytesRange::new(123, Some(2))),
244            ("one byte", "bytes=0-0", BytesRange::new(0, Some(1))),
245            (
246                "lower case header",
247                "bytes=0-0",
248                BytesRange::new(0, Some(1)),
249            ),
250        ];
251
252        for (name, input, expected) in cases {
253            let actual = input.parse()?;
254
255            assert_eq!(expected, actual, "{name}")
256        }
257
258        Ok(())
259    }
260}
```
