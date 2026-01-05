# 

opendal/raw/

time.rs

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
18//! Time related utils.
19
20use crate::*;
21use jiff::SignedDuration;
22use std::fmt;
23use std::ops::{Add, AddAssign, Sub, SubAssign};
24use std::str::FromStr;
25use std::time::{Duration, SystemTime};
26
27/// An instant in time represented as the number of nanoseconds since the Unix epoch.
28#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
29pub struct Timestamp(jiff::Timestamp);
30
31impl FromStr for Timestamp {
32    type Err = Error;
33
34    /// Parse a timestamp by the default [`DateTimeParser`].
35    ///
36    /// All of them are valid time:
37    ///
38    /// - `2022-03-13T07:20:04Z`
39    /// - `2022-03-01T08:12:34+00:00`
40    /// - `2022-03-01T08:12:34.00+00:00`
41    /// - `2022-07-08T02:14:07+02:00[Europe/Paris]`
42    ///
43    /// [`DateTimeParser`]: jiff::fmt::temporal::DateTimeParser
44    fn from_str(s: &str) -> Result<Self, Self::Err> {
45        match s.parse() {
46            Ok(t) => Ok(Timestamp(t)),
47            Err(err) => Err(Error::new(
48                ErrorKind::Unexpected,
49                format!("parse '{s}' into timestamp failed"),
50            )
51            .set_source(err)),
52        }
53    }
54}
55
56impl fmt::Display for Timestamp {
57    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
58        write!(f, "{}", self.0)
59    }
60}
61
62impl Timestamp {
63    /// The minimum timestamp value.
64    pub const MIN: Self = Self(jiff::Timestamp::MIN);
65
66    /// The maximum timestamp value.
67    pub const MAX: Self = Self(jiff::Timestamp::MAX);
68
69    /// Create the timestamp of now.
70    pub fn now() -> Self {
71        Self(jiff::Timestamp::now())
72    }
73
74    /// Format the timestamp into http date: `Sun, 06 Nov 1994 08:49:37 GMT`
75    ///
76    /// ## Note
77    ///
78    /// HTTP date is slightly different from RFC2822.
79    ///
80    /// - Timezone is fixed to GMT.
81    /// - Day must be 2 digit.
82    pub fn format_http_date(self) -> String {
83        self.0.strftime("%a, %d %b %Y %T GMT").to_string()
84    }
85
86    /// Creates a new instant in time from the number of seconds elapsed since the Unix epoch.
87    ///
88    /// When second is negative, it corresponds to an instant in time before the Unix epoch.
89    /// A smaller number corresponds to an instant in time further into the past.
90    pub fn new(second: i64, nanosecond: i32) -> Result<Self, Error> {
91        match jiff::Timestamp::new(second, nanosecond) {
92            Ok(t) => Ok(Timestamp(t)),
93            Err(err) => Err(Error::new(
94                ErrorKind::Unexpected,
95                format!(
96                    "create timestamp from '{second}' seconds and '{nanosecond}' nanoseconds failed"
97                ),
98            )
99            .set_source(err)),
100        }
101    }
102
103    /// Creates a new instant in time from the number of milliseconds elapsed
104    /// since the Unix epoch.
105    ///
106    /// When `millisecond` is negative, it corresponds to an instant in time
107    /// before the Unix epoch. A smaller number corresponds to an instant in
108    /// time further into the past.
109    pub fn from_millisecond(millis: i64) -> Result<Self> {
110        match jiff::Timestamp::from_millisecond(millis) {
111            Ok(t) => Ok(Timestamp(t)),
112            Err(err) => Err(Error::new(
113                ErrorKind::Unexpected,
114                format!("convert '{millis}' milliseconds into timestamp failed"),
115            )
116            .set_source(err)),
117        }
118    }
119
120    /// Creates a new instant in time from the number of seconds elapsed since
121    /// the Unix epoch.
122    ///
123    /// When `second` is negative, it corresponds to an instant in time before
124    /// the Unix epoch. A smaller number corresponds to an instant in time
125    /// further into the past.
126    pub fn from_second(second: i64) -> Result<Self> {
127        match jiff::Timestamp::from_second(second) {
128            Ok(t) => Ok(Timestamp(t)),
129            Err(err) => Err(Error::new(
130                ErrorKind::Unexpected,
131                format!("convert '{second}' seconds into timestamp failed"),
132            )
133            .set_source(err)),
134        }
135    }
136
137    /// Parse a timestamp from RFC2822.
138    ///
139    /// All of them are valid time:
140    ///
141    /// - `Sat, 13 Jul 2024 15:09:59 -0400`
142    /// - `Mon, 15 Aug 2022 16:50:12 GMT`
143    pub fn parse_rfc2822(s: &str) -> Result<Timestamp> {
144        match jiff::fmt::rfc2822::parse(s) {
145            Ok(zoned) => Ok(Timestamp(zoned.timestamp())),
146            Err(err) => Err(Error::new(
147                ErrorKind::Unexpected,
148                format!("parse '{s}' into rfc2822 failed"),
149            )
150            .set_source(err)),
151        }
152    }
153
154    /// Convert to inner `jiff::Timestamp` for compatibility.
155    ///
156    /// This method is provided for accessing the underlying `jiff::Timestamp`
157    /// when needed for interoperability with jiff-specific APIs.
158    pub fn into_inner(self) -> jiff::Timestamp {
159        self.0
160    }
161}
162
163impl From<Timestamp> for jiff::Timestamp {
164    fn from(t: Timestamp) -> Self {
165        t.0
166    }
167}
168
169impl From<Timestamp> for SystemTime {
170    fn from(t: Timestamp) -> Self {
171        t.0.into()
172    }
173}
174
175impl From<jiff::Timestamp> for Timestamp {
176    fn from(t: jiff::Timestamp) -> Self {
177        Timestamp(t)
178    }
179}
180
181impl TryFrom<SystemTime> for Timestamp {
182    type Error = Error;
183
184    fn try_from(t: SystemTime) -> Result<Self> {
185        jiff::Timestamp::try_from(t).map(Timestamp).map_err(|err| {
186            Error::new(ErrorKind::Unexpected, "input timestamp overflow").set_source(err)
187        })
188    }
189}
190
191impl Add<Duration> for Timestamp {
192    type Output = Timestamp;
193
194    fn add(self, rhs: Duration) -> Timestamp {
195        let ts = self
196            .0
197            .checked_add(rhs)
198            .expect("adding unsigned duration to timestamp overflowed");
199
200        Timestamp(ts)
201    }
202}
203
204impl AddAssign<Duration> for Timestamp {
205    fn add_assign(&mut self, rhs: Duration) {
206        *self = *self + rhs
207    }
208}
209
210impl Sub<Duration> for Timestamp {
211    type Output = Timestamp;
212
213    fn sub(self, rhs: Duration) -> Timestamp {
214        let ts = self
215            .0
216            .checked_sub(rhs)
217            .expect("subtracting unsigned duration from timestamp overflowed");
218
219        Timestamp(ts)
220    }
221}
222
223impl SubAssign<Duration> for Timestamp {
224    fn sub_assign(&mut self, rhs: Duration) {
225        *self = *self - rhs
226    }
227}
228
229/// Convert an unsigned [`Duration`] into a jiff [`SignedDuration`].
230/// Parse a duration encoded either as ISO-8601 (e.g. `PT5M`) or friendly (e.g. `5m`).
231#[inline]
232pub fn signed_to_duration(value: &str) -> Result<Duration> {
233    let signed = value.parse::<SignedDuration>().map_err(|err| {
234        Error::new(ErrorKind::ConfigInvalid, "failed to parse duration").set_source(err)
235    })?;
236
237    Duration::try_from(signed).map_err(|err| {
238        Error::new(
239            ErrorKind::ConfigInvalid,
240            "duration must not be negative or overflow",
241        )
242        .set_source(err)
243    })
244}
245
246#[cfg(test)]
247mod tests {
248    use super::*;
249
250    fn test_time() -> Timestamp {
251        Timestamp("2022-03-01T08:12:34Z".parse().unwrap())
252    }
253
254    #[test]
255    fn test_format_http_date() {
256        let t = test_time();
257        assert_eq!("Tue, 01 Mar 2022 08:12:34 GMT", t.format_http_date())
258    }
259
260    #[test]
261    fn test_parse_rfc3339() {
262        let t = test_time();
263
264        for v in [
265            "2022-03-01T08:12:34Z",
266            "2022-03-01T08:12:34+00:00",
267            "2022-03-01T08:12:34.00+00:00",
268        ] {
269            assert_eq!(t, v.parse().expect("must be valid time"));
270        }
271    }
272
273    #[test]
274    fn test_parse_rfc2822() {
275        let s = "Sat, 29 Oct 1994 19:43:31 +0000";
276        let v = Timestamp::parse_rfc2822(s).unwrap();
277        assert_eq!("Sat, 29 Oct 1994 19:43:31 GMT", v.format_http_date());
278    }
279}
```
