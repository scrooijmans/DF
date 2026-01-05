# 

opendal/types/operator/

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
18use std::collections::HashMap;
19
20use http::Uri;
21use percent_encoding::percent_decode_str;
22use url::Url;
23
24use crate::{Error, ErrorKind, Result};
25
26/// Parsed representation of an operator URI with normalized components.
27#[derive(Clone, Debug, Eq, PartialEq)]
28pub struct OperatorUri {
29    scheme: String,
30    authority: Option<String>,
31    name: Option<String>,
32    username: Option<String>,
33    password: Option<String>,
34    root: Option<String>,
35    options: HashMap<String, String>,
36}
37
38impl OperatorUri {
39    /// Build [`OperatorUri`] from a URI string plus additional options.
40    pub fn new(
41        base: &str,
42        extra_options: impl IntoIterator<Item = (String, String)>,
43    ) -> Result<Self> {
44        let url = Url::parse(base).map_err(|err| {
45            Error::new(ErrorKind::ConfigInvalid, "failed to parse uri").set_source(err)
46        })?;
47
48        let scheme = url.scheme().to_ascii_lowercase();
49
50        let mut options = HashMap::<String, String>::new();
51
52        for (key, value) in url.query_pairs() {
53            options.insert(key.to_ascii_lowercase(), value.into_owned());
54        }
55
56        for (key, value) in extra_options {
57            options.insert(key.to_ascii_lowercase(), value);
58        }
59
60        let username = if url.username().is_empty() {
61            None
62        } else {
63            Some(url.username().to_string())
64        };
65
66        let password = url.password().map(|pwd| pwd.to_string());
67
68        let authority = url.host_str().filter(|host| !host.is_empty()).map(|host| {
69            if let Some(port) = url.port() {
70                format!("{host}:{port}")
71            } else {
72                host.to_string()
73            }
74        });
75
76        let name = url
77            .host_str()
78            .filter(|host| !host.is_empty())
79            .map(|host| host.to_string());
80
81        let decoded_path = percent_decode_str(url.path()).decode_utf8_lossy();
82        let trimmed = decoded_path.trim_matches('/');
83        let root = if trimmed.is_empty() {
84            None
85        } else {
86            Some(trimmed.to_string())
87        };
88
89        Ok(Self {
90            scheme,
91            authority,
92            name,
93            username,
94            password,
95            root,
96            options,
97        })
98    }
99
100    /// Normalized scheme in lowercase.
101    pub fn scheme(&self) -> &str {
102        self.scheme.as_str()
103    }
104
105    /// Name extracted from the URI authority, if present.
106    pub fn name(&self) -> Option<&str> {
107        self.name.as_deref()
108    }
109
110    /// Authority extracted from the URI, if present (host with optional port).
111    pub fn authority(&self) -> Option<&str> {
112        self.authority.as_deref()
113    }
114
115    /// Username extracted from the URI, if present.
116    pub fn username(&self) -> Option<&str> {
117        self.username.as_deref()
118    }
119
120    /// Password extracted from the URI, if present.
121    pub fn password(&self) -> Option<&str> {
122        self.password.as_deref()
123    }
124
125    /// Root path (without leading slash) extracted from the URI path, if present.
126    pub fn root(&self) -> Option<&str> {
127        self.root.as_deref()
128    }
129
130    /// Normalized option map merged from query string and extra options (excluding reserved keys).
131    pub fn options(&self) -> &HashMap<String, String> {
132        &self.options
133    }
134
135    /// Retrieve a specific option by key (case-insensitive).
136    pub fn option(&self, key: &str) -> Option<&str> {
137        self.options
138            .get(&key.to_ascii_lowercase())
139            .map(String::as_str)
140    }
141}
142
143/// Conversion trait that builds [`OperatorUri`] from various inputs.
144pub trait IntoOperatorUri {
145    /// Convert the input into an [`OperatorUri`].
146    fn into_operator_uri(self) -> Result<OperatorUri>;
147}
148
149impl IntoOperatorUri for OperatorUri {
150    fn into_operator_uri(self) -> Result<OperatorUri> {
151        Ok(self)
152    }
153}
154
155impl IntoOperatorUri for &OperatorUri {
156    fn into_operator_uri(self) -> Result<OperatorUri> {
157        Ok(self.clone())
158    }
159}
160
161impl IntoOperatorUri for Uri {
162    fn into_operator_uri(self) -> Result<OperatorUri> {
163        let serialized = self.to_string();
164        OperatorUri::new(&serialized, Vec::<(String, String)>::new())
165    }
166}
167
168impl IntoOperatorUri for &Uri {
169    fn into_operator_uri(self) -> Result<OperatorUri> {
170        let serialized = self.to_string();
171        OperatorUri::new(&serialized, Vec::<(String, String)>::new())
172    }
173}
174
175impl IntoOperatorUri for &str {
176    fn into_operator_uri(self) -> Result<OperatorUri> {
177        OperatorUri::new(self, Vec::<(String, String)>::new())
178    }
179}
180
181impl IntoOperatorUri for String {
182    fn into_operator_uri(self) -> Result<OperatorUri> {
183        OperatorUri::new(&self, Vec::<(String, String)>::new())
184    }
185}
186
187impl<O, K, V> IntoOperatorUri for (Uri, O)
188where
189    O: IntoIterator<Item = (K, V)>,
190    K: Into<String>,
191    V: Into<String>,
192{
193    fn into_operator_uri(self) -> Result<OperatorUri> {
194        let (uri, extra) = self;
195        let serialized = uri.to_string();
196        let opts = extra
197            .into_iter()
198            .map(|(k, v)| (k.into(), v.into()))
199            .collect::<Vec<_>>();
200        OperatorUri::new(&serialized, opts)
201    }
202}
203
204impl<O, K, V> IntoOperatorUri for (&Uri, O)
205where
206    O: IntoIterator<Item = (K, V)>,
207    K: Into<String>,
208    V: Into<String>,
209{
210    fn into_operator_uri(self) -> Result<OperatorUri> {
211        let (uri, extra) = self;
212        let serialized = uri.to_string();
213        let opts = extra
214            .into_iter()
215            .map(|(k, v)| (k.into(), v.into()))
216            .collect::<Vec<_>>();
217        OperatorUri::new(&serialized, opts)
218    }
219}
220
221impl<O, K, V> IntoOperatorUri for (&str, O)
222where
223    O: IntoIterator<Item = (K, V)>,
224    K: Into<String>,
225    V: Into<String>,
226{
227    fn into_operator_uri(self) -> Result<OperatorUri> {
228        let (base, extra) = self;
229        let opts = extra
230            .into_iter()
231            .map(|(k, v)| (k.into(), v.into()))
232            .collect::<Vec<_>>();
233        OperatorUri::new(base, opts)
234    }
235}
236
237impl<O, K, V> IntoOperatorUri for (String, O)
238where
239    O: IntoIterator<Item = (K, V)>,
240    K: Into<String>,
241    V: Into<String>,
242{
243    fn into_operator_uri(self) -> Result<OperatorUri> {
244        let (base, extra) = self;
245        (&base[..], extra).into_operator_uri()
246    }
247}
248
249#[cfg(test)]
250mod tests {
251    use super::*;
252    use crate::types::IntoOperatorUri;
253
254    #[test]
255    fn parse_uri_with_name_and_root() {
256        let uri = OperatorUri::new(
257            "s3://example-bucket/photos/2024",
258            Vec::<(String, String)>::new(),
259        )
260        .unwrap();
261
262        assert_eq!(uri.scheme(), "s3");
263        assert_eq!(uri.authority(), Some("example-bucket"));
264        assert_eq!(uri.name(), Some("example-bucket"));
265        assert_eq!(uri.root(), Some("photos/2024"));
266        assert!(uri.options().is_empty());
267    }
268
269    #[test]
270    fn into_operator_uri_merges_extra_options() {
271        let uri = (
272            "s3://bucket/path?region=us-east-1",
273            vec![("region", "override"), ("endpoint", "https://custom")],
274        )
275            .into_operator_uri()
276            .unwrap();
277
278        assert_eq!(uri.scheme(), "s3");
279        assert_eq!(uri.name(), Some("bucket"));
280        assert_eq!(uri.root(), Some("path"));
281        assert_eq!(
282            uri.options().get("region").map(String::as_str),
283            Some("override")
284        );
285        assert_eq!(
286            uri.options().get("endpoint").map(String::as_str),
287            Some("https://custom")
288        );
289    }
290
291    #[test]
292    fn parse_uri_with_port_preserves_authority() {
293        let uri = OperatorUri::new(
294            "http://example.com:8080/root",
295            Vec::<(String, String)>::new(),
296        )
297        .unwrap();
298
299        assert_eq!(uri.scheme(), "http");
300        assert_eq!(uri.authority(), Some("example.com:8080"));
301        assert_eq!(uri.name(), Some("example.com"));
302        assert_eq!(uri.root(), Some("root"));
303    }
304
305    #[test]
306    fn parse_uri_with_credentials_splits_authority() {
307        let uri = OperatorUri::new(
308            "https://alice:secret@example.com:8443/path",
309            Vec::<(String, String)>::new(),
310        )
311        .unwrap();
312
313        assert_eq!(uri.scheme(), "https");
314        assert_eq!(uri.authority(), Some("example.com:8443"));
315        assert_eq!(uri.username(), Some("alice"));
316        assert_eq!(uri.password(), Some("secret"));
317        assert_eq!(uri.root(), Some("path"));
318    }
319}
```
