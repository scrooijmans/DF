# 

opendal/raw/

serde_util.rs

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
19use std::collections::hash_map::IntoIter;
20use std::iter::empty;
21
22use serde::de::Deserializer;
23use serde::de::IntoDeserializer;
24use serde::de::Visitor;
25use serde::de::value::MapDeserializer;
26use serde::de::value::SeqDeserializer;
27use serde::de::{self};
28
29use crate::*;
30
31/// Parse xml serialize error into opendal::Error.
32pub fn new_xml_serialize_error(e: quick_xml::SeError) -> Error {
33    Error::new(ErrorKind::Unexpected, "serialize xml").set_source(e)
34}
35
36/// Parse xml deserialize error into opendal::Error.
37pub fn new_xml_deserialize_error(e: quick_xml::DeError) -> Error {
38    Error::new(ErrorKind::Unexpected, "deserialize xml").set_source(e)
39}
40
41/// Parse json serialize error into opendal::Error.
42pub fn new_json_serialize_error(e: serde_json::Error) -> Error {
43    Error::new(ErrorKind::Unexpected, "serialize json").set_source(e)
44}
45
46/// Parse json deserialize error into opendal::Error.
47pub fn new_json_deserialize_error(e: serde_json::Error) -> Error {
48    Error::new(ErrorKind::Unexpected, "deserialize json").set_source(e)
49}
50
51/// ConfigDeserializer is used to deserialize given configs from `HashMap<String, String>`.
52///
53/// This is only used by our services' config.
54pub struct ConfigDeserializer(MapDeserializer<'static, Pairs, de::value::Error>);
55
56impl ConfigDeserializer {
57    /// Create a new config deserializer.
58    pub fn new(map: HashMap<String, String>) -> Self {
59        let pairs = Pairs(map.into_iter());
60        Self(MapDeserializer::new(pairs))
61    }
62}
63
64impl<'de> Deserializer<'de> for ConfigDeserializer {
65    type Error = de::value::Error;
66
67    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
68    where
69        V: Visitor<'de>,
70    {
71        self.deserialize_map(visitor)
72    }
73
74    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
75    where
76        V: Visitor<'de>,
77    {
78        visitor.visit_map(self.0)
79    }
80
81    serde::forward_to_deserialize_any! {
82        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit seq
83        bytes byte_buf unit_struct tuple_struct
84        identifier tuple ignored_any option newtype_struct enum
85        struct
86    }
87}
88
89/// Pairs is used to implement Iterator to meet the requirement of [`MapDeserializer`].
90struct Pairs(IntoIter<String, String>);
91
92impl Iterator for Pairs {
93    type Item = (String, Pair);
94
95    fn next(&mut self) -> Option<Self::Item> {
96        self.0.next().map(|(k, v)| (k.to_lowercase(), Pair(k, v)))
97    }
98}
99
100/// Pair is used to hold both key and value of a config for better error output.
101struct Pair(String, String);
102
103impl IntoDeserializer<'_, de::value::Error> for Pair {
104    type Deserializer = Self;
105
106    fn into_deserializer(self) -> Self::Deserializer {
107        self
108    }
109}
110
111impl<'de> Deserializer<'de> for Pair {
112    type Error = de::value::Error;
113
114    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
115    where
116        V: Visitor<'de>,
117    {
118        self.1.into_deserializer().deserialize_any(visitor)
119    }
120
121    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
122    where
123        V: Visitor<'de>,
124    {
125        match self.1.to_lowercase().as_str() {
126            "true" | "on" => true.into_deserializer().deserialize_bool(visitor),
127            "false" | "off" => false.into_deserializer().deserialize_bool(visitor),
128            _ => Err(de::Error::custom(format_args!(
129                "parse config '{}' with value '{}' failed for {:?}",
130                self.0, self.1, "invalid bool value"
131            ))),
132        }
133    }
134
135    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
136    where
137        V: Visitor<'de>,
138    {
139        match self.1.parse::<i8>() {
140            Ok(val) => val.into_deserializer().deserialize_i8(visitor),
141            Err(e) => Err(de::Error::custom(format_args!(
142                "parse config '{}' with value '{}' failed for {:?}",
143                self.0, self.1, e
144            ))),
145        }
146    }
147
148    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
149    where
150        V: Visitor<'de>,
151    {
152        match self.1.parse::<i16>() {
153            Ok(val) => val.into_deserializer().deserialize_i16(visitor),
154            Err(e) => Err(de::Error::custom(format_args!(
155                "parse config '{}' with value '{}' failed for {:?}",
156                self.0, self.1, e
157            ))),
158        }
159    }
160
161    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
162    where
163        V: Visitor<'de>,
164    {
165        match self.1.parse::<i32>() {
166            Ok(val) => val.into_deserializer().deserialize_i32(visitor),
167            Err(e) => Err(de::Error::custom(format_args!(
168                "parse config '{}' with value '{}' failed for {:?}",
169                self.0, self.1, e
170            ))),
171        }
172    }
173
174    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
175    where
176        V: Visitor<'de>,
177    {
178        match self.1.parse::<i64>() {
179            Ok(val) => val.into_deserializer().deserialize_i64(visitor),
180            Err(e) => Err(de::Error::custom(format_args!(
181                "parse config '{}' with value '{}' failed for {:?}",
182                self.0, self.1, e
183            ))),
184        }
185    }
186
187    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
188    where
189        V: Visitor<'de>,
190    {
191        match self.1.parse::<u8>() {
192            Ok(val) => val.into_deserializer().deserialize_u8(visitor),
193            Err(e) => Err(de::Error::custom(format_args!(
194                "parse config '{}' with value '{}' failed for {:?}",
195                self.0, self.1, e
196            ))),
197        }
198    }
199
200    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
201    where
202        V: Visitor<'de>,
203    {
204        match self.1.parse::<u16>() {
205            Ok(val) => val.into_deserializer().deserialize_u16(visitor),
206            Err(e) => Err(de::Error::custom(format_args!(
207                "parse config '{}' with value '{}' failed for {:?}",
208                self.0, self.1, e
209            ))),
210        }
211    }
212
213    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
214    where
215        V: Visitor<'de>,
216    {
217        match self.1.parse::<u32>() {
218            Ok(val) => val.into_deserializer().deserialize_u32(visitor),
219            Err(e) => Err(de::Error::custom(format_args!(
220                "parse config '{}' with value '{}' failed for {:?}",
221                self.0, self.1, e
222            ))),
223        }
224    }
225
226    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
227    where
228        V: Visitor<'de>,
229    {
230        match self.1.parse::<u64>() {
231            Ok(val) => val.into_deserializer().deserialize_u64(visitor),
232            Err(e) => Err(de::Error::custom(format_args!(
233                "parse config '{}' with value '{}' failed for {:?}",
234                self.0, self.1, e
235            ))),
236        }
237    }
238
239    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
240    where
241        V: Visitor<'de>,
242    {
243        match self.1.parse::<f32>() {
244            Ok(val) => val.into_deserializer().deserialize_f32(visitor),
245            Err(e) => Err(de::Error::custom(format_args!(
246                "parse config '{}' with value '{}' failed for {:?}",
247                self.0, self.1, e
248            ))),
249        }
250    }
251
252    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
253    where
254        V: Visitor<'de>,
255    {
256        match self.1.parse::<f64>() {
257            Ok(val) => val.into_deserializer().deserialize_f64(visitor),
258            Err(e) => Err(de::Error::custom(format_args!(
259                "parse config '{}' with value '{}' failed for {:?}",
260                self.0, self.1, e
261            ))),
262        }
263    }
264
265    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
266    where
267        V: Visitor<'de>,
268    {
269        if self.1.is_empty() {
270            visitor.visit_none()
271        } else {
272            visitor.visit_some(self)
273        }
274    }
275
276    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
277    where
278        V: Visitor<'de>,
279    {
280        // Return empty instead of `[""]`.
281        if self.1.is_empty() {
282            SeqDeserializer::new(empty::<Pair>())
283                .deserialize_seq(visitor)
284                .map_err(|e| {
285                    de::Error::custom(format_args!(
286                        "parse config '{}' with value '{}' failed for {:?}",
287                        self.0, self.1, e
288                    ))
289                })
290        } else {
291            let values = self
292                .1
293                .split(',')
294                .map(|v| Pair(self.0.clone(), v.trim().to_owned()));
295            SeqDeserializer::new(values)
296                .deserialize_seq(visitor)
297                .map_err(|e| {
298                    de::Error::custom(format_args!(
299                        "parse config '{}' with value '{}' failed for {:?}",
300                        self.0, self.1, e
301                    ))
302                })
303        }
304    }
305
306    serde::forward_to_deserialize_any! {
307        char str string unit newtype_struct enum
308        bytes byte_buf map unit_struct tuple_struct
309        identifier tuple ignored_any
310        struct
311    }
312}
313
314#[cfg(test)]
315mod tests {
316    use serde::Deserialize;
317
318    use super::*;
319
320    #[derive(Debug, Default, Deserialize, Eq, PartialEq)]
321    #[serde(default)]
322    #[non_exhaustive]
323    pub struct TestConfig {
324        bool_value: bool,
325        bool_option_value_none: Option<bool>,
326        bool_option_value_some: Option<bool>,
327        bool_value_with_on: bool,
328        bool_value_with_off: bool,
329
330        string_value: String,
331        string_option_value_none: Option<String>,
332        string_option_value_some: Option<String>,
333
334        u8_value: u8,
335        u16_value: u16,
336        u32_value: u32,
337        u64_value: u64,
338        i8_value: i8,
339        i16_value: i16,
340        i32_value: i32,
341        i64_value: i64,
342
343        vec_value: Vec<String>,
344        vec_value_two: Vec<String>,
345        vec_none: Option<Vec<String>>,
346        vec_empty: Vec<String>,
347    }
348
349    #[test]
350    fn test_config_deserializer() {
351        let mut map = HashMap::new();
352        map.insert("bool_value", "true");
353        map.insert("bool_option_value_none", "");
354        map.insert("bool_option_value_some", "false");
355        map.insert("bool_value_with_on", "on");
356        map.insert("bool_value_with_off", "off");
357        map.insert("string_value", "hello");
358        map.insert("string_option_value_none", "");
359        map.insert("string_option_value_some", "hello");
360        map.insert("u8_value", "8");
361        map.insert("u16_value", "16");
362        map.insert("u32_value", "32");
363        map.insert("u64_value", "64");
364        map.insert("i8_value", "-8");
365        map.insert("i16_value", "16");
366        map.insert("i32_value", "-32");
367        map.insert("i64_value", "64");
368        map.insert("vec_value", "hello");
369        map.insert("vec_value_two", "hello,world");
370        map.insert("vec_none", "");
371        map.insert("vec_empty", "");
372        let map = map
373            .into_iter()
374            .map(|(k, v)| (k.to_string(), v.to_string()))
375            .collect();
376
377        let output = TestConfig::deserialize(ConfigDeserializer::new(map)).unwrap();
378        assert_eq!(
379            output,
380            TestConfig {
381                bool_value: true,
382                bool_option_value_none: None,
383                bool_option_value_some: Some(false),
384                bool_value_with_on: true,
385                bool_value_with_off: false,
386                string_value: "hello".to_string(),
387                string_option_value_none: None,
388                string_option_value_some: Some("hello".to_string()),
389                u8_value: 8,
390                u16_value: 16,
391                u32_value: 32,
392                u64_value: 64,
393                i8_value: -8,
394                i16_value: 16,
395                i32_value: -32,
396                i64_value: 64,
397                vec_value: vec!["hello".to_string()],
398                vec_value_two: vec!["hello".to_string(), "world".to_string()],
399                vec_none: None,
400                vec_empty: vec![],
401            }
402        );
403    }
404
405    #[test]
406    fn test_part_config_deserializer() {
407        let mut map = HashMap::new();
408        map.insert("bool_value", "true");
409        let map = map
410            .into_iter()
411            .map(|(k, v)| (k.to_string(), v.to_string()))
412            .collect();
413
414        let output = TestConfig::deserialize(ConfigDeserializer::new(map)).unwrap();
415        assert_eq!(
416            output,
417            TestConfig {
418                bool_value: true,
419                ..TestConfig::default()
420            }
421        );
422    }
423}
```
