# 

opendal/raw/http_util/

uri.rs

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
18use percent_encoding::AsciiSet;
19use percent_encoding::NON_ALPHANUMERIC;
20use percent_encoding::percent_decode_str;
21use percent_encoding::utf8_percent_encode;
22
23use crate::*;
24
25/// Parse http uri invalid error in to opendal::Error.
26pub fn new_http_uri_invalid_error(err: http::uri::InvalidUri) -> Error {
27    Error::new(ErrorKind::Unexpected, "parse http uri").set_source(err)
28}
29
30/// PATH_ENCODE_SET is the encode set for http url path.
31///
32/// This set follows [encodeURIComponent](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent) which will encode all non-ASCII characters except `A-Z a-z 0-9 - _ . ! ~ * ' ( )`
33///
34/// There is a special case for `/` in path: we will allow `/` in path as
35/// required by storage services like s3.
36static PATH_ENCODE_SET: AsciiSet = NON_ALPHANUMERIC
37    .remove(b'/')
38    .remove(b'-')
39    .remove(b'_')
40    .remove(b'.')
41    .remove(b'!')
42    .remove(b'~')
43    .remove(b'*')
44    .remove(b'\'')
45    .remove(b'(')
46    .remove(b')');
47
48/// percent_encode_path will do percent encoding for http encode path.
49///
50/// Follows [encodeURIComponent](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent) which will encode all non-ASCII characters except `A-Z a-z 0-9 - _ . ! ~ * ' ( )`
51///
52/// There is a special case for `/` in path: we will allow `/` in path as
53/// required by storage services like s3.
54pub fn percent_encode_path(path: &str) -> String {
55    utf8_percent_encode(path, &PATH_ENCODE_SET).to_string()
56}
57
58/// percent_decode_path will do percent decoding for http decode path.
59///
60/// If the input is not percent encoded or not valid utf8, return the input.
61pub fn percent_decode_path(path: &str) -> String {
62    match percent_decode_str(path).decode_utf8() {
63        Ok(v) => v.to_string(),
64        Err(_) => path.to_string(),
65    }
66}
67
68/// QueryPairsWriter is used to write query pairs to a url.
69pub struct QueryPairsWriter {
70    base: String,
71    has_query: bool,
72}
73
74impl QueryPairsWriter {
75    /// Create a new QueryPairsWriter with the given base.
76    pub fn new(s: &str) -> Self {
77        // 256 is the average size we observed of a url
78        // in production.
79        //
80        // We eagerly allocate the string to avoid multiple
81        // allocations.
82        let mut base = String::with_capacity(256);
83        base.push_str(s);
84
85        Self {
86            base,
87            has_query: false,
88        }
89    }
90
91    /// Push a new pair of key and value to the url.
92    ///
93    /// The input key and value must already been percent
94    /// encoded correctly.
95    pub fn push(mut self, key: &str, value: &str) -> Self {
96        if self.has_query {
97            self.base.push('&');
98        } else {
99            self.base.push('?');
100            self.has_query = true;
101        }
102
103        // Append the key and value to the base string
104        self.base.push_str(key);
105        if !value.is_empty() {
106            self.base.push('=');
107            self.base.push_str(value);
108        }
109
110        self
111    }
112
113    /// Finish the url and return it.
114    pub fn finish(self) -> String {
115        self.base
116    }
117}
118
119#[cfg(test)]
120mod tests {
121    use super::*;
122
123    #[test]
124    fn test_percent_encode_path() {
125        let cases = vec![
126            (
127                "Reserved Characters",
128                ";,/?:@&=+$",
129                "%3B%2C/%3F%3A%40%26%3D%2B%24",
130            ),
131            ("Unescaped Characters", "-_.!~*'()", "-_.!~*'()"),
132            ("Number Sign", "#", "%23"),
133            (
134                "Alphanumeric Characters + Space",
135                "ABC abc 123",
136                "ABC%20abc%20123",
137            ),
138            (
139                "Unicode",
140                "ä½ å¥½ï¼ä¸çï¼â¤",
141                "%E4%BD%A0%E5%A5%BD%EF%BC%8C%E4%B8%96%E7%95%8C%EF%BC%81%E2%9D%A4",
142            ),
143        ];
144
145        for (name, input, expected) in cases {
146            let actual = percent_encode_path(input);
147
148            assert_eq!(actual, expected, "{name}");
149        }
150    }
151
152    #[test]
153    fn test_percent_decode_path() {
154        let cases = vec![
155            (
156                "Reserved Characters",
157                "%3B%2C/%3F%3A%40%26%3D%2B%24",
158                ";,/?:@&=+$",
159            ),
160            ("Unescaped Characters", "-_.!~*'()", "-_.!~*'()"),
161            ("Number Sign", "%23", "#"),
162            (
163                "Alphanumeric Characters + Space",
164                "ABC%20abc%20123",
165                "ABC abc 123",
166            ),
167            (
168                "Unicode Characters",
169                "%E4%BD%A0%E5%A5%BD%EF%BC%8C%E4%B8%96%E7%95%8C%EF%BC%81%E2%9D%A4",
170                "ä½ å¥½ï¼ä¸çï¼â¤",
171            ),
172            (
173                "Double Encoded Characters",
174                "Double%2520Encoded",
175                "Double%20Encoded",
176            ),
177            (
178                "Not Percent Encoded Characters",
179                "/not percent encoded/path;,/?:@&=+$-",
180                "/not percent encoded/path;,/?:@&=+$-",
181            ),
182        ];
183
184        for (name, input, expected) in cases {
185            let actual = percent_decode_path(input);
186
187            assert_eq!(actual, expected, "{name}");
188        }
189    }
190}
```
