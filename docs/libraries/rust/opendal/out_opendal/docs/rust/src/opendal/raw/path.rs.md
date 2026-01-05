# 

opendal/raw/

path.rs

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
18use crate::*;
19use std::hash::{BuildHasher, Hasher};
20
21/// build_abs_path will build an absolute path with root.
22///
23/// # Rules
24///
25/// - Input root MUST be the format like `/abc/def/`
26/// - Output will be the format like `path/to/root/path`.
27pub fn build_abs_path(root: &str, path: &str) -> String {
28    debug_assert!(root.starts_with('/'), "root must start with /");
29    debug_assert!(root.ends_with('/'), "root must end with /");
30
31    let p = root[1..].to_string();
32
33    if path == "/" {
34        p
35    } else {
36        debug_assert!(!path.starts_with('/'), "path must not start with /");
37        p + path
38    }
39}
40
41/// build_rooted_abs_path will build an absolute path with root.
42///
43/// # Rules
44///
45/// - Input root MUST be the format like `/abc/def/`
46/// - Output will be the format like `/path/to/root/path`.
47pub fn build_rooted_abs_path(root: &str, path: &str) -> String {
48    debug_assert!(root.starts_with('/'), "root must start with /");
49    debug_assert!(root.ends_with('/'), "root must end with /");
50
51    let p = root.to_string();
52
53    if path == "/" {
54        p
55    } else {
56        debug_assert!(!path.starts_with('/'), "path must not start with /");
57        p + path
58    }
59}
60
61/// build_rel_path will build a relative path towards root.
62///
63/// # Rules
64///
65/// - Input root MUST be the format like `/abc/def/`
66/// - Input path MUST start with root like `/abc/def/path/to/file`
67/// - Output will be the format like `path/to/file`.
68pub fn build_rel_path(root: &str, path: &str) -> String {
69    debug_assert!(root != path, "get rel path with root is invalid");
70
71    if path.starts_with('/') {
72        debug_assert!(
73            path.starts_with(root),
74            "path {path} doesn't start with root {root}"
75        );
76        path[root.len()..].to_string()
77    } else {
78        debug_assert!(
79            path.starts_with(&root[1..]),
80            "path {path} doesn't start with root {root}"
81        );
82        path[root.len() - 1..].to_string()
83    }
84}
85
86/// Make sure all operation are constructed by normalized path:
87///
88/// - Path endswith `/` means it's a dir path.
89/// - Otherwise, it's a file path.
90///
91/// # Normalize Rules
92///
93/// - All whitespace will be trimmed: ` abc/def ` => `abc/def`
94/// - All leading / will be trimmed: `///abc` => `abc`
95/// - Internal // will be replaced by /: `abc///def` => `abc/def`
96/// - Empty path will be `/`: `` => `/`
97pub fn normalize_path(path: &str) -> String {
98    // - all whitespace has been trimmed.
99    // - all leading `/` has been trimmed.
100    let path = path.trim().trim_start_matches('/');
101
102    // Fast line for empty path.
103    if path.is_empty() {
104        return "/".to_string();
105    }
106
107    let has_trailing = path.ends_with('/');
108
109    let mut p = path
110        .split('/')
111        .filter(|v| !v.is_empty())
112        .collect::<Vec<&str>>()
113        .join("/");
114
115    // Append trailing back if input path is endswith `/`.
116    if has_trailing {
117        p.push('/');
118    }
119
120    p
121}
122
123/// Make sure root is normalized to style like `/abc/def/`.
124///
125/// # Normalize Rules
126///
127/// - All whitespace will be trimmed: ` abc/def ` => `abc/def`
128/// - All leading / will be trimmed: `///abc` => `abc`
129/// - Internal // will be replaced by /: `abc///def` => `abc/def`
130/// - Empty path will be `/`: `` => `/`
131/// - Add leading `/` if not starts with: `abc/` => `/abc/`
132/// - Add trailing `/` if not ends with: `/abc` => `/abc/`
133///
134/// Finally, we will get path like `/path/to/root/`.
135pub fn normalize_root(v: &str) -> String {
136    let mut v = v
137        .split('/')
138        .filter(|v| !v.is_empty())
139        .collect::<Vec<&str>>()
140        .join("/");
141    if !v.starts_with('/') {
142        v.insert(0, '/');
143    }
144    if !v.ends_with('/') {
145        v.push('/')
146    }
147    v
148}
149
150/// Get basename from path.
151pub fn get_basename(path: &str) -> &str {
152    // Handle root case
153    if path == "/" {
154        return "/";
155    }
156
157    // Handle file case
158    if !path.ends_with('/') {
159        return path
160            .split('/')
161            .next_back()
162            .expect("file path without name is invalid");
163    }
164
165    // The idx of second `/` if path in reserve order.
166    // - `abc/` => `None`
167    // - `abc/def/` => `Some(3)`
168    let idx = path[..path.len() - 1].rfind('/').map(|v| v + 1);
169
170    match idx {
171        Some(v) => {
172            let (_, name) = path.split_at(v);
173            name
174        }
175        None => path,
176    }
177}
178
179/// Get parent from path.
180pub fn get_parent(path: &str) -> &str {
181    if path == "/" {
182        return "/";
183    }
184
185    if !path.ends_with('/') {
186        // The idx of first `/` if path in reserve order.
187        // - `abc` => `None`
188        // - `abc/def` => `Some(3)`
189        let idx = path.rfind('/');
190
191        return match idx {
192            Some(v) => {
193                let (parent, _) = path.split_at(v + 1);
194                parent
195            }
196            None => "/",
197        };
198    }
199
200    // The idx of second `/` if path in reserve order.
201    // - `abc/` => `None`
202    // - `abc/def/` => `Some(3)`
203    let idx = path[..path.len() - 1].rfind('/').map(|v| v + 1);
204
205    match idx {
206        Some(v) => {
207            let (parent, _) = path.split_at(v);
208            parent
209        }
210        None => "/",
211    }
212}
213
214// Sets the size of random generated postfix for random file names
215const RANDOM_TMP_PATH_POSTFIX_LENGTH: usize = 8;
216// Allowed characters for choices in a random-generated char
217const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
218const CHARS_LENGTH: u64 = CHARS.len() as u64;
219
220/// Build a temporary path of a file path.
221///
222/// `build_tmp_path_of` appends a dot following a random generated postfix.
223/// Don't use it with a path to a folder.
224#[inline]
225pub fn build_tmp_path_of(path: &str) -> String {
226    let name = get_basename(path);
227    let mut buf = String::with_capacity(name.len() + RANDOM_TMP_PATH_POSTFIX_LENGTH);
228    buf.push_str(name);
229    buf.push('.');
230
231    // Uses `std` for the random number generator instead of external crates.
232    //
233    // `RandomState::new` builds a hasher that generates a different random sequence each time.
234    // Calling `RandomState::new` each time produces a `RandomState` from
235    // a per-thread pseudo seed pools that Rust manages.
236    // The default hasher, `SipHasher13`, has some notable properties:
237    //
238    // 1. `fastrand` is roughly 10x faster than `SipHasher13`, but it adds an extra dependency.
239    // 2. While `fastrand` is faster, `SipHasher13` is fast enough for our needs since
240    //    we're only generating a few characters.
241    // 3. This is not a cryptographically secure pseudorandom number generator (CSPRNG).
242    //
243    // If we need stronger randomness in the future, we can:
244    // 1. Increase the output length.
245    // 2. Use the `getrandom` crate to source randomness from the OS.
246    for _ in 0..RANDOM_TMP_PATH_POSTFIX_LENGTH {
247        let random = std::collections::hash_map::RandomState::new()
248            .build_hasher()
249            .finish();
250        let choice: usize = (random % CHARS_LENGTH).try_into().unwrap();
251        buf.push(CHARS[choice] as char);
252    }
253
254    buf
255}
256
257/// Validate given path is match with given EntryMode.
258#[inline]
259pub fn validate_path(path: &str, mode: EntryMode) -> bool {
260    debug_assert!(!path.is_empty(), "input path should not be empty");
261
262    match mode {
263        EntryMode::FILE => !path.ends_with('/'),
264        EntryMode::DIR => path.ends_with('/'),
265        EntryMode::Unknown => false,
266    }
267}
268
269#[cfg(test)]
270mod tests {
271    use super::*;
272
273    #[test]
274    fn test_normalize_path() {
275        let cases = vec![
276            ("file path", "abc", "abc"),
277            ("dir path", "abc/", "abc/"),
278            ("empty path", "", "/"),
279            ("root path", "/", "/"),
280            ("root path with extra /", "///", "/"),
281            ("abs file path", "/abc/def", "abc/def"),
282            ("abs dir path", "/abc/def/", "abc/def/"),
283            ("abs file path with extra /", "///abc/def", "abc/def"),
284            ("abs dir path with extra /", "///abc/def/", "abc/def/"),
285            ("file path contains ///", "abc///def", "abc/def"),
286            ("dir path contains ///", "abc///def///", "abc/def/"),
287            ("file with whitespace", "abc/def   ", "abc/def"),
288        ];
289
290        for (name, input, expect) in cases {
291            assert_eq!(normalize_path(input), expect, "{name}")
292        }
293    }
294
295    #[test]
296    fn test_normalize_root() {
297        let cases = vec![
298            ("dir path", "abc/", "/abc/"),
299            ("empty path", "", "/"),
300            ("root path", "/", "/"),
301            ("root path with extra /", "///", "/"),
302            ("abs dir path", "/abc/def/", "/abc/def/"),
303            ("abs file path with extra /", "///abc/def", "/abc/def/"),
304            ("abs dir path with extra /", "///abc/def/", "/abc/def/"),
305            ("dir path contains ///", "abc///def///", "/abc/def/"),
306        ];
307
308        for (name, input, expect) in cases {
309            assert_eq!(normalize_root(input), expect, "{name}")
310        }
311    }
312
313    #[test]
314    fn test_get_basename() {
315        let cases = vec![
316            ("file abs path", "foo/bar/baz.txt", "baz.txt"),
317            ("file rel path", "bar/baz.txt", "baz.txt"),
318            ("file walk", "foo/bar/baz", "baz"),
319            ("dir rel path", "bar/baz/", "baz/"),
320            ("dir root", "/", "/"),
321            ("dir walk", "foo/bar/baz/", "baz/"),
322        ];
323
324        for (name, input, expect) in cases {
325            let actual = get_basename(input);
326            assert_eq!(actual, expect, "{name}")
327        }
328    }
329
330    #[test]
331    fn test_get_parent() {
332        let cases = vec![
333            ("file abs path", "foo/bar/baz.txt", "foo/bar/"),
334            ("file rel path", "bar/baz.txt", "bar/"),
335            ("file walk", "foo/bar/baz", "foo/bar/"),
336            ("dir rel path", "bar/baz/", "bar/"),
337            ("dir root", "/", "/"),
338            ("dir abs path", "/foo/bar/", "/foo/"),
339            ("dir walk", "foo/bar/baz/", "foo/bar/"),
340        ];
341
342        for (name, input, expect) in cases {
343            let actual = get_parent(input);
344            assert_eq!(actual, expect, "{name}")
345        }
346    }
347
348    #[test]
349    fn test_build_abs_path() {
350        let cases = vec![
351            ("input abs file", "/abc/", "/", "abc/"),
352            ("input dir", "/abc/", "def/", "abc/def/"),
353            ("input file", "/abc/", "def", "abc/def"),
354            ("input abs file with root /", "/", "/", ""),
355            ("input empty with root /", "/", "", ""),
356            ("input dir with root /", "/", "def/", "def/"),
357            ("input file with root /", "/", "def", "def"),
358        ];
359
360        for (name, root, input, expect) in cases {
361            let actual = build_abs_path(root, input);
362            assert_eq!(actual, expect, "{name}")
363        }
364    }
365
366    #[test]
367    fn test_build_rooted_abs_path() {
368        let cases = vec![
369            ("input abs file", "/abc/", "/", "/abc/"),
370            ("input dir", "/abc/", "def/", "/abc/def/"),
371            ("input file", "/abc/", "def", "/abc/def"),
372            ("input abs file with root /", "/", "/", "/"),
373            ("input dir with root /", "/", "def/", "/def/"),
374            ("input file with root /", "/", "def", "/def"),
375        ];
376
377        for (name, root, input, expect) in cases {
378            let actual = build_rooted_abs_path(root, input);
379            assert_eq!(actual, expect, "{name}")
380        }
381    }
382
383    #[test]
384    fn test_build_rel_path() {
385        let cases = vec![
386            ("input abs file", "/abc/", "/abc/def", "def"),
387            ("input dir", "/abc/", "/abc/def/", "def/"),
388            ("input file", "/abc/", "abc/def", "def"),
389            ("input dir with root /", "/", "def/", "def/"),
390            ("input file with root /", "/", "def", "def"),
391        ];
392
393        for (name, root, input, expect) in cases {
394            let actual = build_rel_path(root, input);
395            assert_eq!(actual, expect, "{name}")
396        }
397    }
398
399    #[test]
400    fn test_validate_path() {
401        let cases = vec![
402            ("input file with mode file", "abc", EntryMode::FILE, true),
403            ("input file with mode dir", "abc", EntryMode::DIR, false),
404            ("input dir with mode file", "abc/", EntryMode::FILE, false),
405            ("input dir with mode dir", "abc/", EntryMode::DIR, true),
406            ("root with mode dir", "/", EntryMode::DIR, true),
407            (
408                "input file with mode unknown",
409                "abc",
410                EntryMode::Unknown,
411                false,
412            ),
413            (
414                "input dir with mode unknown",
415                "abc/",
416                EntryMode::Unknown,
417                false,
418            ),
419        ];
420
421        for (name, path, mode, expect) in cases {
422            let actual = validate_path(path, mode);
423            assert_eq!(actual, expect, "{name}")
424        }
425    }
426
427    #[test]
428    fn test_build_tmp_path_of() {
429        let cases = vec![
430            ("a file path", "example.txt", "example.txt."),
431            (
432                "a file path in a directory",
433                "folder/example.txt",
434                "example.txt.",
435            ),
436        ];
437
438        for (name, path, expect_starts_with) in cases {
439            let actual = build_tmp_path_of(path);
440            assert!(
441                actual.starts_with(expect_starts_with),
442                "{name}: got `{actual}`, but expect `{expect_starts_with}`"
443            );
444            assert_eq!(
445                actual.len(),
446                expect_starts_with.len() + 8, // See RANDOM_TMP_PATH_POSTFIX_SIZE
447                "{name}: got `{actual}`, but expect `{expect_starts_with}`"
448            )
449        }
450    }
451}
```
