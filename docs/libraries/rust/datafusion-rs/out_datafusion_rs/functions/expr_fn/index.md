# Module expr_fn Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/lib.rs.html#148" class="src">Source</a>

Expand description

Fluent-style API for creating `Expr`s

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.abs.html" class="fn" title="fn datafusion::functions::expr_fn::abs">abs</a>  
returns the absolute value of a given number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.acos.html" class="fn" title="fn datafusion::functions::expr_fn::acos">acos</a>  
returns the arc cosine or inverse cosine of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.acosh.html" class="fn" title="fn datafusion::functions::expr_fn::acosh">acosh</a>  
returns inverse hyperbolic cosine

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.arrow_cast.html" class="fn" title="fn datafusion::functions::expr_fn::arrow_cast">arrow_cast</a>  
Returns value2 if value1 is NULL; otherwise it returns value1

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.arrow_typeof.html" class="fn" title="fn datafusion::functions::expr_fn::arrow_typeof">arrow_typeof</a>  
Returns the Arrow type of the input expression.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.ascii.html" class="fn" title="fn datafusion::functions::expr_fn::ascii">ascii</a>  
Returns the numeric code of the first character of the argument.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.asin.html" class="fn" title="fn datafusion::functions::expr_fn::asin">asin</a>  
returns the arc sine or inverse sine of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.asinh.html" class="fn" title="fn datafusion::functions::expr_fn::asinh">asinh</a>  
returns inverse hyperbolic sine

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.atan.html" class="fn" title="fn datafusion::functions::expr_fn::atan">atan</a>  
returns inverse tangent

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.atan2.html" class="fn" title="fn datafusion::functions::expr_fn::atan2">atan2</a>  
returns inverse tangent of a division given in the argument

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.atanh.html" class="fn" title="fn datafusion::functions::expr_fn::atanh">atanh</a>  
returns inverse hyperbolic tangent

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.bit_length.html" class="fn" title="fn datafusion::functions::expr_fn::bit_length">bit_length</a>  
Returns the number of bits in the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.btrim.html" class="fn" title="fn datafusion::functions::expr_fn::btrim">btrim</a>  
Removes all characters, spaces by default, from both sides of a string

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.cbrt.html" class="fn" title="fn datafusion::functions::expr_fn::cbrt">cbrt</a>  
cube root of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.ceil.html" class="fn" title="fn datafusion::functions::expr_fn::ceil">ceil</a>  
nearest integer greater than or equal to argument

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.char_length.html" class="fn" title="fn datafusion::functions::expr_fn::char_length">char_length</a>  
the number of characters in the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.character_length.html" class="fn" title="fn datafusion::functions::expr_fn::character_length">character_length</a>  
the number of characters in the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.chr.html" class="fn" title="fn datafusion::functions::expr_fn::chr">chr</a>  
Converts the Unicode code point to a UTF8 character

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.coalesce.html" class="fn" title="fn datafusion::functions::expr_fn::coalesce">coalesce</a>  
Returns `coalesce(args...)`, which evaluates to the value of the first expr which is not NULL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.concat.html" class="fn" title="fn datafusion::functions::expr_fn::concat">concat</a>  
Concatenates the text representations of all the arguments. NULL arguments are ignored

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.concat_ws.html" class="fn" title="fn datafusion::functions::expr_fn::concat_ws">concat_ws</a>  
Concatenates all but the first argument, with separators. The first argument is used as the separator string, and should not be NULL. Other NULL arguments are ignored.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.contains.html" class="fn" title="fn datafusion::functions::expr_fn::contains">contains</a>  
Return true if `search_string` is found within `string`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.cos.html" class="fn" title="fn datafusion::functions::expr_fn::cos">cos</a>  
cosine

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.cosh.html" class="fn" title="fn datafusion::functions::expr_fn::cosh">cosh</a>  
hyperbolic cosine

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.cot.html" class="fn" title="fn datafusion::functions::expr_fn::cot">cot</a>  
cotangent of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.current_date.html" class="fn" title="fn datafusion::functions::expr_fn::current_date">current_date</a>  
returns current UTC date as a Date32 value

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.current_time.html" class="fn" title="fn datafusion::functions::expr_fn::current_time">current_time</a>  
returns current UTC time as a Time64 value

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.date_bin.html" class="fn" title="fn datafusion::functions::expr_fn::date_bin">date_bin</a>  
coerces an arbitrary timestamp to the start of the nearest specified interval

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.date_part.html" class="fn" title="fn datafusion::functions::expr_fn::date_part">date_part</a>  
extracts a subfield from the date

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.date_trunc.html" class="fn" title="fn datafusion::functions::expr_fn::date_trunc">date_trunc</a>  
truncates the date to a specified level of precision

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.decode.html" class="fn" title="fn datafusion::functions::expr_fn::decode">decode</a>  
decode the `input`, using the `encoding`. encoding can be base64 or hex

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.degrees.html" class="fn" title="fn datafusion::functions::expr_fn::degrees">degrees</a>  
converts radians to degrees

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.digest.html" class="fn" title="fn datafusion::functions::expr_fn::digest">digest</a>  
Computes the binary hash of an expression using the specified algorithm.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.encode.html" class="fn" title="fn datafusion::functions::expr_fn::encode">encode</a>  
encode the `input`, using the `encoding`. encoding can be base64 or hex

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.ends_with.html" class="fn" title="fn datafusion::functions::expr_fn::ends_with">ends_with</a>  
Returns true if the `string` ends with the `suffix`, false otherwise.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.exp.html" class="fn" title="fn datafusion::functions::expr_fn::exp">exp</a>  
exponential

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.factorial.html" class="fn" title="fn datafusion::functions::expr_fn::factorial">factorial</a>  
factorial

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.find_in_set.html" class="fn" title="fn datafusion::functions::expr_fn::find_in_set">find_in_set</a>  
Returns a value in the range of 1 to N if the string `str` is in the string list `strlist` consisting of N substrings

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.floor.html" class="fn" title="fn datafusion::functions::expr_fn::floor">floor</a>  
nearest integer less than or equal to argument

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.from_unixtime.html" class="fn" title="fn datafusion::functions::expr_fn::from_unixtime">from_unixtime</a>  
converts an integer to RFC3339 timestamp format string

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.gcd.html" class="fn" title="fn datafusion::functions::expr_fn::gcd">gcd</a>  
greatest common divisor

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.get_field.html" class="fn" title="fn datafusion::functions::expr_fn::get_field">get_field</a>  
Returns the value of the field with the given name from the struct

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.greatest.html" class="fn" title="fn datafusion::functions::expr_fn::greatest">greatest</a>  
Returns `greatest(args...)`, which evaluates to the greatest value in the list of expressions or NULL if all the expressions are NULL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.initcap.html" class="fn" title="fn datafusion::functions::expr_fn::initcap">initcap</a>  
converts the first letter of each word in `string` in uppercase and the remaining characters in lowercase

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.instr.html" class="fn" title="fn datafusion::functions::expr_fn::instr">instr</a>  
finds the position from where the `substring` matches the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.isnan.html" class="fn" title="fn datafusion::functions::expr_fn::isnan">isnan</a>  
returns true if a given number is +NaN or -NaN otherwise returns false

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.iszero.html" class="fn" title="fn datafusion::functions::expr_fn::iszero">iszero</a>  
returns true if a given number is +0.0 or -0.0 otherwise returns false

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.lcm.html" class="fn" title="fn datafusion::functions::expr_fn::lcm">lcm</a>  
least common multiple

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.least.html" class="fn" title="fn datafusion::functions::expr_fn::least">least</a>  
Returns `least(args...)`, which evaluates to the smallest value in the list of expressions or NULL if all the expressions are NULL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.left.html" class="fn" title="fn datafusion::functions::expr_fn::left">left</a>  
returns the first `n` characters in the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.length.html" class="fn" title="fn datafusion::functions::expr_fn::length">length</a>  
the number of characters in the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.levenshtein.html" class="fn" title="fn datafusion::functions::expr_fn::levenshtein">levenshtein</a>  
Returns the Levenshtein distance between the two given strings

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.ln.html" class="fn" title="fn datafusion::functions::expr_fn::ln">ln</a>  
natural logarithm (base e) of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.log.html" class="fn" title="fn datafusion::functions::expr_fn::log">log</a>  
logarithm of a number for a particular `base`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.log2.html" class="fn" title="fn datafusion::functions::expr_fn::log2">log2</a>  
base 2 logarithm of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.log10.html" class="fn" title="fn datafusion::functions::expr_fn::log10">log10</a>  
base 10 logarithm of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.lower.html" class="fn" title="fn datafusion::functions::expr_fn::lower">lower</a>  
Converts a string to lowercase.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.lpad.html" class="fn" title="fn datafusion::functions::expr_fn::lpad">lpad</a>  
fill up a string to the length by prepending the characters

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.ltrim.html" class="fn" title="fn datafusion::functions::expr_fn::ltrim">ltrim</a>  
Removes all characters, spaces by default, from the beginning of a string

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.make_date.html" class="fn" title="fn datafusion::functions::expr_fn::make_date">make_date</a>  
make a date from year, month and day component parts

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.md5.html" class="fn" title="fn datafusion::functions::expr_fn::md5">md5</a>  
Computes an MD5 128-bit checksum for a string expression.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.named_struct.html" class="fn" title="fn datafusion::functions::expr_fn::named_struct">named_struct</a>  
Returns a struct with the given names and arguments pairs

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.nanvl.html" class="fn" title="fn datafusion::functions::expr_fn::nanvl">nanvl</a>  
returns x if x is not NaN otherwise returns y

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.now.html" class="fn" title="fn datafusion::functions::expr_fn::now">now</a>  
returns the current timestamp in nanoseconds, using the same value for all instances of now() in same statement

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.nullif.html" class="fn" title="fn datafusion::functions::expr_fn::nullif">nullif</a>  
Returns NULL if value1 equals value2; otherwise it returns value1. This can be used to perform the inverse operation of the COALESCE expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.nvl.html" class="fn" title="fn datafusion::functions::expr_fn::nvl">nvl</a>  
Returns value2 if value1 is NULL; otherwise it returns value1

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.nvl2.html" class="fn" title="fn datafusion::functions::expr_fn::nvl2">nvl2</a>  
Returns value2 if value1 is not NULL; otherwise, it returns value3.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.octet_length.html" class="fn" title="fn datafusion::functions::expr_fn::octet_length">octet_length</a>  
returns the number of bytes of a string

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.overlay.html" class="fn" title="fn datafusion::functions::expr_fn::overlay">overlay</a>  
replace the substring of string that starts at the start’th character and extends for count characters with new substring

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.pi.html" class="fn" title="fn datafusion::functions::expr_fn::pi">pi</a>  
Returns an approximate value of π

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.position.html" class="fn" title="fn datafusion::functions::expr_fn::position">position</a>  
finds the position from where the `substring` matches the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.power.html" class="fn" title="fn datafusion::functions::expr_fn::power">power</a>  
`base` raised to the power of `exponent`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.radians.html" class="fn" title="fn datafusion::functions::expr_fn::radians">radians</a>  
converts degrees to radians

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.random.html" class="fn" title="fn datafusion::functions::expr_fn::random">random</a>  
Returns a random value in the range 0.0 \<= x \< 1.0

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.regexp_count.html" class="fn" title="fn datafusion::functions::expr_fn::regexp_count">regexp_count</a>  
Returns the number of consecutive occurrences of a regular expression in a string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.regexp_instr.html" class="fn" title="fn datafusion::functions::expr_fn::regexp_instr">regexp_instr</a>  
Returns index of regular expression matches in a string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.regexp_like.html" class="fn" title="fn datafusion::functions::expr_fn::regexp_like">regexp_like</a>  
Returns true if a regex has at least one match in a string, false otherwise.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.regexp_match.html" class="fn" title="fn datafusion::functions::expr_fn::regexp_match">regexp_match</a>  
Returns a list of regular expression matches in a string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.regexp_replace.html" class="fn" title="fn datafusion::functions::expr_fn::regexp_replace">regexp_replace</a>  
Replaces substrings in a string that match.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.repeat.html" class="fn" title="fn datafusion::functions::expr_fn::repeat">repeat</a>  
Repeats the `string` to `n` times

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.replace.html" class="fn" title="fn datafusion::functions::expr_fn::replace">replace</a>  
Replaces all occurrences of `from` with `to` in the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.reverse.html" class="fn" title="fn datafusion::functions::expr_fn::reverse">reverse</a>  
reverses the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.right.html" class="fn" title="fn datafusion::functions::expr_fn::right">right</a>  
returns the last `n` characters in the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.round.html" class="fn" title="fn datafusion::functions::expr_fn::round">round</a>  
round to nearest integer

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.rpad.html" class="fn" title="fn datafusion::functions::expr_fn::rpad">rpad</a>  
fill up a string to the length by appending the characters

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.rtrim.html" class="fn" title="fn datafusion::functions::expr_fn::rtrim">rtrim</a>  
Removes all characters, spaces by default, from the end of a string

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.sha224.html" class="fn" title="fn datafusion::functions::expr_fn::sha224">sha224</a>  
Computes the SHA-224 hash of a binary string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.sha256.html" class="fn" title="fn datafusion::functions::expr_fn::sha256">sha256</a>  
Computes the SHA-256 hash of a binary string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.sha384.html" class="fn" title="fn datafusion::functions::expr_fn::sha384">sha384</a>  
Computes the SHA-384 hash of a binary string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.sha512.html" class="fn" title="fn datafusion::functions::expr_fn::sha512">sha512</a>  
Computes the SHA-512 hash of a binary string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.signum.html" class="fn" title="fn datafusion::functions::expr_fn::signum">signum</a>  
sign of the argument (-1, 0, +1)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.sin.html" class="fn" title="fn datafusion::functions::expr_fn::sin">sin</a>  
sine

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.sinh.html" class="fn" title="fn datafusion::functions::expr_fn::sinh">sinh</a>  
hyperbolic sine

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.split_part.html" class="fn" title="fn datafusion::functions::expr_fn::split_part">split_part</a>  
Splits a string based on a delimiter and picks out the desired field based on the index.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.sqrt.html" class="fn" title="fn datafusion::functions::expr_fn::sqrt">sqrt</a>  
square root of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.starts_with.html" class="fn" title="fn datafusion::functions::expr_fn::starts_with">starts_with</a>  
Returns true if string starts with prefix.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.strpos.html" class="fn" title="fn datafusion::functions::expr_fn::strpos">strpos</a>  
finds the position from where the `substring` matches the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.struct.html" class="fn" title="fn datafusion::functions::expr_fn::struct">struct</a>  
Returns a struct with the given arguments

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.substr.html" class="fn" title="fn datafusion::functions::expr_fn::substr">substr</a>  
substring from the `position` to the end

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.substr_index.html" class="fn" title="fn datafusion::functions::expr_fn::substr_index">substr_index</a>  
Returns the substring from str before count occurrences of the delimiter

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.substring.html" class="fn" title="fn datafusion::functions::expr_fn::substring">substring</a>  
substring from the `position` with `length` characters

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.tan.html" class="fn" title="fn datafusion::functions::expr_fn::tan">tan</a>  
returns the tangent of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.tanh.html" class="fn" title="fn datafusion::functions::expr_fn::tanh">tanh</a>  
returns the hyperbolic tangent of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.to_char.html" class="fn" title="fn datafusion::functions::expr_fn::to_char">to_char</a>  
Returns a string representation of a date, time, timestamp or duration based on a Chrono pattern.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.to_date.html" class="fn" title="fn datafusion::functions::expr_fn::to_date">to_date</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.to_hex.html" class="fn" title="fn datafusion::functions::expr_fn::to_hex">to_hex</a>  
Converts an integer to a hexadecimal string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.to_local_time.html" class="fn" title="fn datafusion::functions::expr_fn::to_local_time">to_local_time</a>  
converts a timezone-aware timestamp to local time (with no offset or timezone information), i.e. strips off the timezone from the timestamp

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.to_timestamp.html" class="fn" title="fn datafusion::functions::expr_fn::to_timestamp">to_timestamp</a>  
converts a string and optional formats to a `Timestamp(Nanoseconds, None)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.to_timestamp_micros.html" class="fn" title="fn datafusion::functions::expr_fn::to_timestamp_micros">to_timestamp_micros</a>  
converts a string and optional formats to a `Timestamp(Microseconds, None)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.to_timestamp_millis.html" class="fn" title="fn datafusion::functions::expr_fn::to_timestamp_millis">to_timestamp_millis</a>  
converts a string and optional formats to a `Timestamp(Milliseconds, None)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.to_timestamp_nanos.html" class="fn" title="fn datafusion::functions::expr_fn::to_timestamp_nanos">to_timestamp_nanos</a>  
converts a string and optional formats to a `Timestamp(Nanoseconds, None)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.to_timestamp_seconds.html" class="fn" title="fn datafusion::functions::expr_fn::to_timestamp_seconds">to_timestamp_seconds</a>  
converts a string and optional formats to a `Timestamp(Seconds, None)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.to_unixtime.html" class="fn" title="fn datafusion::functions::expr_fn::to_unixtime">to_unixtime</a>  
converts a string and optional formats to a Unixtime

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.translate.html" class="fn" title="fn datafusion::functions::expr_fn::translate">translate</a>  
replaces the characters in `from` with the counterpart in `to`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.trim.html" class="fn" title="fn datafusion::functions::expr_fn::trim">trim</a>  
Removes all characters, spaces by default, from both sides of a string

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.trunc.html" class="fn" title="fn datafusion::functions::expr_fn::trunc">trunc</a>  
truncate toward zero, with optional precision

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.union_extract.html" class="fn" title="fn datafusion::functions::expr_fn::union_extract">union_extract</a>  
Returns the value of the field with the given name from the union when it’s selected, or NULL otherwise

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.union_tag.html" class="fn" title="fn datafusion::functions::expr_fn::union_tag">union_tag</a>  
Returns the name of the currently selected field in the union

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.upper.html" class="fn" title="fn datafusion::functions::expr_fn::upper">upper</a>  
Converts a string to uppercase.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/fn.uuid.html" class="fn" title="fn datafusion::functions::expr_fn::uuid">uuid</a>  
returns uuid v4 as a string value
